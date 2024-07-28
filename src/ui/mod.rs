use std::{cmp::min, ops::Range};

use ratatui::{
  layout::{Alignment, Constraint, Direction, Layout, Rect},
  widgets::{Block, Gauge, Paragraph},
  Frame,
};

#[path = "../rsys/mod.rs"]
mod rsys;

// constants
const CORES_PER_ROW: f64 = 8.0;

pub fn render_app(frame: &mut Frame) {
  let cpu = rsys::cpu().expect("could not get cpu info");
  let areas = Layout::new(
    Direction::Vertical,
    [
      Constraint::Length(3),
      Constraint::Length(calc_cpus_height(cpu.cores.len())),
      Constraint::Min(1),
      Constraint::Length(1),
    ],
  )
  .split(frame.size());
  frame.render_widget(mem_widget(), areas[0]);
  render_cores(frame, areas[1], cpu.cores);
  frame.render_widget(Block::new(), areas[2]);
  let quit_message = Paragraph::new("Press 'q' to quit").alignment(Alignment::Right);
  frame.render_widget(quit_message, areas[3]);
}

fn calc_cpus_height(len: usize) -> u16 {
  let rows = calc_cpus_rows(len);
  let row_height = 3;
  rows * row_height
}

fn calc_cpus_rows (len: usize) -> u16 {
  let len = len as f64;
  (len / CORES_PER_ROW).ceil() as u16
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

fn get_row_range (i: usize, cores: usize) -> Range<usize> {
  let cores_per_row = CORES_PER_ROW as usize;
  let start = i * cores_per_row;
  let end = min(start + cores_per_row, cores);
  start..end
}

fn render_cores(frame: &mut Frame, cpu_area: Rect, cores: Vec<rsys::CoreInfo>) {
  let rows = calc_cpus_rows(cores.len());
  let cores_per_row_u = CORES_PER_ROW as usize;
  // rows areas
  let rows = Layout::new(Direction::Vertical, vec![Constraint::Length(3); rows as usize])
    .split(cpu_area);
  for (i, row) in rows.iter().enumerate() {
    let cols = Layout::new(
        Direction::Horizontal, 
        vec![
          Constraint::Percentage(100 / cores_per_row_u as u16); 
          cores_per_row_u as usize
        ]
      )
      .split(*row);
    // row cores: starting from i * cores.len() to min  between i * cores.len() + CORES_PER_ROW and cores.len()
    let row_cores = &cores[get_row_range(i, cores.len())];
    for (j, core) in row_cores.iter().enumerate() {
      frame.render_widget(core_widget(core), cols[j]);
    }
  }
}

fn core_widget(core: &rsys::CoreInfo) -> Gauge {
  let label = format!("{:.2}%", core.usage);
  return Gauge::default()
    .block(Block::bordered().title("CPU"))
    .gauge_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan))
    .percent(core.usage as u16)
    .label(label);
}
