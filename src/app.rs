use std::sync::mpsc;
use std::thread;

use crate::ip::{self, IpInfo};

pub enum AppState {
    Loading { frame: u64 },
    Loaded { ip_info: IpInfo },
    Error { message: String },
}

pub struct App {
    pub state: AppState,
    pub should_quit: bool,
    receiver: Option<mpsc::Receiver<Result<IpInfo, String>>>,
}

impl App {
    pub fn new() -> Self {
        let mut app = App {
            state: AppState::Loading { frame: 0 },
            should_quit: false,
            receiver: None,
        };
        app.start_fetch();
        app
    }

    pub fn start_fetch(&mut self) {
        self.state = AppState::Loading { frame: 0 };
        let (tx, rx) = mpsc::channel();
        self.receiver = Some(rx);
        thread::spawn(move || {
            let result = ip::fetch_ip_info().map_err(|e| e.to_string());
            let _ = tx.send(result);
        });
    }

    pub fn tick(&mut self) {
        // Advance loading animation
        if let AppState::Loading { ref mut frame } = self.state {
            *frame += 1;
        }

        // Check for fetch result
        if let Some(ref rx) = self.receiver {
            if let Ok(result) = rx.try_recv() {
                self.receiver = None;
                match result {
                    Ok(info) => self.state = AppState::Loaded { ip_info: info },
                    Err(msg) => self.state = AppState::Error { message: msg },
                }
            }
        }
    }

    pub fn on_key(&mut self, key: char) {
        match key {
            'q' => self.should_quit = true,
            'r' => self.start_fetch(),
            _ => {}
        }
    }
}
