use crate::map::{self, Map};
use core::num;
use eframe::egui;
use eframe::emath::RectTransform;
use eframe::{
    egui::{Color32, Frame, Pos2, Rect, Rounding, Sense, Shape, Vec2},
    egui_glow::painter,
    emath,
};
use rusttype::Point;
use std::{borrow::Borrow, default};

pub struct App {
    map: Map,
    map_middle_point_coord: egui::Pos2,
    points: Vec<Pos2>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            map: Map::new(1.0, 500, 500),
            map_middle_point_coord: egui::pos2(500.0 / 2.0, 500.0 / 2.0),
            points: Vec::default(), // Holds coordinates of the points
        }
    }
}

impl App {
    fn setup_map(&mut self, map: Map) {
        self.map = map;
        self.map_middle_point_coord =
            egui::pos2(self.map.get_size().x / 2.0, self.map.get_size().y / 2.0);
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
        
        while i != num_lines{

          let start_point: Pos2 = Pos2 { x: x_begin, y: y_begin };
          let start_point = tf.transform_pos(start_point);
          let end_point: Pos2 = Pos2 {x: x_begin, y: y_end};
          let end_point = tf.transform_pos(end_point);
          horizontal_lines.push(egui::Shape::line_segment([start_point, end_point], egui::Stroke::new(0.25, egui::Color32::from_rgb(255, 255, 255))));
          
          let start_point: Pos2 = Pos2 { x: y_begin, y: x_begin };
          let start_point = tf.transform_pos(start_point);
          let end_point: Pos2 = Pos2 {x: y_end, y: x_begin};
          let end_point = tf.transform_pos(end_point);
          
          vertical_lines.push(egui::Shape::line_segment([start_point, end_point], egui::Stroke::new(0.25, egui::Color32::from_rgb(255, 255, 255))));
          i += 1;
          x_begin += 10.0;
        }

        (horizontal_lines, vertical_lines)
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // Add central panel
        // The central panel consists of a drawn map ...

        eframe::egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            eframe::egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            Frame::canvas(ui.style()).show(ui, |ui| {
                let (response, painter) = ui.allocate_painter(self.map.get_size(), Sense::hover());
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
                
                let grid_lines: (Vec<egui::Shape>, Vec<egui::Shape>) = self.generate_grid_lines(&to_screen);  
                
                painter.add(Shape::rect_filled(
                    response.rect,
                    Rounding::default(),
                    Color32::from_rgb(122, 122, 122),
                ));
                painter.add(grid_lines.0);
                painter.add(grid_lines.1);
                painter.add(points_on_screen);
            });
        });
    }
}
