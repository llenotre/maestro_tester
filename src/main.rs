use std::fs;
use std::process::Command;
use std::process;

use serde::Deserialize;

/// Structure representing a test machine on which the kernel will run.
#[derive(Deserialize)]
struct TestMachine {
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

/// Structure representing an environment variable.
#[derive(Deserialize)]
struct EnvVar {
    /// The name of the variable.
    name: String,
    /// The value of the variable.
    value: String,
}

/// Structure representing the compilation command.
#[derive(Deserialize)]
struct CompilationCommand {
    /// The compilation environment.
    environment: Vec::<EnvVar>,
    /// The command to execute.
    command: String,
    /// The arguments for the command.
    arguments: Vec::<String>,
}

/// Sturcture representing the testing configuration.
#[derive(Deserialize)]
struct Config {
    /// The URL to the kernel's repository.
    repository: String,
    /// The compilation command.
    compilation: CompilationCommand,
    /// The path to the output binary.
    output_binary: String,
    /// The list of test machines.
    test_machines: Vec::<TestMachine>,
}

/// Reads the configuration file.
fn read_config(file: &String) -> Result<Config, ()> {
    if let Ok(content) = fs::read_to_string(file) {
        if let Ok(config) = serde_json::from_str(&content) {
            Ok(config)
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

/// Clones the repository at the given URL `url` into the directory `dir`.
fn clone_repo(url: &String, dir: &String) -> Result<(), ()> {
    let mut command = Command::new("git");
    command.arg(url).arg(dir);

    if let Ok(status) = command.status() {
        if status.success() {
            Ok(())
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

/// Compiles the kernel with the given configuration `config`.
fn compile(config: &Config) -> Result<(), ()> {
    let mut command = Command::new(config.compilation.command.clone());
    for a in &config.compilation.arguments {
        command.arg(a);
    }
    for e in &config.compilation.environment {
        command.env(e.name.clone(), e.value.clone());
    }

    if let Ok(status) = command.status() {
        if status.success() {
            Ok(())
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

fn main() {
    let config = read_config(&String::from("config.json"));
    if config.is_err() {
        eprintln!("Failed to read configuration!");
        process::exit(1);
    }
    let config = config.unwrap();

    if clone_repo(&config.repository, &String::from("sources")).is_err() {
        eprintln!("Failed to clone repository!");
        process::exit(1);
    }
    // TODO Set checkout commit hash (get as argument)

    if compile(&config).is_err() {
        eprintln!("Failed to clone repository!");
        process::exit(1);
    }

    // TODO Run on every test machines
}
