use std::collections::HashMap;
use std::fs;
use std::sync;

mod julius;
mod parser;
mod worker;

use parser::Parser;

fn get_control_words(string: String) -> (String, String) {
    let words: Vec<&str> = string.split(";").collect();
    let invoke_word = words.get(0).expect("Cant get invoking word").to_string();
    let stop_word = words.get(1).expect("Cant get sleep word").to_string();
    if invoke_word.len() == 0 {
        panic!("Invoking word cannot be empty!");
    }
    if stop_word.len() == 0 {
        panic!("Stop word cannot be empty!");
    }
    (invoke_word, stop_word)
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

fn parse_config(file_content: String) {
    let mut lines = file_content.lines();
    let important_words = lines.next().unwrap();
    println!("Important words: {}", important_words);
    lines.for_each(|line| println!("{}", line));
}

fn main() {
    parse_config(read_config_file("sai_config".to_string()));
    let (julius_sender, julius_receiver) = sync::mpsc::channel();
    let (worker_sender, worker_reciever) = sync::mpsc::channel();
    let parser = Parser::new("KOMPUTER".to_string());

    julius::listen_and_send(julius_sender, "j_polski.jconf".to_string());
    worker::listen_and_do(worker_reciever);
    parser.parse_commands(julius_receiver, worker_sender);
}

#[cfg(test)]
mod test;
