use std::sync;
use std::thread;

pub fn listen_and_do(receiver: sync::mpsc::Receiver<String>) {
    println!("Worker acitve!");
    thread::spawn(move || loop {
        match receiver.recv() {
            Ok(result) => {
                println!("{}", result);
            }
            Err(_) => {}
        }
    });
}
