use std::net::UdpSocket;
use std::thread;
use std::time;

use serde::Deserialize;

use crate::gpio::GPIO;

/// The number of magic packets to send (to ensure it is received).
const WOL_COUNT: usize = 3;
/// The delay between each magic packet in milliseconds.
const WOL_DELAY: u64 = 100;

/// Structure representing a test machine on which the kernel will run.
#[derive(Clone, Deserialize)]
pub struct TestMachine {
    /// The machine's name.
    name: String,
    /// The address on which to broadcast the Wake On LAN packet.
    broadcast_address: String,
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

// TODO Unit test
/// Parses the given MAC address.
fn parse_mac(mac: &String) -> Result<[u8; 6], ()> {
    let mut i = 0;
    let mut addr: [u8; 6] = [0; 6];

    for s in mac.split(":") {
        if i >= 6 || s.len() != 2 {
            return Err(());
        }
        if let Ok(val) = u8::from_str_radix(s, 16) {
            addr[i] = val;
        } else {
            return Err(());
        }

        i += 1;
    }

    Ok(addr)
}

impl TestMachine {
    /// Returns the name of the machine.
    pub fn get_name(&self) -> &String {
        &self.name
    }

    fn send_wol(&self) -> Result<(), ()> {
        let mac = parse_mac(&self.mac)?;
        let mut buf: [u8; 102] = [0xff; 102];
        for i in 1..17 {
            for j in 0..6 {
                buf[i * 6 + j] = mac[j];
            }
        }

        let sock = UdpSocket::bind("0.0.0.0:3000");
        if let Err(e) = sock {
            eprintln!("Could not create socket: {}", e);
            return Err(());
        }
        let sock = sock.unwrap();
        sock.set_broadcast(true);

        if let Err(e) = sock.send_to(&buf, self.broadcast_address.clone() + ":9") {
            eprintln!("Could not send WOL: {}", e);
            return Err(());
        }

        Ok(())
    }

    /// Boots the test machine.
    pub fn boot(&self) -> Result<(), ()> {
        let gpio = GPIO::from_id(self.gpio);
        gpio.set_output(false)?;

        thread::sleep(time::Duration::from_millis(self.boot_delay as _));

        for i in 0..WOL_COUNT {
            self.send_wol()?;
            if i < WOL_COUNT - 1 {
                thread::sleep(time::Duration::from_millis(WOL_DELAY));
            }
        }

        thread::sleep(time::Duration::from_millis(self.boot_timeout as _)); // TODO
        // TODO Wait for boot_timeout ms (unless results are received before)
        // TODO Read results from serial

        Ok(())
    }

    /// Shutdowns the machine.
    pub fn shutdown(&self) -> Result<(), ()> {
        let gpio = GPIO::from_id(self.gpio);
        gpio.set_output(true)
    }
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn parse_mac0() {
        assert!(parse_mac(&String::from("")).is_err());
	}

	#[test]
	fn parse_mac1() {
        assert!(parse_mac(&String::from(":::::")).is_err());
	}

	#[test]
	fn parse_mac2() {
        assert!(parse_mac(&String::from("0:0:0:0:0:0")).is_err());
	}

	#[test]
	fn parse_mac3() {
        assert!(parse_mac(&String::from("00:00:00:00:00:00")).is_ok());
	}

	#[test]
	fn parse_mac4() {
        assert!(parse_mac(&String::from("00:00:00:00:00:000")).is_err());
	}

	#[test]
	fn parse_mac5() {
        assert!(parse_mac(&String::from("01:23:45:67:89:ab")).is_ok());
	}
}
