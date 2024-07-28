use serde::Deserialize;
use wmi::{COMLibrary, WMIConnection};

#[path ="../../rmath/mod.rs"]
mod rmath;

pub use rmath::SizeUnit;

pub struct RamInfo {
  pub total: f64,
  pub used: f64,
  pub used_percent: f64,
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct Win32Ram {
  pub TotalVisibleMemorySize: f64,
  pub FreePhysicalMemory: f64,
}

impl Win32Ram {
  pub fn to(&self) -> RamInfo {
    let total = rmath::parse_data_size(
      self.TotalVisibleMemorySize,
      SizeUnit::KB,
      rmath::SizeUnit::GB,
    );
    let free = rmath::parse_data_size(
      self.FreePhysicalMemory,
      SizeUnit::KB,
      SizeUnit::GB,
    );
    RamInfo {
      total,
      used: total - free,
      used_percent: (total - free) / total * 100f64,
    }
  }
}

pub fn mem() -> Result<RamInfo, Box<dyn std::error::Error>> {
  let com_con = COMLibrary::new()?;
  let wmi_con = WMIConnection::new(com_con.into())?;
  // Get-WmiObject -Query "SELECT * FROM Win32_OperatingSystem" | select FreePhysicalMemory, TotalVisibleMemorySize, TotalVirtualMemorySize, FreeVirtualMemory
  let os_info: Vec<Win32Ram> = wmi_con
    .raw_query("SELECT TotalVisibleMemorySize, FreePhysicalMemory FROM Win32_OperatingSystem")?;
  if os_info.len() < 1 {
    return Err("No data returned".into());
  }
  let os_info = &os_info[0];
  Ok(os_info.to())
}

pub struct CoreInfo {
  pub usage: f64,
}

pub struct CpuInfo {
  pub cores: Vec<CoreInfo>
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Win32Cpu {
  pub PercentProcessorTime: f64,
}

impl Win32Cpu {
  pub fn to(&self) -> CoreInfo {
    CoreInfo {
      usage: self.PercentProcessorTime,
    }
  }
}

pub fn cpu() -> Result<CpuInfo, Box<dyn std::error::Error>> {
  let com_con = COMLibrary::new()?;
  let wmi_con = WMIConnection::new(com_con.into())?;
  // Get-WmiObject -Query "SELECT * FROM Win32_PerfFormattedData_PerfOS_Processor" | select Name, PercentProcessorTime, Frequency_Object
  let cpu_info: Vec<Win32Cpu> = wmi_con
    .raw_query("SELECT PercentProcessorTime, Frequency_Object FROM Win32_PerfFormattedData_PerfOS_Processor")?;
  if cpu_info.len() < 1 {
    return Err("No data returned".into());
  }
  let mut cores = Vec::new();
  for core in cpu_info {
    cores.push(core.to());
  }
  Ok(CpuInfo {
    cores
  })
}
