use std::sync;

mod julius;
mod parser;

fn main() {
    let (julius_sender, julius_receiver) = sync::mpsc::channel();
    let (worker_sender, _worker_reciever) = sync::mpsc::channel();

    julius::listen_and_send(julius_sender, "my_conf.jconf".to_string());
    parser::parse_commands(julius_receiver, worker_sender);
}
