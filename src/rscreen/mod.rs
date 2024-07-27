use std::{
  io::{self, Stdout},
  time::Duration,
};

use ratatui::{
  backend::CrosstermBackend,
  crossterm::{
    event, execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  },
  Frame, Terminal,
};

pub struct Screen {
  pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Screen {
  pub fn new() -> Result<Self, ()> {
    let terminal = get_terminal();
    Ok(Self {
      terminal: terminal?,
    })
  }

  pub fn run<F>(&mut self, draw_fn: F)
  where
    F: FnOnce(&mut Frame) + Copy,
  {
    loop {
      self.terminal.draw(draw_fn).expect("draw failed");
      if should_quit().expect("should_quit failed") {
        break;
      }
    }
  }

  pub fn restore(mut self) {
    disable_raw_mode().expect("failed to disable raw mode");
    execute!(self.terminal.backend_mut(), LeaveAlternateScreen)
      .expect("unable to switch to main screen");
    self.terminal.show_cursor().expect("unable to show cursor");
  }
}

fn get_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, ()> {
  let mut stdout = io::stdout();
  enable_raw_mode().expect("failed to enable raw mode");
  execute!(stdout, EnterAlternateScreen).expect("unable to switch to alternate screen");
  let terminal = Terminal::new(CrosstermBackend::new(stdout)).expect("unable to create terminal");
  Ok(terminal)
}

fn should_quit() -> Result<bool, ()> {
  if event::poll(Duration::from_millis(1000)).expect("event poll failed") {
    if let event::Event::Key(key) = event::read().expect("event read failed") {
      return Ok(key.code == event::KeyCode::Char('q'));
    }
  }
  Ok(false)
}
