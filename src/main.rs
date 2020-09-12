use std::sync;

mod julius;
mod parser;
mod worker;

fn main() {
    let (julius_sender, julius_receiver) = sync::mpsc::channel();
    let (worker_sender, worker_reciever) = sync::mpsc::channel();

    julius::listen_and_send(julius_sender, "j_polski.jconf".to_string());
    worker::listen_and_do(worker_reciever);
    parser::parse_commands(julius_receiver, worker_sender);
}
