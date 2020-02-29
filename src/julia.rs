use wasm_bindgen::prelude::*;

//use complex::Cplx;
use num_complex::Complex as Cplx;

use crate::utils;
use crate::ZPlane;

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
pub struct Julia {
  z: ZPlane<u8>,
  c: Cplx<f64>, // as in z <-> z*z + c
  a: Cplx<f64>, // attrction point that c moves to
  rng: LCG
}

const MAX_DEPTH: u8 = 14;

// speed at which c is pulled to a
const SPEED: f64 = 0.01;

#[wasm_bindgen] 
impl Julia {

  pub fn new(cr: f64, ci: f64, scale: f64, width: u32, height: u32) -> Julia {

    utils::set_panic_hook();

    let mut julia = Julia {
      z: ZPlane::<u8>::new(Cplx::new(-scale, -scale), Cplx::new(scale, scale), width, height),
      c: Cplx::new(cr, ci),
      a: Cplx::new(0.0, 0.0),
      rng: LCG::new(19937)
    };
    julia.draw();
    julia
  }

  pub fn cells(&self) -> *const u8 {
    self.z.cells.as_ptr()
  }

  pub fn locus_r(&self) -> u32 {
    ((self.c.re - self.z.zmin.re) * self.z.rscale) as u32
  }

  pub fn locus_i(&self) -> u32 {
    ((self.c.im - self.z.zmin.im) * self.z.iscale) as u32
  }

  pub fn set_attract(&mut self, row: u32, col: u32) {
    let (c, _) = self.z.get_point(row, col);
    self.a = c;
  }

  pub fn tick(&mut self) {
    //let theta = self.c.arg();
    //let dr = 0.0001 * (1.0 - (self.rng.next_1() as f64 / std::i32::MAX as f64));
    //self.c = Cplx::from_polar(&(self.c.norm() * (1.0 + dr)), &(theta + 0.01));
    self.c += Cplx::new((self.a.re - self.c.re) * SPEED, (self.a.im - self.c.im) * SPEED);
    if self.c.re > self.z.zmax.re { self.c.re = self.z.zmax.re; }
    if self.c.re < self.z.zmin.re { self.c.re = self.z.zmin.re; }
    if self.c.im > self.z.zmax.im { self.c.im = self.z.zmax.im; }
    if self.c.im < self.z.zmin.im { self.c.im = self.z.zmin.im; }

    self.draw();
  }
  
  // Uses the MIIM algorithm
  fn draw(&mut self) {
    //let mut next = self.cells.clone();
    let mut next = vec![0u8; (self.z.width * self.z.height) as usize];

    //let mut rng = LCG::new(19937);

    let mut z = Cplx::new(0.0, 0.0);
    let mut sign = 1.0;
    // warmup
    for _ in 0..25 {
      if self.rng.next_1() % 2 == 1 { sign *= -1.0; }
      z = (z - self.c).sqrt() * sign;
    }
    self.draw_impl(z, &mut next, 0);

    self.z.cells = next;
  }

  fn draw_impl(&mut self, z: Cplx<f64>, cells: &mut Vec<u8>, depth: u8) {

    let z = (z - self.c).sqrt();

    let idx = self.z.get_index(&z);
    cells[idx] += 1;
    let idx = self.z.get_index(&-z);
    cells[idx] += 1;
    if depth >= MAX_DEPTH { return; }

    self.draw_impl(z, cells, depth+1);
    self.draw_impl(-z, cells, depth+1);
  }

}
