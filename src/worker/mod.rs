use std::collections::HashMap;
use std::process::Command;
use std::sync;
use std::thread;

pub fn listen_and_do(receiver: sync::mpsc::Receiver<String>, commands: HashMap<String, String>) {
    println!("Worker acitve!");
    thread::spawn(move || loop {
        match receiver.recv() {
            Ok(result) => {
                execute_command(commands.get(&result).unwrap().to_string());
            }
            Err(_) => {}
        }
    });
}

fn execute_command(command: String) {
    let mut command_parts = command.split_whitespace();
    let command_name = command_parts.next().unwrap();
    let mut execution = Command::new(command_name);
    for command_part in command_parts {
        execution.arg(command_part);
    }
    match execution.spawn() {
        Ok(_) => {}
        Err(_) => println!("Problem during trying to execute command: {}", command),
    }
}
