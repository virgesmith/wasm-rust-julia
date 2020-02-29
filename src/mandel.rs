use wasm_bindgen::prelude::*;

//use complex::Cplx;
use num_complex::Complex as Cplx;

use crate::utils;
use crate::ZPlane;

type Cell = u16;

#[wasm_bindgen]
pub struct Mandel {
  z: ZPlane<Cell>,
  depth: Cell
}

#[wasm_bindgen] 
impl Mandel {

  // TODO need non-centred zplane...
  pub fn new(width: u32, height: u32, maxiter: Cell) -> Mandel {

    utils::set_panic_hook();

    let bottom_left = Cplx::<f64>::new(-2.0, -1.25);
    let top_right = Cplx::<f64>::new(0.5, 1.25);

    let mut mandel = Mandel {
      z: ZPlane::<Cell>::new(bottom_left, top_right, width, height),
      depth: maxiter - 1
    };
    mandel.draw();
    mandel
  }

  pub fn cells(&self) -> *const Cell {
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
    //format!("({},{}) => {:?} {:} {:}", row, col, c, self.z.rscale, self.z.height)
  }
  
  fn draw(&mut self) {

    for row in 0..self.z.height {
      for col in 0..self.z.width {
        let (c, idx) = self.z.get_point(row, col);
        let mut z = Cplx::new(0.0, 0.0);
        let mut it: Cell = 0;
        let mut r2 = 0.0;
        let mut i2 = 0.0;
        while it < self.depth && (r2 + i2) < 4.0 {
          //z = z * z + c;
          // hand optimised
          z.im = (z.re + z.re) * z.im + c.im; 
          z.re = r2 - i2 + c.re;
          r2 = z.re * z.re;
          i2 = z.im * z.im;  
          it += 1;
        }
        self.z.cells[idx] = it as Cell;
      }
    }
  }

}
