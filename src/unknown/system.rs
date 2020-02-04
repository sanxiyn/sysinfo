//
// Sysinfo
//
// Copyright (c) 2015 Guillaume Gomez
//

use sys::component::Component;
use sys::process::*;
use sys::processor::*;
use sys::Disk;
use sys::Networks;
use Pid;
use {RefreshKind, SystemExt};

use std::collections::HashMap;

/// Structs containing system's information.
#[derive(Debug)]
pub struct System {
    processes_list: HashMap<Pid, Process>,
    networks: Networks,
}

impl SystemExt for System {
    fn new_with_specifics(_: RefreshKind) -> System {
        System {
            processes_list: Default::default(),
            networks: Networks::new(),
        }
    }

    fn refresh_memory(&mut self) {}

    fn refresh_cpu(&mut self) {}

    fn refresh_temperatures(&mut self) {}

    fn refresh_processes(&mut self) {}

    fn refresh_process(&mut self, _pid: Pid) -> bool {
        false
    }

    fn refresh_disks(&mut self) {}

    fn refresh_disks_list(&mut self) {}

    fn refresh_network(&mut self) {}

    // COMMON PART
    //
    // Need to be moved into a "common" file to avoid duplication.

    fn get_process_list(&self) -> &HashMap<Pid, Process> {
        &self.processes_list
    }

    fn get_process(&self, _pid: Pid) -> Option<&Process> {
        None
    }

    fn get_networks(&self) -> &Networks {
        &self.networks
    }

    fn get_networks_mut(&mut self) -> &mut Networks {
        &mut self.networks
    }

    fn get_processor_list(&self) -> &[Processor] {
        &[]
    }

    fn get_total_memory(&self) -> u64 {
        0
    }

    fn get_free_memory(&self) -> u64 {
        0
    }

    fn get_used_memory(&self) -> u64 {
        0
    }

    fn get_total_swap(&self) -> u64 {
        0
    }

    fn get_free_swap(&self) -> u64 {
        0
    }

    fn get_used_swap(&self) -> u64 {
        0
    }

    fn get_components_list(&self) -> &[Component] {
        &[]
    }

    fn get_disks(&self) -> &[Disk] {
        &[]
    }

    fn get_uptime(&self) -> u64 {
        0
    }

    fn get_load_average(&self) -> LoadAvg {
        LoadAvg {
            one: 0.,
            five: 0.,
            fifteen: 0.,
        }
    }
}

impl Default for System {
    fn default() -> System {
        System::new()
    }
}
