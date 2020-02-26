
mod utils;
mod julia;

use num_complex::Complex as Cplx;

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

pub struct ZPlane {
  // scale: f64,
  // z0: Cplx<f64>,
  zmin: Cplx<f64>, // bottom left
  zmax: Cplx<f64>, // top right
  rscale: f64,
  iscale: f64,
  width: u32,
  height: u32,
  cells: Vec<u8>,
}

impl ZPlane {
  pub fn new(zmin: Cplx<f64>, zmax: Cplx<f64>, width: u32, height: u32) -> ZPlane {

    ZPlane {
      zmin: zmin,
      zmax: zmax,
      width: width,
      height: height,
      rscale: width as f64 / (zmax.re - zmin.re),
      iscale: height as f64 / (zmax.im - zmin.im),
      cells: vec![0u8; (width * height) as usize]
    }
  }

  fn get_index(&self, z: &Cplx<f64>) -> usize {
    // let c = (self.offset + z.re/*()*/ * self.mult) as u32;
    // let r = (self.offset + z.im/*()*/ * self.mult) as u32;
    //let idx = (z - self.zmin) * self.zmult;
    let r = ((z.re - self.zmin.re) * self.rscale) as u32;
    let c = ((z.im - self.zmin.im) * self.iscale) as u32; 
    (c * self.width + r) as usize
  }

}



