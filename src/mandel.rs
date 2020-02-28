use wasm_bindgen::prelude::*;

//use complex::Cplx;
use num_complex::Complex as Cplx;

use crate::utils;
use crate::ZPlane;

#[wasm_bindgen]
pub struct Mandel {
  z: ZPlane,
  // c: Cplx<f64>, // as in z <-> z*z + c
  // a: Cplx<f64>, // attrction point that c moves to
}


#[wasm_bindgen] 
impl Mandel {

  // TODO need non-centred zplane...
  pub fn new(width: u32, height: u32) -> Mandel {

    utils::set_panic_hook();

    let bottom_left = Cplx::<f64>::new(-2.0, -1.25);
    let top_right = Cplx::<f64>::new(0.5, 1.25);

    let mut mandel = Mandel {
      z: ZPlane::new(bottom_left, top_right, width, height),
    };
    mandel.draw();
    mandel
  }

  pub fn cells(&self) -> *const u8 {
    self.z.cells.as_ptr()
  }


  pub fn zoom(&mut self, row: u32, col: u32) {

    let (c, _) = self.z.get_point(row, col);
    self.z.rscale *= 2.0;
    self.z.iscale *= 2.0;
    let dr = (self.z.zmax.re - self.z.zmin.re) / 4.0;
    let di = (self.z.zmax.im - self.z.zmin.im) / 4.0;
    self.z.zmin.re = c.re - dr;
    self.z.zmin.im = c.im - di;
    self.z.zmax.re = c.re + dr;
    self.z.zmax.im = c.im + di;
    self.draw();
  }
  
  fn draw(&mut self) {

    for row in 0..self.z.height {
      for col in 0..self.z.width {
        let (c, idx) = self.z.get_point(row, col);
        let mut z = Cplx::new(0.0, 0.0);
        let mut it = 0;
        while it < 256 && z.norm_sqr() < 4.0 {
          z = z * z + c;
          it += 1;
        }
        self.z.cells[idx] = it as u8;
      }
    }
  }

}
