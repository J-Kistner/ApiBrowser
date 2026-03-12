use crate::api::models::{Match, MatchStatus};
use crate::ui::app::App;
use chrono::{Local, TimeZone};
use ratatui::{
   layout::{Constraint, Layout, Rect},
   style::{Color, Modifier, Style},
   text::{Line, Span},
   widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
   Frame,
};

pub fn render(f: &mut Frame, app: &App, team_key: &str, selected_match_index: usize, area: Rect) {
   let chunks = Layout::default()
      .constraints([
         Constraint::Length(3), // Header
         Constraint::Length(6), // Stats
         Constraint::Min(0),    // Matches
      ])
      .split(area);

   // Header with team info
   render_header(f, app, team_key, chunks[0]);

   // Stats section
   render_stats(f, app, team_key, chunks[1]);

   // Match list
   render_matches(f, app, team_key, selected_match_index, chunks[2]);
}

fn render_header(f: &mut Frame, app: &App, team_key: &str, area: Rect) {
   if let Some(stats) = app.get_team_stats(team_key) {
      let header_text = vec![Line::from(vec![
         Span::styled(
            format!("Team {} - {} ", stats.team_number, stats.team_name),
            Style::default()
               .fg(Color::Cyan)
               .add_modifier(Modifier::BOLD),
         ),
         Span::styled(
            format!("Rank: {}/{}", stats.rank, stats.total_teams),
            Style::default().fg(Color::Yellow),
         ),
      ])];

      let header = Block::default()
         .borders(Borders::ALL)
         .title(" Team Detail ")
         .style(Style::default());

      let paragraph = Paragraph::new(header_text).block(header);
      f.render_widget(paragraph, area);
   }
}

fn render_stats(f: &mut Frame, app: &App, team_key: &str, area: Rect) {
   if let Some(stats) = app.get_team_stats(team_key) {
      let stats_text = if stats.matches_played == 0 {
         vec![
            Line::from("No matches played yet"),
            Line::from("Check back after qualification matches begin"),
         ]
      } else {
         vec![
            Line::from(vec![
               Span::raw("  Auto Points: "),
               Span::styled(
                  format!("{:.1}", stats.auto_average),
                  Style::default().fg(Color::Green),
               ),
               Span::raw("      Teleop Points: "),
               Span::styled(
                  format!("{:.1}", stats.teleop_average),
                  Style::default().fg(Color::Green),
               ),
            ]),
            Line::from(vec![
               Span::raw("  Endgame Points: "),
               Span::styled(
                  format!("{:.1}", stats.endgame_average),
                  Style::default().fg(Color::Green),
               ),
               Span::raw("   RP Average: "),
               Span::styled(
                  format!("{:.2}", stats.rp_average),
                  Style::default().fg(Color::Green),
               ),
            ]),
            Line::from(vec![
               Span::raw("  Record: "),
               Span::styled(&stats.record, Style::default().fg(Color::Yellow)),
               Span::raw("   OPR: "),
               Span::styled(
                  format!("{:.1}", stats.opr),
                  Style::default().fg(Color::Yellow),
               ),
               Span::raw("   Matches Played: "),
               Span::styled(
                  stats.matches_played.to_string(),
                  Style::default().fg(Color::Yellow),
               ),
            ]),
         ]
      };

      let stats_block = Paragraph::new(stats_text).block(
         Block::default()
            .borders(Borders::ALL)
            .title(" Season Averages "),
      );

      f.render_widget(stats_block, area);
   }
}

fn render_matches(
   f: &mut Frame,
   app: &App,
   team_key: &str,
   selected_match_index: usize,
   area: Rect,
) {
   let matches = app.get_team_matches(team_key);

   let header = Row::new(vec!["", "Match", "Alliance", "Result", "Score", "RP"])
      .style(
         Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
      )
      .height(1);

   let rows: Vec<Row> = matches
      .iter()
      .enumerate()
      .map(|(i, match_data)| {
         let (icon, match_name, alliance, result, score, rp) =
            format_match_row(team_key, match_data);

         let style = if i == selected_match_index {
            Style::default()
               .bg(Color::DarkGray)
               .fg(Color::White)
               .add_modifier(Modifier::BOLD)
         } else {
            Style::default()
         };

         Row::new(vec![
            Cell::from(icon),
            Cell::from(match_name),
            Cell::from(alliance),
            Cell::from(result),
            Cell::from(score),
            Cell::from(rp),
         ])
         .style(style)
      })
      .collect();

   let widths = [
      Constraint::Length(2),
      Constraint::Length(15),
      Constraint::Length(13),
      Constraint::Length(12),
      Constraint::Length(12),
      Constraint::Length(6),
   ];

   let table = Table::new(rows, widths).header(header).block(
      Block::default()
         .borders(Borders::ALL)
         .title(" Match Schedule ")
         .title_bottom(" [↑/↓: Navigate] [Enter: Match Details] [Esc: Back] [q: Quit] "),
   );

   let mut state = TableState::default();
   state.select(Some(selected_match_index));

   f.render_stateful_widget(table, area, &mut state);
}

fn format_match_row(
   team_key: &str,
   match_data: &Match,
) -> (String, String, String, String, String, String) {
   let match_name = match_data.display_name();

   // Determine which alliance
   let is_blue = match_data
      .alliances
      .blue
      .team_keys
      .contains(&team_key.to_string());
   let alliance = if is_blue { "Blue" } else { "Red" };

   match match_data.status() {
      MatchStatus::Completed {
         blue_score,
         red_score,
         winner,
      } => {
         let our_score = if is_blue { blue_score } else { red_score };
         let their_score = if is_blue { red_score } else { blue_score };

         let result = match winner.as_deref() {
            Some("blue") if is_blue => "Won",
            Some("red") if !is_blue => "Won",
            Some(_) => "Lost",
            None => "Tie",
         };

         let score = format!("{}-{}", our_score, their_score);

         // Extract RP from score breakdown
         let rp = if let Some(breakdown) = &match_data.score_breakdown {
            let alliance_breakdown = if is_blue {
               &breakdown.blue
            } else {
               &breakdown.red
            };

            alliance_breakdown
               .get("rp")
               .and_then(|v| v.as_i64())
               .map(|v| v.to_string())
               .unwrap_or_else(|| "?".to_string())
         } else {
            "?".to_string()
         };

         (
            "✓".to_string(),
            match_name,
            alliance.to_string(),
            result.to_string(),
            score,
            rp.to_string(),
         )
      }
      MatchStatus::Scheduled { time } => {
         let time_str = if let Some(timestamp) = time {
            let dt = Local.timestamp_opt(timestamp, 0).unwrap();
            dt.format("%a %-I:%M %p").to_string()
         } else {
            "Not scheduled".to_string()
         };

         (
            "⏰".to_string(),
            match_name,
            alliance.to_string(),
            "Scheduled".to_string(),
            time_str,
            "-".to_string(),
         )
      }
   }
}
