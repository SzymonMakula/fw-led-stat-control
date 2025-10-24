use std::time::{Instant, UNIX_EPOCH};

use battery::Manager;
use log::error;
use sysinfo::{
    CpuRefreshKind, MemoryRefreshKind, MINIMUM_CPU_UPDATE_INTERVAL, RefreshKind, System,
};

pub struct SystemStatMonitor {
    system_api: System,
    last_refresh: Instant,
}

impl SystemStatMonitor {
    pub fn new() -> Self {
        Self {
            system_api: System::new(),
            last_refresh: Instant::now(),
        }
    }

    pub fn get_memory_usage(&mut self) -> f32 {
        self.check_refresh();

        let system = &self.system_api;
        let total_memory = system.total_memory();
        let used_memory = system.used_memory();

        let ratio = used_memory as f32 / total_memory as f32;

        ratio
    }

    pub fn get_global_cpu_usage(&mut self) -> f32 {
        self.check_refresh();
        self.system_api.global_cpu_usage()
    }

    pub fn get_battery_state_of_charge() -> f32 {
        let mut batteries = Manager::new()
            .unwrap_or_else(|err| {
                error!(target: "SystemStatMonitor", "Failed to instantiate Battery Manager {}", err);
                std::process::exit(1)
            })
            .batteries()
            .unwrap_or_else(|err| {
                error!(target: "SystemStatMonitor", "Failed to construct Batteries iterator {}", err);
                std::process::exit(1)
            });
        let first_battery = batteries
            .next()
            .unwrap_or_else(|| {
                error!(target: "SystemStatMonitor", "No batteries found in the 'Batteries' iterator");
                std::process::exit(1)
            })
            .unwrap_or_else(|err| {
                error!(target: "SystemStatMonitor", "Failed to get battery information {}", err);
                std::process::exit(1)
            });

        first_battery.state_of_charge().value
    }

    pub fn get_epoch_time() -> u64 {
        let time = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        time
    }

    fn check_refresh(&mut self) {
        if Instant::now().duration_since(self.last_refresh) > MINIMUM_CPU_UPDATE_INTERVAL {
            self.system_api.refresh_specifics(
                RefreshKind::nothing()
                    .with_cpu(CpuRefreshKind::everything())
                    .with_memory(MemoryRefreshKind::everything()),
            );
            self.last_refresh = Instant::now()
        }
    }
}
