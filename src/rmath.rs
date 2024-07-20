pub fn round(x: f64, n: i32) -> f64 {
  let p = 10f64.powi(n);
  return (x * p).round() / p;
}

// size units enum
pub enum SizeUnit {
  KB,
  GB,
}

impl SizeUnit {
  pub fn to_f64(&self) -> f64 {
      match self {
          SizeUnit::KB => 1024.0,
          SizeUnit::GB => 1024.0f64.powi(3),
      }
  }
}

// fun to parse data size
// first parameter is the size
// second parameter is the from size unit
// third parameter is the to size unit
// return the size in the new unit
pub fn parse_data_size(size: f64, from: SizeUnit, to: SizeUnit) -> f64 {
    return size * from.to_f64() / to.to_f64();
}
