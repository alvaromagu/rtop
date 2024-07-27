use ratatui::{
  layout::{Alignment, Constraint, Direction, Layout},
  widgets::{Block, Gauge, Paragraph},
  Frame,
};

mod rscreen;
mod rsys;

fn main() -> Result<(), ()> {
  let mut screen = rscreen::Screen::new().expect("failed to create screen");
  screen.run(render_app);
  screen.restore();
  Ok(())
}

fn render_app(frame: &mut Frame) {
  let areas = Layout::new(
    Direction::Vertical,
    [
      Constraint::Length(3),
      Constraint::Min(1),
      Constraint::Length(1),
    ],
  )
  .split(frame.size());
  frame.render_widget(mem_widget(), areas[0]);
  frame.render_widget(Block::new(), areas[1]);
  let quit_message = Paragraph::new("Press 'q' to quit").alignment(Alignment::Right);
  frame.render_widget(quit_message, areas[2]);
}

fn mem_widget<'a>() -> Gauge<'a> {
  let mem = rsys::mem().expect("could not get memory info");
  let label = format!("{:.2}/{:.2} GB", mem.used, mem.total);
  return Gauge::default()
    .block(Block::bordered().title("RAM usage"))
    .gauge_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan))
    .percent(mem.used_percent as u16)
    .label(label);
}
