use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};
use std::sync;
use std::thread;

pub struct JuliusWorker {
    _work_thread: thread::JoinHandle<()>,
}

impl JuliusWorker {
    pub fn create(sender: sync::mpsc::Sender<String>, conf_name: String) -> JuliusWorker {
        let work_thread = thread::spawn(move || {
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
                .for_each(|line| sender.send(line).unwrap());
        });
        JuliusWorker {
            _work_thread: work_thread,
        }
    }
}

#[cfg(test)]
mod test;
