use wasm_bindgen::prelude::*;

//use complex::Cplx;
use num_complex::Complex as Cplx;

use crate::utils;

// need to copy this from rand beacuse the crate links to C++ static libs
pub struct LCG {
  /// The seed
  r: u32
}

impl LCG {
  const A: u64 = 48271;
  const M: u64 = std::i32::MAX as u64;

  pub fn new(seed: u32) -> LCG {
    assert_ne!(seed, 0);
    LCG{r: seed}   
  }

  fn next_1(&mut self) -> u32 {
    self.r = ((self.r as u64 * LCG::A) % LCG::M) as u32;
    self.r
  }

}


#[wasm_bindgen]
pub struct ZPlane {
  offset: f64,
  mult: f64, 
  width: u32,
  height: u32,
  c: Cplx<f64>, // as in z <-> z*z + c
  a: Cplx<f64>, // attrction point that c moves to
  cells: Vec<u8>,
  rng: LCG
}

const MAX_DEPTH: u8 = 14;

// speed at which c is pulled to a
const SPEED: f64 = 0.01;

#[wasm_bindgen] 
impl ZPlane {

  pub fn new(cr: f64, ci: f64, scale: f64, width: u32, height: u32) -> ZPlane {

    utils::set_panic_hook();

    let cells = vec![0u8; (width * height) as usize];

    let mut zplane = ZPlane {
      offset: (width / 2) as f64,
      mult: (width / 2) as f64 / scale,
      width: width,
      height: height,
      c: Cplx::new(cr, ci),
      a: Cplx::new(0.0, 0.0),
      cells: cells,
      rng: LCG::new(19937)
    };
    zplane.draw();
    zplane
  }

  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
  }

  pub fn cells(&self) -> *const u8 {
    self.cells.as_ptr()
  }

  pub fn locus_r(&self) -> u32 {
    (self.offset + self.c.re/*()*/ * self.mult) as u32
  }

  pub fn locus_i(&self) -> u32 {
    (self.offset + self.c.im/*()*/ * self.mult) as u32
  }

  pub fn set_attract_r(&mut self, r: f64) {
    self.a.re = r;
  }

  pub fn set_attract_i(&mut self, i: f64) {
    self.a.im = i;
  }

  fn get_index(&self, z: &Cplx<f64>) -> usize {
    let c = (self.offset + z.re/*()*/ * self.mult) as u32;
    let r = (self.offset + z.im/*()*/ * self.mult) as u32;
    (r * self.width + c) as usize
  }

  pub fn tick(&mut self) {
    //let theta = self.c.arg();
    //let dr = 0.0001 * (1.0 - (self.rng.next_1() as f64 / std::i32::MAX as f64));
    //self.c = Cplx::from_polar(&(self.c.norm() * (1.0 + dr)), &(theta + 0.01));
    self.c += Cplx::new((self.a.re - self.c.re) * SPEED, (self.a.im - self.c.im) * SPEED);

    self.draw();
  }
  
  // Uses the MIIM algorithm
  fn draw(&mut self) {
    //let mut next = self.cells.clone();
    let mut next = vec![0u8; (self.width * self.height) as usize];

    //let mut rng = LCG::new(19937);

    let mut z = Cplx::new(0.0, 0.0);
    let mut sign = 1.0;
    // warmup
    for _ in 0..15 {
      if self.rng.next_1() % 2 == 1 { sign *= -1.0; }
      z = (z - self.c).sqrt() * sign;
    }
    self.draw_impl(z, &mut next, 0);

    self.cells = next;
  }

  fn draw_impl(&mut self, z: Cplx<f64>, cells: &mut Vec<u8>, depth: u8) {

    let z = (z - self.c).sqrt();

    let idx = self.get_index(&z);
    cells[idx] += 1;
    let idx = self.get_index(&-z);
    cells[idx] += 1;
    if depth >= MAX_DEPTH { return; }

    self.draw_impl(z, cells, depth+1);
    self.draw_impl(-z, cells, depth+1);
  }

}
