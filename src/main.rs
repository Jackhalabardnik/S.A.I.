use std::collections::HashMap;
use std::env;
use std::fs;
use std::sync;

mod julius;
mod parser;
mod worker;

use parser::Parser;

fn get_control_phases(mut phases: Vec<String>) -> (String, String, Vec<String>) {
    if phases.len() <= 2 {
        panic!("SAI config has to contain control words");
    }
    let wakeup_word = phases.remove(0);
    let sleep_word = phases.remove(0);
    (wakeup_word, sleep_word, phases)
}

fn get_command_collections(lines: std::str::Lines) -> (Vec<String>, HashMap<String, String>) {
    let mut vector: Vec<String> = Vec::new();
    let mut hashmap: HashMap<String, String> = HashMap::new();
    lines.for_each(|line| {
        let words: Vec<&str> = line.split(";").collect();
        let phase = words.get(0).expect("Cant get phase").to_string();
        let command = words.get(1).expect("Cant get command").to_string();
        if phase.len() == 0 {
            panic!("Phase cannot be empty!");
        }
        if command.len() == 0 {
            panic!("Command word cannot be empty!");
        }

        if let Some(_) = hashmap.insert(phase.clone(), command) {
            panic!("Phases cannot repeat");
        }

        vector.push(phase);
    });
    (vector, hashmap)
}

fn read_config_file(filename: String) -> String {
    let file_content = fs::read(filename).expect("Cant find config file...");
    String::from_utf8_lossy(&file_content).to_string()
}

fn parse_config(file_content: String) -> ((String, String, Vec<String>), HashMap<String, String>) {
    let lines = file_content.lines();
    let (all_phases, phases_and_commands) = get_command_collections(lines);
    (get_control_phases(all_phases), phases_and_commands)
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("SAI run: SAI [julius config file] [SAI config file]");
    }

    let ((wakeup_word, sleep_word, phases), phases_and_commands) =
        parse_config(read_config_file(args.pop().unwrap()));

    let (julius_sender, julius_receiver) = sync::mpsc::channel();
    let (worker_sender, worker_reciever) = sync::mpsc::channel();
    let parser = Parser::new(wakeup_word, sleep_word, phases);

    julius::listen_and_send(julius_sender, args.pop().unwrap());
    worker::listen_and_do(worker_reciever, phases_and_commands);
    parser.parse_commands(julius_receiver, worker_sender);
}

#[cfg(test)]
mod test;
