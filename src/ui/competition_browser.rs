use crate::api::models::Event;
use ratatui::{
   layout::{Constraint, Direction, Layout, Rect},
   style::{Color, Modifier, Style},
   text::{Line, Span},
   widgets::{Block, Borders, List, ListItem, Paragraph},
   Frame,
};

pub fn render(
   f: &mut Frame,
   area: Rect,
   events: &[Event],
   selected_index: usize,
   current_event_key: &str,
) {
   let chunks = Layout::default()
      .direction(Direction::Vertical)
      .constraints([
         Constraint::Length(3),
         Constraint::Min(1),
         Constraint::Length(3),
      ])
      .split(area);

   // Header
   let header = Paragraph::new("Select Competition")
      .style(
         Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
      )
      .block(Block::default().borders(Borders::ALL));
   f.render_widget(header, chunks[0]);

   // Filter to only regional events (event_type 1) and sort by start date
   let mut regional_events: Vec<&Event> = events.iter().filter(|e| e.event_type == 1).collect();

   regional_events.sort_by(|a, b| a.start_date.cmp(&b.start_date));

   // Event list
   let items: Vec<ListItem> = regional_events
      .iter()
      .enumerate()
      .map(|(i, event)| {
         let is_selected = i == selected_index;
         let is_current = event.key == current_event_key;

         let mut style = Style::default();
         if is_selected {
            style = style.bg(Color::DarkGray).add_modifier(Modifier::BOLD);
         }
         if is_current {
            style = style.fg(Color::Green);
         }

         let location = format!(
            "{}, {}",
            event.city.as_deref().unwrap_or("Unknown"),
            event.state_prov.as_deref().unwrap_or("")
         );

         let marker = if is_current { "→ " } else { "  " };
         let date = &event.start_date[5..10]; // Extract MM-DD

         let content = format!("{}{:<40} {:<25} {}", marker, event.name, location, date);

         ListItem::new(Line::from(Span::styled(content, style)))
      })
      .collect();

   let list = List::new(items).block(Block::default().borders(Borders::ALL).title(format!(
      " {} Regionals ",
      events.first().map(|e| e.year).unwrap_or(2026)
   )));

   f.render_widget(list, chunks[1]);

   // Footer with instructions
   let footer_text = vec![Line::from(vec![
      Span::styled("↑/↓", Style::default().fg(Color::Yellow)),
      Span::raw(" Navigate  "),
      Span::styled("Enter", Style::default().fg(Color::Yellow)),
      Span::raw(" Select  "),
      Span::styled("Esc", Style::default().fg(Color::Yellow)),
      Span::raw(" Cancel"),
   ])];

   let footer = Paragraph::new(footer_text).block(Block::default().borders(Borders::ALL));
   f.render_widget(footer, chunks[2]);
}
