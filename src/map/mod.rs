use eframe::egui::Vec2;

pub struct Map
{
  resolution : f64, // Meters / Cell
  width : u32,
  height : u32,
}

impl Map {
  pub fn new(res : f64, w: u32, h : u32) -> Self {
    Map {
      resolution: res,
      width: w,
      height: h,
    }
  }

  pub fn get_size(&self) -> Vec2{
    let size_vector = Vec2::new(self.width as f32, self.height as f32);

    size_vector
  }
}

