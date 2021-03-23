use rand::{thread_rng, Rng};
use std::f32::consts::PI;

pub struct Temp {
  average: f32,
  max_temp: f32,
  min_temp: f32,
}

impl Temp {
  /// constructor.
  ///
  /// ### Args
  ///
  /// - average (Option<f32>): average temp value.
  /// - max (Option<f32>): max temp value.
  /// - min (Option<f32>): min temp value.
  ///
  /// ### Returns
  ///
  /// Self: this instance.
  ///
  pub fn new(average: Option<f32>, max: Option<f32>, min: Option<f32>) -> Self {
    let average = match average {
      Some(t) => t,
      None => 36.0,
    };

    let max_temp = match max {
      Some(t) => t,
      None => 42.0,
    };

    let min_temp = match min {
      Some(t) => t,
      None => 35.0,
    };

    Self {
      average: average,
      max_temp: max_temp,
      min_temp: min_temp,
    }
  }

  /// Create temp value.
  ///
  /// ### Returns
  ///
  /// f32: Randomly created with a normal distribution.
  pub fn create(&self) -> f32 {
    let mut rng = thread_rng();
    box_muller(&mut rng, self.average, self.max_temp - self.min_temp)
  }

  /// Create multiple temp value.
  ///
  /// ### Args
  ///
  /// - size (usize): value size.
  ///
  /// ### Returns
  ///
  /// Vec<f32> : a list of temp values.
  pub fn create_multiple(&self, size: usize) -> Vec<f32> {
    let mut temps: Vec<f32> = Vec::with_capacity(size);

    for _ in 0..size {
      temps.push(self.create());
    }

    temps
  }
}

///
/// normal distribution
///
/// ### Args
///
/// - rng (Rng): rand instance. create by thread_rng.
/// - mu (f32): Normal distribution mean.
/// - sigma (f32): Variance of normal distribution.
///
/// ### Returns
///
/// f32: value.
fn box_muller<R: Rng>(rng: &mut R, mu: f32, sigma: f32) -> f32 {
  let u1 = rng.gen::<f32>();
  let u2 = rng.gen::<f32>();

  mu + (-2.0 * u1.ln() * sigma).sqrt() * (2.0 * PI * u2).cos()
}
