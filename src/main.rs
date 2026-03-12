mod api;
mod cache;
mod config;
mod stats;
mod ui;

use anyhow::{Context, Result};
use crossterm::{
   event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
   execute,
   terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

use api::{endpoints::ApiEndpoints, ApiClient};
use cache::Cache;
use config::Config;
use ui::{App, View};

fn main() -> Result<()> {
   // Load configuration
   let config = Config::load().context("Failed to load configuration")?;

   // Initialize API client and cache
   let client = ApiClient::new(config.api_key.clone())?;
   let cache = Cache::new(&config.event_key)?;
   let endpoints = ApiEndpoints::new(&client, &cache, config.event_key.clone());

   // Load initial data
   let mut app = load_app_data(&endpoints, config.test_mode)?;

   // Setup terminal
   enable_raw_mode()?;
   let mut stdout = io::stdout();
   execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
   let backend = CrosstermBackend::new(stdout);
   let mut terminal = Terminal::new(backend)?;

   // Run app
   let res = run_app(&mut terminal, &mut app, &endpoints, config.test_mode);

   // Restore terminal
   disable_raw_mode()?;
   execute!(
      terminal.backend_mut(),
      LeaveAlternateScreen,
      DisableMouseCapture
   )?;
   terminal.show_cursor()?;

   if let Err(err) = res {
      println!("Error: {:?}", err);
   }

   Ok(())
}

fn load_app_data(endpoints: &ApiEndpoints, test_mode: bool) -> Result<App> {
   // Show loading message (only visible before TUI starts or during refresh)
   println!("Loading data from The Blue Alliance...");

   // Fetch all data
   let event = endpoints
      .get_event()
      .context("Failed to fetch event data")?;
   let teams = endpoints
      .get_teams()
      .context("Failed to fetch teams data")?;
   let rankings = endpoints
      .get_rankings()
      .context("Failed to fetch rankings data")?;
   let matches = endpoints
      .get_matches()
      .context("Failed to fetch matches data")?;
   let oprs = endpoints.get_oprs().context("Failed to fetch OPR data")?;

   println!("Data loaded successfully!");

   // Check if we have any teams to display
   if teams.is_empty() {
      anyhow::bail!("No teams registered for this event yet.");
   }

   if rankings.is_empty() {
      println!("Note: No rankings available yet. Teams will be sorted by number.");
   }

   // Create app
   Ok(App::new(event, teams, rankings, matches, oprs, test_mode))
}

fn run_app<B: ratatui::backend::Backend>(
   terminal: &mut Terminal<B>,
   app: &mut App,
   endpoints: &ApiEndpoints,
   test_mode: bool,
) -> Result<()> {
   loop {
      terminal.draw(|f| {
         let area = f.area();
         match &app.current_view {
            View::TeamList {
               selected_index,
               search_query,
               searching,
            } => {
               ui::team_list::render(f, app, *selected_index, search_query, *searching, area);
            }
            View::TeamDetail {
               team_key,
               selected_match_index,
            } => {
               ui::team_detail::render(f, app, team_key, *selected_match_index, area);
            }
            View::MatchDetail {
               match_key,
               selected_team_index,
            } => {
               ui::match_detail::render(f, app, match_key, *selected_team_index, area);
            }
         }
      })?;

      if let Event::Key(key) = event::read()? {
         // Handle global quit key
         if let KeyCode::Char('q') = key.code {
            return Ok(());
         }

         // Handle global refresh key
         if let KeyCode::Char('r') = key.code {
            // Reload data
            match load_app_data(endpoints, test_mode) {
               Ok(new_app) => {
                  *app = new_app;
                  // Show brief confirmation message
                  terminal.draw(|f| {
                     use ratatui::layout::Alignment;
                     use ratatui::style::{Color, Style};
                     use ratatui::widgets::{Block, Borders, Paragraph};

                     let area = f.area();
                     let popup_area = centered_rect(50, 20, area);

                     let text = vec![ratatui::text::Line::from("Data refreshed successfully!")];

                     let popup = Paragraph::new(text)
                        .block(Block::default().borders(Borders::ALL).title(" Refresh "))
                        .style(Style::default().fg(Color::Green))
                        .alignment(Alignment::Center);

                     f.render_widget(popup, popup_area);
                  })?;

                  // Brief pause to show the message
                  std::thread::sleep(std::time::Duration::from_millis(500));
               }
               Err(e) => {
                  // Show error message
                  terminal.draw(|f| {
                     use ratatui::layout::Alignment;
                     use ratatui::style::{Color, Style};
                     use ratatui::widgets::{Block, Borders, Paragraph};

                     let area = f.area();
                     let popup_area = centered_rect(60, 30, area);

                     let text = vec![
                        ratatui::text::Line::from("Failed to refresh data:"),
                        ratatui::text::Line::from(""),
                        ratatui::text::Line::from(format!("{}", e)),
                     ];

                     let popup = Paragraph::new(text)
                        .block(Block::default().borders(Borders::ALL).title(" Error "))
                        .style(Style::default().fg(Color::Red))
                        .alignment(Alignment::Center);

                     f.render_widget(popup, popup_area);
                  })?;

                  // Wait for user to dismiss
                  std::thread::sleep(std::time::Duration::from_secs(2));
               }
            }
            continue;
         }

         match &app.current_view.clone() {
            View::TeamList {
               selected_index,
               search_query,
               searching,
            } => {
               handle_team_list_input(app, key.code, *selected_index, search_query, *searching)?;
            }
            View::TeamDetail {
               team_key,
               selected_match_index,
            } => {
               handle_team_detail_input(app, key.code, team_key, *selected_match_index)?;
            }
            View::MatchDetail {
               match_key,
               selected_team_index,
            } => {
               handle_match_detail_input(app, key.code, match_key, *selected_team_index)?;
            }
         }
      }
   }
}

fn handle_team_list_input(
   app: &mut App,
   key: KeyCode,
   selected_index: usize,
   search_query: &str,
   searching: bool,
) -> Result<()> {
   if searching {
      // Handle search mode input
      match key {
         KeyCode::Esc => {
            // Cancel search
            app.current_view = View::TeamList {
               selected_index,
               search_query: String::new(),
               searching: false,
            };
         }
         KeyCode::Enter => {
            // Execute search
            if !search_query.is_empty() {
               if let Ok(team_number) = search_query.parse::<i32>() {
                  // Find team by number
                  if let Some(index) = app
                     .data
                     .team_stats
                     .iter()
                     .position(|s| s.team_number == team_number)
                  {
                     app.current_view = View::TeamList {
                        selected_index: index,
                        search_query: String::new(),
                        searching: false,
                     };
                  } else {
                     // Team not found, exit search mode
                     app.current_view = View::TeamList {
                        selected_index,
                        search_query: String::new(),
                        searching: false,
                     };
                  }
               }
            } else {
               // Empty search, just exit search mode
               app.current_view = View::TeamList {
                  selected_index,
                  search_query: String::new(),
                  searching: false,
               };
            }
         }
         KeyCode::Backspace => {
            // Remove last character
            let mut new_query = search_query.to_string();
            new_query.pop();
            app.current_view = View::TeamList {
               selected_index,
               search_query: new_query,
               searching: true,
            };
         }
         KeyCode::Char(c) if c.is_ascii_digit() => {
            // Add digit to search query
            let mut new_query = search_query.to_string();
            new_query.push(c);
            app.current_view = View::TeamList {
               selected_index,
               search_query: new_query,
               searching: true,
            };
         }
         _ => {}
      }
   } else {
      // Normal navigation mode
      match key {
         KeyCode::Char('/') => {
            // Enter search mode
            app.current_view = View::TeamList {
               selected_index,
               search_query: String::new(),
               searching: true,
            };
         }
         KeyCode::Down | KeyCode::Char('j') => {
            if !app.data.team_stats.is_empty() {
               let new_index = (selected_index + 1).min(app.data.team_stats.len() - 1);
               app.current_view = View::TeamList {
                  selected_index: new_index,
                  search_query: String::new(),
                  searching: false,
               };
            }
         }
         KeyCode::Up | KeyCode::Char('k') => {
            let new_index = selected_index.saturating_sub(1);
            app.current_view = View::TeamList {
               selected_index: new_index,
               search_query: String::new(),
               searching: false,
            };
         }
         KeyCode::Enter => {
            if let Some(stats) = app.data.team_stats.get(selected_index) {
               app.navigate_to(View::TeamDetail {
                  team_key: stats.team_key.clone(),
                  selected_match_index: 0,
               });
            }
         }
         _ => {}
      }
   }
   Ok(())
}

fn handle_team_detail_input(
   app: &mut App,
   key: KeyCode,
   team_key: &str,
   selected_match_index: usize,
) -> Result<()> {
   let matches = app.get_team_matches(team_key);
   let max_index = matches.len().saturating_sub(1);

   match key {
      KeyCode::Esc | KeyCode::Backspace => {
         app.navigate_back();
      }
      KeyCode::Down | KeyCode::Char('j') => {
         let new_index = (selected_match_index + 1).min(max_index);
         app.current_view = View::TeamDetail {
            team_key: team_key.to_string(),
            selected_match_index: new_index,
         };
      }
      KeyCode::Up | KeyCode::Char('k') => {
         let new_index = selected_match_index.saturating_sub(1);
         app.current_view = View::TeamDetail {
            team_key: team_key.to_string(),
            selected_match_index: new_index,
         };
      }
      KeyCode::Enter => {
         if let Some(match_data) = matches.get(selected_match_index) {
            app.navigate_to(View::MatchDetail {
               match_key: match_data.key.clone(),
               selected_team_index: 0,
            });
         }
      }
      _ => {}
   }
   Ok(())
}

fn handle_match_detail_input(
   app: &mut App,
   key: KeyCode,
   match_key: &str,
   selected_team_index: usize,
) -> Result<()> {
   match key {
      KeyCode::Esc | KeyCode::Backspace => {
         app.navigate_back();
      }
      KeyCode::Down | KeyCode::Char('j') => {
         let new_index = (selected_team_index + 1).min(5); // 6 teams total (0-5)
         app.current_view = View::MatchDetail {
            match_key: match_key.to_string(),
            selected_team_index: new_index,
         };
      }
      KeyCode::Up | KeyCode::Char('k') => {
         let new_index = selected_team_index.saturating_sub(1);
         app.current_view = View::MatchDetail {
            match_key: match_key.to_string(),
            selected_team_index: new_index,
         };
      }
      KeyCode::Enter => {
         // Get the selected team and navigate to their detail page
         if let Some(match_data) = app.get_match(match_key) {
            let all_teams: Vec<String> = match_data
               .alliances
               .blue
               .team_keys
               .iter()
               .chain(match_data.alliances.red.team_keys.iter())
               .cloned()
               .collect();

            if let Some(team_key) = all_teams.get(selected_team_index) {
               app.navigate_to(View::TeamDetail {
                  team_key: team_key.clone(),
                  selected_match_index: 0,
               });
            }
         }
      }
      _ => {}
   }
   Ok(())
}

// Helper function to create a centered rectangle for popup
fn centered_rect(
   percent_x: u16,
   percent_y: u16,
   r: ratatui::layout::Rect,
) -> ratatui::layout::Rect {
   use ratatui::layout::{Constraint, Direction, Layout};

   let popup_layout = Layout::default()
      .direction(Direction::Vertical)
      .constraints([
         Constraint::Percentage((100 - percent_y) / 2),
         Constraint::Percentage(percent_y),
         Constraint::Percentage((100 - percent_y) / 2),
      ])
      .split(r);

   Layout::default()
      .direction(Direction::Horizontal)
      .constraints([
         Constraint::Percentage((100 - percent_x) / 2),
         Constraint::Percentage(percent_x),
         Constraint::Percentage((100 - percent_x) / 2),
      ])
      .split(popup_layout[1])[1]
}
