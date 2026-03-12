use crate::ui::app::App;
use ratatui::{
   layout::{Constraint, Layout, Rect},
   style::{Color, Modifier, Style},
   text::{Line, Span},
   widgets::{Block, Borders, Cell, Row, Table, TableState},
   Frame,
};

pub fn render(
   f: &mut Frame,
   app: &App,
   selected_index: usize,
   search_query: &str,
   searching: bool,
   area: Rect,
) {
   let chunks = if searching {
      Layout::default()
         .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(3), // Search box
            Constraint::Min(0),    // Team list
         ])
         .split(area)
   } else {
      Layout::default()
         .constraints([Constraint::Length(3), Constraint::Min(0)])
         .split(area)
   };

   // Header
   render_header(f, app, chunks[0]);

   if searching {
      // Search box
      render_search_box(f, search_query, chunks[1]);
      // Team list table
      render_team_list(f, app, selected_index, chunks[2]);
   } else {
      // Team list table
      render_team_list(f, app, selected_index, chunks[1]);
   }
}

fn render_header(f: &mut Frame, app: &App, area: Rect) {
   let year = if app.test_mode {
      format!("{} [TEST MODE]", app.data.event.year)
   } else {
      app.data.event.year.to_string()
   };

   let header_text = vec![Line::from(vec![
      Span::styled(
         format!("{} ", app.data.event.name),
         Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
      ),
      Span::styled(year, Style::default().fg(Color::Yellow)),
   ])];

   let header = Block::default()
      .borders(Borders::ALL)
      .title(" Event ")
      .style(Style::default());

   let paragraph = ratatui::widgets::Paragraph::new(header_text).block(header);
   f.render_widget(paragraph, area);
}

fn render_search_box(f: &mut Frame, search_query: &str, area: Rect) {
   let search_text = vec![Line::from(vec![
      Span::styled("Search Team #: ", Style::default().fg(Color::Yellow)),
      Span::styled(
         search_query,
         Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
      ),
      Span::styled("_", Style::default().fg(Color::Gray)),
   ])];

   let search_box = ratatui::widgets::Paragraph::new(search_text).block(
      Block::default()
         .borders(Borders::ALL)
         .title(" Search ")
         .title_bottom(" [Type number] [Enter: Go] [Esc: Cancel] ")
         .style(Style::default().fg(Color::Cyan)),
   );

   f.render_widget(search_box, area);
}

fn render_team_list(f: &mut Frame, app: &App, selected_index: usize, area: Rect) {
   let header = Row::new(vec!["Rank", "Team #", "Team Name", "Record", "RP Avg"])
      .style(
         Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
      )
      .height(1);

   let rows: Vec<Row> = app
      .data
      .team_stats
      .iter()
      .enumerate()
      .map(|(i, stats)| {
         let style = if i == selected_index {
            Style::default()
               .bg(Color::DarkGray)
               .fg(Color::White)
               .add_modifier(Modifier::BOLD)
         } else {
            Style::default()
         };

         Row::new(vec![
            Cell::from(stats.rank.to_string()),
            Cell::from(stats.team_number.to_string()),
            Cell::from(stats.team_name.clone()),
            Cell::from(stats.record.clone()),
            Cell::from(format!("{:.2}", stats.rp_average)),
         ])
         .style(style)
      })
      .collect();

   let widths = [
      Constraint::Length(6),
      Constraint::Length(8),
      Constraint::Min(20),
      Constraint::Length(10),
      Constraint::Length(8),
   ];

   let table = Table::new(rows, widths)
      .header(header)
      .block(
         Block::default()
            .borders(Borders::ALL)
            .title(" Teams (Sorted by Rank) ")
            .title_bottom(" [↑/↓: Navigate] [/: Search] [r: Refresh] [Enter: Details] [q: Quit] "),
      )
      .row_highlight_style(
         Style::default()
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
      );

   let mut state = TableState::default();
   state.select(Some(selected_index));

   f.render_stateful_widget(table, area, &mut state);
}
