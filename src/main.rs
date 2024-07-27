mod rscreen;
mod ui;

use rscreen::Screen;

fn main() -> Result<(), ()> {
  let mut screen = Screen::new().expect("failed to create screen");
  screen.run(ui::render_app);
  screen.restore();
  Ok(())
}
