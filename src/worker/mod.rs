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
    let splitted_command = split_command_into_parts(&command);
    let command_name = splitted_command.first().unwrap();
    let mut execution = Command::new(command_name);
    for command_part in &splitted_command[1..] {
        execution.arg(command_part);
    }
    match execution.spawn() {
        Ok(_) => {}
        Err(_) => println!("Problem during trying to execute command: {}", command),
    }
}

fn split_command_into_parts(command: &String) -> Vec<String> {
    let mut splitted_command = split_at_char_and_append_it(&command, '`');
    let mut command_parts: Vec<String> = Vec::new();
    for i in 0..splitted_command.len() {
        if i % 2 == 0 {
            splitted_command[i] = splitted_command[i].trim_start_matches("\" ").to_string();
            splitted_command[i] = splitted_command[i].trim_end_matches(" \"").to_string();

            let single_command_parts = splitted_command[i].split_whitespace();

            for single_command_part in single_command_parts {
                command_parts.push(single_command_part.to_string());
            }
        } else {
            splitted_command[i].insert(0, '\"');
            let lenght = splitted_command[i].len();
            splitted_command[i].insert(lenght, '\"');
            command_parts.push(splitted_command[i].clone());
        }
    }
    command_parts
}

fn split_at_char_and_append_it(string: &String, character: char) -> Vec<String> {
    let splitted = string.split(character.clone()).collect::<Vec<&str>>();

    let mut splitted_strings = Vec::new();

    for splitted_element in splitted {
        splitted_strings.push(splitted_element.to_string());
    }

    for i in 1..splitted_strings.len() {
        if i % 2 == 1 {
            splitted_strings[i].insert(0, character);
            let length = splitted_strings[i].len();
            splitted_strings[i].insert(length, character);
        }
    }
    if splitted_strings.last().unwrap() == "" {
        splitted_strings.pop();
    }
    splitted_strings
}

#[cfg(test)]
mod test;
