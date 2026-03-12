use crate::api::models::{Match, MatchStatus};
use crate::ui::app::App;
use chrono::{Local, TimeZone};
use ratatui::{
   layout::{Constraint, Layout, Rect},
   style::{Color, Modifier, Style},
   text::{Line, Span},
   widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
   Frame,
};

pub fn render(f: &mut Frame, app: &App, match_key: &str, selected_team_index: usize, area: Rect) {
   if let Some(match_data) = app.get_match(match_key) {
      let chunks = Layout::default()
         .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Alliances
         ])
         .split(area);

      // Header with match info
      render_header(f, match_data, chunks[0]);

      // Alliances
      render_alliances(f, app, match_data, selected_team_index, chunks[1]);
   }
}

fn render_header(f: &mut Frame, match_data: &Match, area: Rect) {
   let match_name = match_data.display_name();

   let time_info = match match_data.status() {
      MatchStatus::Completed { .. } => {
         if let Some(timestamp) = match_data.actual_time {
            let dt = Local.timestamp_opt(timestamp, 0).unwrap();
            format!("Played: {}", dt.format("%a, %b %-d, %Y at %-I:%M %p"))
         } else {
            "Completed".to_string()
         }
      }
      MatchStatus::Scheduled { time } => {
         if let Some(timestamp) = time {
            let dt = Local.timestamp_opt(timestamp, 0).unwrap();
            format!("Scheduled: {}", dt.format("%a, %b %-d, %Y at %-I:%M %p"))
         } else {
            "Not scheduled yet".to_string()
         }
      }
   };

   let header_text = vec![
      Line::from(vec![Span::styled(
         match_name,
         Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
      )]),
      Line::from(vec![Span::styled(
         time_info,
         Style::default().fg(Color::Yellow),
      )]),
   ];

   let header = Block::default()
      .borders(Borders::ALL)
      .title(" Match Details ")
      .style(Style::default());

   let paragraph = Paragraph::new(header_text).block(header);
   f.render_widget(paragraph, area);
}

fn render_alliances(
   f: &mut Frame,
   app: &App,
   match_data: &Match,
   selected_team_index: usize,
   area: Rect,
) {
   match match_data.status() {
      MatchStatus::Completed {
         blue_score,
         red_score,
         winner,
      } => {
         render_completed_match(
            f,
            app,
            match_data,
            selected_team_index,
            area,
            blue_score,
            red_score,
            winner,
         );
      }
      MatchStatus::Scheduled { .. } => {
         render_scheduled_match(f, app, match_data, selected_team_index, area);
      }
   }
}

fn render_completed_match(
   f: &mut Frame,
   app: &App,
   match_data: &Match,
   selected_team_index: usize,
   area: Rect,
   blue_score: i32,
   red_score: i32,
   winner: Option<String>,
) {
   let chunks = Layout::default()
      .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
      .split(area);

   // Blue Alliance
   let blue_teams = &match_data.alliances.blue.team_keys;
   let blue_won = winner.as_deref() == Some("blue");

   let blue_title = if blue_won {
      format!(" 🔵 Blue Alliance ({}) - WON ", blue_score)
   } else {
      format!(" 🔵 Blue Alliance ({}) ", blue_score)
   };

   render_alliance_teams(
      f,
      app,
      blue_teams,
      selected_team_index,
      0,
      chunks[0],
      &blue_title,
      Color::Blue,
      match_data,
      "blue",
   );

   // Red Alliance
   let red_teams = &match_data.alliances.red.team_keys;
   let red_won = winner.as_deref() == Some("red");

   let red_title = if red_won {
      format!(" 🔴 Red Alliance ({}) - WON ", red_score)
   } else {
      format!(" 🔴 Red Alliance ({}) ", red_score)
   };

   render_alliance_teams(
      f,
      app,
      red_teams,
      selected_team_index,
      3,
      chunks[1],
      &red_title,
      Color::Red,
      match_data,
      "red",
   );
}

fn render_scheduled_match(
   f: &mut Frame,
   app: &App,
   match_data: &Match,
   selected_team_index: usize,
   area: Rect,
) {
   let chunks = Layout::default()
      .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
      .split(area);

   // Blue Alliance
   let blue_teams = &match_data.alliances.blue.team_keys;
   render_alliance_teams(
      f,
      app,
      blue_teams,
      selected_team_index,
      0,
      chunks[0],
      " 🔵 Blue Alliance ",
      Color::Blue,
      match_data,
      "blue",
   );

   // Red Alliance
   let red_teams = &match_data.alliances.red.team_keys;
   render_alliance_teams(
      f,
      app,
      red_teams,
      selected_team_index,
      3,
      chunks[1],
      " 🔴 Red Alliance ",
      Color::Red,
      match_data,
      "red",
   );
}

fn render_alliance_teams(
   f: &mut Frame,
   app: &App,
   team_keys: &[String],
   selected_team_index: usize,
   offset: usize,
   area: Rect,
   title: &str,
   color: Color,
   match_data: &Match,
   alliance: &str,
) {
   let mut items: Vec<ListItem> = team_keys
      .iter()
      .enumerate()
      .map(|(i, team_key)| {
         let team = app.get_team(team_key);
         let team_number = team
            .map(|t| t.team_number.to_string())
            .unwrap_or_else(|| team_key.clone());
         let team_name = team.map(|t| t.nickname.as_str()).unwrap_or("Unknown");

         let line = Line::from(vec![
            Span::styled(
               format!("  {}  ", team_number),
               Style::default().fg(Color::Yellow),
            ),
            Span::raw(team_name),
            Span::styled(" [Press Enter]", Style::default().fg(Color::DarkGray)),
         ]);

         let style = if i + offset == selected_team_index {
            Style::default()
               .bg(Color::DarkGray)
               .add_modifier(Modifier::BOLD)
         } else {
            Style::default()
         };

         ListItem::new(line).style(style)
      })
      .collect();

   // Add score breakdown if available
   if let Some(breakdown) = &match_data.score_breakdown {
      let alliance_breakdown = if alliance == "blue" {
         &breakdown.blue
      } else {
         &breakdown.red
      };

      let auto = alliance_breakdown
         .get("autoPoints")
         .and_then(|v| v.as_i64())
         .unwrap_or(0);
      let teleop = alliance_breakdown
         .get("teleopPoints")
         .and_then(|v| v.as_i64())
         .unwrap_or(0);
      let endgame = alliance_breakdown
         .get("endgamePoints")
         .and_then(|v| v.as_i64())
         .unwrap_or(0);

      items.push(ListItem::new(Line::from("")));
      items.push(ListItem::new(Line::from(vec![
         Span::styled("  Auto: ", Style::default().fg(Color::Gray)),
         Span::styled(auto.to_string(), Style::default().fg(Color::White)),
         Span::styled("  Teleop: ", Style::default().fg(Color::Gray)),
         Span::styled(teleop.to_string(), Style::default().fg(Color::White)),
         Span::styled("  Endgame: ", Style::default().fg(Color::Gray)),
         Span::styled(endgame.to_string(), Style::default().fg(Color::White)),
      ])));
   }

   let list = List::new(items).block(
      Block::default()
         .borders(Borders::ALL)
         .title(title)
         .border_style(Style::default().fg(color)),
   );

   let mut state = ListState::default();
   if selected_team_index >= offset && selected_team_index < offset + team_keys.len() {
      state.select(Some(selected_team_index - offset));
   }

   f.render_stateful_widget(list, area, &mut state);
}
