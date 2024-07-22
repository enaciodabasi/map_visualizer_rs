use eframe::egui::Vec2;
use rusttype::Point;

pub struct Map
{
  resolution : f64, // Meters / Cell
  width : u32,
  height : u32,
  origin : rusttype::Point<f64> 
}

impl Map {
  pub fn new(res : f64, w: u32, h : u32) -> Self {
    Map {
      resolution: res,
      width: w,
      height: h,
      origin: rusttype::Point{x: 0.0, y: 0.0}
    }
  }

  pub fn get_size(&self) -> Vec2{
    let size_vector = Vec2::new(self.width as f32, self.height as f32);

    size_vector
  }

  pub fn get_origin(&self) -> rusttype::Point<f64>{
    self.origin
  }
}

