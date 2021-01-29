use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};
use std::sync;
use std::thread;

pub fn listen_and_send(sender: sync::mpsc::Sender<String>, conf_name: String) {
    println!("Julius acitve!");

    let quit_message = "00000".to_string();

    thread::spawn(move || {
        let stdout = Command::new("julius")
            .arg("-C")
            .arg(conf_name)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap()
            .stdout
            .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))
            .unwrap();

        let reader = BufReader::new(stdout);

        reader
            .lines()
            .filter_map(|line| line.ok())
            .for_each(|line| {
                if contains_julius_critical_error(&line) {
                    //just to say parser to quit and this will kill all threads
                    println!("Julius is dead");
                    sender.send(quit_message.clone()).unwrap()
                } else if line_has_sentence(&line) {
                    let message = trim_julius_debug(line);
                    if message.len() > 0 {
                        //julius can record loud air blow: "sentence1: <s> </s>"
                        sender.send(message).unwrap()
                    }
                }
            });
    });
}

fn line_has_sentence(line: &String) -> bool {
    line.contains("sentence1:")
}

fn trim_julius_debug(line: String) -> String {
    let mut result = String::new();
    line.trim_start_matches("sentence1: ")
        .to_string()
        .trim_end_matches(" </s>")
        .to_string()
        .trim_start_matches("<s> ")
        .to_string()
        .split_whitespace()
        .for_each(|s| {
            if s != "<s>" {
                result.push_str(&s);
                result.push(' ');
            }
        });

    result.trim_end().to_string()
}

fn contains_julius_critical_error(sentence: &String) -> bool {
    sentence.contains("failed to open jconf file")
}

#[cfg(test)]
mod test;
