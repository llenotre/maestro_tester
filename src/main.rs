use std::fs;
use std::process::Command;
use std::process;
use std::thread;

use serde::Deserialize;

use test_machine::TestMachine;

mod gpio;
mod test_machine;

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
    let sources_dir = fs::canonicalize(dir);
    if sources_dir.is_err() {
        return Err(());
    }
    let sources_dir = sources_dir.unwrap();
    fs::remove_dir_all(sources_dir);

    let mut command = Command::new("git");
    command.arg("clone").arg(url).arg(dir);

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
fn compile(config: &Config, dir: &String) -> Result<(), ()> {
    let sources_dir = fs::canonicalize(dir);
    if sources_dir.is_err() {
        return Err(());
    }
    let sources_dir = sources_dir.unwrap();

    let mut command = Command::new(config.compilation.command.clone());
    command.current_dir(sources_dir);
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

    println!("Getting source code...");
    if clone_repo(&config.repository, &String::from("sources")).is_err() {
        eprintln!("Failed to clone repository!");
        process::exit(1);
    }
    // TODO Set checkout commit hash (get as argument)

    println!("Compiling...");
    if compile(&config, &String::from("sources")).is_err() {
        eprintln!("Failed to clone repository!");
        process::exit(1);
    }

    // TODO Copy output to PXE directory

    println!("Running tests...");
    for m in config.test_machines.clone() {
        let t = thread::spawn(move || {
            println!("Booting test machine `{}` and running kernel...", m.get_name());
            if m.boot().is_err() {
                eprintln!("Failed to boot machine `{}`!", m.get_name());
                // TODO Continue running but end program with status `1`
            }
            println!("Testing ended on machine `{}`", m.get_name());
            if m.shutdown().is_err() {
                eprintln!("Failed to shutdown machine `{}`!", m.get_name());
                // TODO Continue running but end program with status `1`
            }
        });
        if t.join().is_err() {
            eprintln!("An error was raised while run the tests!");
        }
    }

    // TODO Loop on every machines to ensure they are all down

    // TODO Print results
    println!("Done!");
}
