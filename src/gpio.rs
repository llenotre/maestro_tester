use std::fs;
use std::path::Path;

/// Structure representing a General Purpose Input Output.
pub struct GPIO {
    /// The ID of the GPIO.
    id: u32,
}

impl GPIO {
    /// Returns an instance for GPIO id `id`.
    pub fn from_id(id: u32) -> Self {
        Self {
            id: id,
        }
    }

    /// Tells whether the GPIO is ready or not. If not, the function `prepare` has to be called.
    pub fn is_ready(&self) -> bool {
        let path = "/sys/class/gpio/gpio".to_owned() + &self.id.to_string();
        Path::new(&path).exists()
    }

    /// Prepares the GPIO.
    pub fn prepare(&self) -> Result<(), ()> {
        if self.is_ready() {
            return Ok(());
        }

        if let Err(e) = fs::write("/sys/class/gpio/export", &self.id.to_string()) {
            eprintln!("Error while exporting GPIO {}: {}", self.id, e);
            return Err(());
        }

        if let Err(e) = fs::write("/sys/class/gpio/gpio".to_owned() + &self.id.to_string() + "/direction", "out") {
            eprintln!("Error while setting GPIO {} direction: {}", self.id, e);
            return Err(());
        }


        Ok(())
    }

    /// Sets the GPIO output to state `state`.
    pub fn set_output(&self, state: bool) -> Result<(), ()> {
        if !self.is_ready() {
            self.prepare()?;
        }

        let path = "/sys/class/gpio/gpio".to_owned() + &self.id.to_string() + "/value";
        let state_str = if state {
            "1"
        } else {
            "0"
        };

        if let Err(e) = fs::write(path, state_str) {
            eprintln!("Error while setting GPIO {} state: {}", self.id, e);
            Err(())
        } else {
            Ok(())
        }
    }
}
