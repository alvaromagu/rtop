use std::{io::{self, Stdout}, time::Duration};
use ratatui::{
  backend::CrosstermBackend, crossterm::{event, execute, terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen
  }}, layout::{Constraint, Direction, Layout}, widgets::Paragraph, Frame, Terminal
};

mod rsys_info;

fn main() -> Result<(), ()> {
  let mut terminal = setup_terminal().expect("setup failed");
  run(&mut terminal).expect("run failed");
  restore_terminal(&mut terminal).expect("restore terminal failed");
  Ok(())
}

fn setup_terminal () -> Result<Terminal<CrosstermBackend<Stdout>>, ()> {
  let mut stdout = io::stdout();
  enable_raw_mode().expect("failed to enable raw mode");
  execute!(stdout, EnterAlternateScreen).expect("unable to switch to alternate screen");
  let terminal = Terminal::new(CrosstermBackend::new(stdout)).expect("unable to create terminal");
  Ok(terminal)
}

fn restore_terminal (terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), ()> {
  disable_raw_mode().expect("failed to disable raw mode");
  execute!(terminal.backend_mut(), LeaveAlternateScreen).expect("unable to switch to main screen");
  terminal.show_cursor().expect("unable to show cursor");
  Ok(())
}

fn run (terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), ()> {
  loop {
    terminal.draw(|f| {
      render_app(f);
    }).expect("draw failed");
    if should_quit().expect("should_quit failed") {
      break;
    }
  }
  Ok(())
}

fn render_app (frame: &mut Frame) {
  let areas = Layout::new(
    Direction::Vertical,
    [
      Constraint::Length(1),
      Constraint::Min(1)
    ]
  ).split(frame.size());

  let ram_info = rsys_info::ram_info();
  let ram_info_str = format!("Total: {:.2} GB, Free: {:.2} GB, Used: {:.2} GB", ram_info.total, ram_info.free, ram_info.used);
  let ram_info = Paragraph::new(ram_info_str);
  frame.render_widget(ram_info,  areas[0]);
  let quit_message = Paragraph::new("Press 'q' to quit");
  frame.render_widget(quit_message, areas[1]);
}

fn should_quit () -> Result<bool, ()> {
  if event::poll(Duration::from_millis(1000)).expect("event poll failed") {
    if let event::Event::Key(key) = event::read().expect("event read failed") {
      return Ok(key.code == event::KeyCode::Char('q'));
    }
  }
  Ok(false)
}
