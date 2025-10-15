use std::io::Error;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread::sleep;
use std::time::Duration;

use log::info;
use sd_notify::NotifyState;
use signal_hook::consts::{SIGHUP, TERM_SIGNALS};
use signal_hook::flag;
use signal_hook::iterator::exfiltrator::WithOrigin;
use signal_hook::iterator::SignalsInfo;

use crate::controller::{Controller, ControllerMessage};

mod canvas;
mod config;
mod controller;
mod led_controller;
mod matrix;
mod picture;
mod plugin;
mod wasm_module;

fn main() -> Result<(), Error> {
    env_logger::init();

    let (tx, rx) = std::sync::mpsc::channel::<ControllerMessage>();

    let handle = std::thread::spawn(move || {
        let mut controller = Controller::init();
        loop {
            if let Ok(message) = rx.try_recv() {
                match message {
                    ControllerMessage::ReloadConfig => {
                        controller.reload_config();
                        sd_notify::notify(true, &[NotifyState::Ready]).unwrap();
                    }
                    ControllerMessage::Terminate => break,
                }
            }

            controller.schedule_paint();

            sleep(Duration::from_millis(250))
        }
    });

    let term_now = Arc::new(AtomicBool::new(false));

    for sig in TERM_SIGNALS {
        flag::register_conditional_shutdown(*sig, 1, Arc::clone(&term_now))?;
        flag::register(*sig, Arc::clone(&term_now))?;
    }

    let mut sigs = vec![
        SIGHUP, // Reload configs signal
    ];
    sigs.extend(TERM_SIGNALS);
    let mut signals = SignalsInfo::<WithOrigin>::new(&sigs)?;

    sd_notify::notify(true, &[NotifyState::Ready]).unwrap();

    for info in &mut signals {
        match info.signal {
            SIGHUP => {
                info!("Reloading configuration file");
                sd_notify::notify(true, &[NotifyState::Reloading]).unwrap();
                sd_notify::notify(true, &[NotifyState::monotonic_usec_now().unwrap()]).unwrap();

                tx.send(ControllerMessage::ReloadConfig).unwrap();
            }
            _term_sig => {
                eprintln!("Terminating process");
                tx.send(ControllerMessage::Terminate).unwrap();
                break;
            }
        };
        if handle.is_finished() {
            match handle.join() {
                Ok(_) => {
                    unreachable!("Worker thread cannot successfully finish early")
                }
                Err(err) => {
                    eprintln!("Worker thread panicked! {:?}", err);
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}
