use std::io::Write;
use std::time::Duration;

use log::error;
use serialport::SerialPort;

use crate::matrix::{Matrix, MATRIX_WIDTH};

pub struct LEDController {
    port: Box<dyn SerialPort>,
}

const LED_PORT_PATH: &'static str = "/dev/ttyACM0";

impl LEDController {
    pub fn init() -> Self {
        let port = serialport::new(LED_PORT_PATH, 115_200)
            .timeout(Duration::from_secs(3))
            .open()
            .unwrap_or_else(|err| {
                error!("Failed to open LED Matrix Serial Port {}", err);
                std::process::exit(1)
            });

        Self { port }
    }

    pub fn draw_matrix(&mut self, matrix: Matrix) {
        // todo maybe implement iterator for Matrix?
        for col_index in 0..MATRIX_WIDTH {
            let column = matrix.get_col(col_index);
            self.send_col(&column, col_index)
        }
        self.flush_cols()
    }

    fn send_col(&mut self, buffer: &[u8], col: usize) {
        let mut command = [0u8; 38];

        command[0..4].copy_from_slice(&[0x32, 0xAC, 0x07, col as u8]);
        command[4..38].copy_from_slice(buffer);

        self.port.write_all(&command).unwrap()
    }

    fn flush_cols(&mut self) {
        let command = [0x32, 0xAC, 0x08];
        self.port.write_all(&command).unwrap()
    }
}
