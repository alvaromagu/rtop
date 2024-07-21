use sys_info;

#[path = "rmath.rs"] mod rmath;

pub struct RamInfo {
  pub total: f64,
  pub free: f64,
  pub used: f64,
  pub used_percent: f64
}

pub fn ram_info () -> RamInfo {
  let mem = sys_info::mem_info().expect("Could not get mem info");
  let total_in_gb: f64 = rmath::parse_data_size(mem.total as f64, rmath::SizeUnit::KB, rmath::SizeUnit::GB);
  let free_in_gb: f64 = rmath::parse_data_size(mem.free as f64, rmath::SizeUnit::KB, rmath::SizeUnit::GB);
  let used_in_gb: f64 = total_in_gb - free_in_gb;
  let used_percent: f64 = (used_in_gb / total_in_gb) * 100.0;
  return RamInfo {
    total: total_in_gb,
    free: free_in_gb,
    used: used_in_gb,
    used_percent
  };
}