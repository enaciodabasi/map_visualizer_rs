use crate::map::{self, Map};
use core::num;
use std::sync::mpsc::Sender;
use eframe::egui;
use eframe::emath::RectTransform;
use eframe::{
    egui::{Color32, Frame, Pos2, Rect, Rounding, Sense, Shape, Vec2},
    egui_glow::painter,
    emath,
};
use rusttype::Point;
use std::{borrow::Borrow, default, option, sync::mpsc, thread};

fn point_listening_function(sender: &mpsc::Sender<()>) {
  
  

}

pub struct App {
    map: Map,
    points: Vec<Pos2>,
    grid_line_spacing: f64,
    point_sender: mpsc::Sender<Vec<Pos2>>,
    point_receiver: mpsc::Receiver<Vec<Pos2>>,
    sender_thread_handle: thread::JoinHandle<()>,
}

impl Default for App {
    fn default() -> Self {
        Self::new(map::Map::new(1.0, 500, 500), Some(50.0))
    }
}

// TODO: Add a factory function
//pub fn build_app()

impl App {

    pub fn new(map: map::Map, grid_line_spacing_opt: Option<f64>) -> App{
      let mut grid_line_spacing_res;
      match grid_line_spacing_opt {
        Some(grid_line_spacing) => grid_line_spacing_res = grid_line_spacing,

        None => grid_line_spacing_res = 100.0,
      };

      let (tx, rx) = mpsc::channel::<Vec<Pos2>>();
      let sender_thread_handle = thread::spawn(|| {
        
      });
      App { map: (map), points: (Vec::default()), grid_line_spacing: (grid_line_spacing_res) , point_sender: tx, point_receiver: rx, sender_thread_handle: sender_thread_handle}    
    }

    fn setup_map(&mut self, map: Map) {
        self.map = map;
    }

    fn generate_grid_lines(&self, tf: &RectTransform) -> (Vec<egui::Shape>, Vec<egui::Shape>) {
        let mut horizontal_lines: Vec<egui::Shape> = Vec::default();
        let mut vertical_lines: Vec<egui::Shape> = Vec::default();
        let map_size = self.map.get_size();
        let num_lines: u32 = (map_size.x / 10.0) as u32;

        let mut i = 0;
        let y_begin: f32 = 0.0;
        let y_end: f32 = map_size.y;
        let mut x_begin: f32 = 0.0;

        while i != num_lines {
            let start_point: Pos2 = Pos2 {
                x: x_begin,
                y: y_begin,
            };
            let start_point = tf.transform_pos(start_point);
            let end_point: Pos2 = Pos2 {
                x: x_begin,
                y: y_end,
            };
            let end_point = tf.transform_pos(end_point);
            horizontal_lines.push(egui::Shape::line_segment(
                [start_point, end_point],
                egui::Stroke::new(0.25, egui::Color32::from_rgb(255, 255, 255)),
            ));

            let start_point: Pos2 = Pos2 {
                x: y_begin,
                y: x_begin,
            };
            let start_point = tf.transform_pos(start_point);
            let end_point: Pos2 = Pos2 {
                x: y_end,
                y: x_begin,
            };
            let end_point = tf.transform_pos(end_point);

            vertical_lines.push(egui::Shape::line_segment(
                [start_point, end_point],
                egui::Stroke::new(0.25, egui::Color32::from_rgb(255, 255, 255)),
            ));
            i += 1;
            x_begin += self.grid_line_spacing as f32;
        }

        (horizontal_lines, vertical_lines)
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // Add central panel
        // The central panel consists of a drawn map ...

        let receive_result = self.point_receiver.try_recv();
        if receive_result.is_ok() {
          ctx.request_repaint();
        }

        eframe::egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            eframe::egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            Frame::canvas(ui.style()).show(ui, |ui| {
                let (response, mut painter) = ui.allocate_painter(self.map.get_size(), Sense::hover());
                let to_screen = emath::RectTransform::from_to(
                    Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                    response.rect,
                );

                let points_on_screen: Vec<eframe::egui::Shape> = self
                    .points
                    .iter_mut()
                    .enumerate()
                    .map(|(i, point)| {
                        let size = Vec2::splat(2.0 * 5.0);

                        let point_in_screen = to_screen.transform_pos(*point);
                        *point = to_screen.from().clamp(*point);
                        let point_in_screen = to_screen.transform_pos(*point);

                        Shape::circle_filled(point_in_screen, 5.0, Color32::from_rgb(255, 255, 255))
                    })
                    .collect();

                let map_size = self.map.get_size();

                let grid_lines: (Vec<egui::Shape>, Vec<egui::Shape>) =
                    self.generate_grid_lines(&to_screen);

                painter.add(Shape::rect_filled(
                    response.rect,
                    Rounding::default(),
                    Color32::from_rgb(122, 122, 122),
                ));
                
                if !points_on_screen.is_empty() {
                    painter.add(points_on_screen);    
                }
                
                painter.set_opacity(0.3);
                painter.add(grid_lines.0);
                painter.add(grid_lines.1);
                
            });
        });
    }
}
