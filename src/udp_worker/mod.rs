
use std::{
  clone, fmt::Result, net::UdpSocket, option, result, sync::mpsc, thread, vec
};
use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};

use eframe::egui::Pos2;
use tokio::io::join;

#[derive(Serialize, Deserialize)]
struct Point2D {
  x: f64,
  y: f64
}

pub struct UDP_Worker {
  addr: String,
  sender: Option<mpsc::Sender<Vec<Pos2>>>,
  thread_handle: Option<thread::JoinHandle<()>>,
  connected: bool,
}

impl UDP_Worker {
  pub fn new(sender : mpsc::Sender<Vec<Pos2>>) -> UDP_Worker {
    
    let mut connected: bool = false;
     
    let sender_thread_handle = thread::spawn(|| {
      
      let socket_res = UdpSocket::bind(address_clone);
      let socket: Option<UdpSocket> = match socket_res {
        Ok(socket) => Some(socket),
        Err(error) => None,
      };

      while socket.is_none()  {
        
      }
      loop {

      }
    
    });

    UDP_Worker { 
      addr: String::default(),
      sender: Some(sender),
      thread_handle: Some(sender_thread_handle),
      connected: connected,
    }
  }

  pub fn listen(address: String) -> result::Result<(), ()> {

    
      
    Ok(())
  }

  pub fn is_connected(&self) -> bool{
    self.connected.clone()
  }
  
}

impl Drop for UDP_Worker {
  fn drop(&mut self) {
    let join_handle = self.thread_handle.take();
    if let Some(handle) = self.thread_handle.take() {
      handle.join();
    }
  }
}