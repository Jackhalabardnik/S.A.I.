use std::sync;

mod julius;

use julius::*;

use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};

fn main() {
    let (jw_sender, jw_reciver) = sync::mpsc::channel();

    let jw = JuliusWorker::create(jw_sender, "my_conf.jconf".to_string());

    loop {
        let result = jw_reciver.recv().unwrap();
        println!("{}", result);
    }
}
