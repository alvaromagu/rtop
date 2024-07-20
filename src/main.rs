mod rmath;

use sys_info;

fn main() {
  let mem = sys_info::mem_info().expect("Could not get mem info");
  let total_in_gb: f64 = rmath::parse_data_size(mem.total as f64, rmath::SizeUnit::KB, rmath::SizeUnit::GB);
  println!("Total RAM: {} GB", rmath::round(total_in_gb, 2));
  let free_in_gb: f64 = rmath::parse_data_size(mem.free as f64, rmath::SizeUnit::KB, rmath::SizeUnit::GB);
  println!("Free RAM: {} GB", rmath::round(free_in_gb, 2));
  let used_in_gb: f64 = total_in_gb - free_in_gb;
  println!("Used RAM: {} GB", rmath::round(used_in_gb, 2));
}
