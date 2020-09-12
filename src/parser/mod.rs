use std::sync;

pub fn parse_commands(
    command_input: sync::mpsc::Receiver<String>,
    command_output: sync::mpsc::Sender<String>,
) {
    println!("Parser acitve!");
    let mut is_new_command: bool;
    let mut is_active = false;
    let mut time = std::time::Instant::now();
    let standard_duration = std::time::Duration::new(10, 0);
    loop {
        let mut command = String::new();
        match command_input.try_recv() {
            Ok(message) => {
                command = message;
                is_new_command = true;
            }
            Err(_) => {
                is_new_command = false;
            }
        }

        if is_active {
            if is_new_command {
                if contains_special_word(&command) {
                    time = std::time::Instant::now();
                    println!("SIA hears you");
                } else {
                    match command_output.send(command) {
                        Err(problem) => {
                            println!("{}", problem);
                        }
                        Ok(_) => {}
                    }
                }
            }
            if time.elapsed() > standard_duration {
                is_active = false;
                println!("SIA falls asleep...");
            }
        } else if is_new_command {
            if contains_special_word(&command) {
                is_active = true;
                println!("SIA is active!");
                time = std::time::Instant::now();
            }
        }
    }
}

fn contains_special_word(word: &String) -> bool {
    word.contains("KOMPUTER")
}

#[cfg(test)]
mod test;
