use wasm_bindgen::prelude::*;

//use complex::Cplx;
use num_complex::Complex as Cplx;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
  ( $( $t:tt )* ) => {
      web_sys::console::log_1(&format!( $( $t )* ).into());
  }
}

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
  c: Cplx<f64>, // as in z' = z*z + c
  cells: Vec<u8>,
}

const MAX_DEPTH: u8 = 14;

#[wasm_bindgen]
impl ZPlane {

  pub fn new(cr: f64, ci: f64, scale: f64, width: u32, height: u32) -> ZPlane {

    let cells = vec![0u8; (width * height) as usize];

    let mut zplane = ZPlane {
      offset: (width / 2) as f64,
      mult: (width / 2) as f64 / scale,
      width: width,
      height: height,
      c: Cplx::new(cr, ci),
      cells: cells
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

  fn get_index(&self, z: &Cplx<f64>) -> usize {
    let c = (self.offset + z.re/*()*/ * self.mult) as u32;
    let r = (self.offset + z.im/*()*/ * self.mult) as u32;
    (r * self.width + c) as usize
  }

  pub fn tick(&mut self) {
    let theta = self.c.arg();
    self.c = Cplx::from_polar/*from_normarg*/(&(self.c.norm() + 0.005 * (theta/1.57).sin()), &(theta + 0.01));
    self.draw();
  }
  
  // Uses the MIIM algorithm
  fn draw(&mut self) {
    //let mut next = self.cells.clone();
    let mut next = vec![0u8; (self.width * self.height) as usize];

    let mut rng = LCG::new(19937);

    let mut z = Cplx::new(0.0, 0.0);
    let mut sign = 1.0;
    // warmup
    for _ in 0..15 {
      if rng.next_1() % 2 == 1 { sign *= -1.0; }
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
