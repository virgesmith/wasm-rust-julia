
mod utils;
mod julia;
mod mandel;

use num_complex::Complex as Cplx;
use num_traits::{Unsigned, Zero};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[allow(unused_macros)]
macro_rules! log {
  ( $( $t:tt )* ) => {
      web_sys::console::log_1(&format!( $( $t )* ).into());
  }
}

pub struct ZPlane<T> {
  zmin: Cplx<f64>, // bottom left
  zmax: Cplx<f64>, // top right
  rscale: f64,
  iscale: f64,
  width: u32,
  height: u32,
  cells: Vec<T>,
}

impl<T: Zero + Unsigned + Clone> ZPlane<T> {

  pub fn new(zmin: Cplx<f64>, zmax: Cplx<f64>, width: u32, height: u32) -> ZPlane<T> {

    ZPlane {
      zmin,
      zmax,
      width,
      height,
      rscale: width as f64 / (zmax.re - zmin.re),
      iscale: height as f64 / (zmax.im - zmin.im),
      cells: vec![T::zero(); (width * height) as usize]
    }
  }

  fn get_index(&self, z: &Cplx<f64>) -> usize {
    let r = ((z.re - self.zmin.re) * self.rscale) as u32;
    let c = ((z.im - self.zmin.im) * self.iscale) as u32;
    (c * self.width + r) as usize
  }

  fn get_point(&self, r: u32, c: u32) -> (Cplx::<f64>, usize) {
    (Cplx::new(r as f64 / self.rscale + self.zmin.re,
              c as f64 / self.iscale + self.zmin.im),
    (c * self.width + r) as usize)
  }

}



