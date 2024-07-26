use eframe::egui::{accesskit::Point, Pos2};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    clone,
    fmt::Result,
    net::UdpSocket,
    option, result,
    sync::{mpsc, Arc},
    thread, vec,
};

#[derive(Serialize, Deserialize)]
struct Point2D {
    x: f64,
    y: f64,
}

pub struct UDP_Worker {
    addr: String,
    sender: Option<mpsc::Sender<Vec<Pos2>>>,
    keep_thread_alive: Arc<()>,
    thread_handle: Option<thread::JoinHandle<()>>,
    connected: bool,
}

impl UDP_Worker {
    pub fn new(sender: mpsc::Sender<Vec<Pos2>>) -> UDP_Worker {
        let mut connected: bool = false;
        let keepalive = Arc::default();
        UDP_Worker {
            addr: String::default(),
            sender: Some(sender),
            keep_thread_alive: keepalive,
            thread_handle: None,
            connected: connected,
        }
    }

    pub fn listen(&mut self, address: String) -> result::Result<(), ()> {
        let socket = UdpSocket::bind(address);
        if socket.is_err() {
            return Err(());
        }

        let keepalive_recv = Arc::downgrade(&self.keep_thread_alive);
        let mut received_points: Vec<Pos2> = Vec::default();
        // UDP Listening Thread
        let socket = socket.unwrap();
        let sender = self.sender.take().unwrap();
        let join_thread_handle = thread::spawn(move || {
            let mut buff = [0u8; 1500];

            while keepalive_recv.upgrade().is_some() {
                match socket.recv_from(&mut buff) {
                    Ok((dtgrm_size, src)) => {
                        let points: Vec<Point2D> = serde_json::from_slice(&buff).unwrap();
                        if !points.is_empty() {
                            let points_on_screen =
                                points.iter().enumerate().map(|(i, value)| {
                                  Pos2 { x: (*value).x as f32, y: (*value).y as f32 }
                                }).collect();
                            let send_res = sender.send(points_on_screen);
                            match send_res {
                              Ok(()) => {
                                received_points.clear();
                              }
                              Err(e) => {
                                received_points = e.0;
                              }
                            }
                        }
                    }
                    Err(e) => {
                      if !received_points.is_empty() {
                        let send_res = sender.send(received_points.clone());
                            match send_res {
                              Ok(()) => {
                                received_points.clear();
                              }
                              Err(e) => {
                                received_points = e.0;
                              }
                            }
                      }
                    }
                }
            }
        });

        self.thread_handle = Some(join_thread_handle);

        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.connected.clone()
    }
}

impl Drop for UDP_Worker {
    fn drop(&mut self) {
        if let Some(handle) = self.thread_handle.take() {
            handle.join();
        }
    }
}
