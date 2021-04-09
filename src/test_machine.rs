use serde::Deserialize;

/// Structure representing a test machine on which the kernel will run.
#[derive(Clone, Deserialize)]
pub struct TestMachine {
    /// The machine's name.
    name: String,
    /// The machine's ip address.
    ip: String,
    /// The machine's MAC address.
    mac: String,

    /// The machine's relay's GPIO number.
    gpio: u32,

    /// The delay between switching the relay and sending the magic packet in milliseconds.
    boot_delay: usize,
    /// The booting timeout, killing the power input if no response from the test machine is
    /// received.
    boot_timeout: usize,
}

impl TestMachine {
    /// Returns the name of the machine.
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Boots the test machine.
    pub fn boot(&self) {
        // TODO
    }

    /// Runs the tests on the machine.
    pub fn run(&self) {
        // TODO
    }

    /// Shutdowns the machine.
    pub fn shutdown(&self) {
        // TODO
    }
}
