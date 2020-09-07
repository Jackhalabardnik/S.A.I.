use std::sync;

pub fn parse_commands(
    command_input: sync::mpsc::Receiver<String>,
    _command_output: sync::mpsc::Sender<String>,
) {
    let mut command = String::new();
    let mut is_new_command : bool;
    loop {
        match command_input.try_recv() {
            Ok(message) => {
                command = message;
                is_new_command = true;
            }
            Err(_) => {
                is_new_command = false;
            }
        }

        if is_new_command {
            println!("{}", &command);
        }
    }
}

#[cfg(test)]
mod test;
