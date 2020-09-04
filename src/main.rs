use std::sync;

mod julius;

use julius::*;

fn main() {
    let (jw_sender, jw_reciver) = sync::mpsc::channel();

    listen_and_send(jw_sender, "my_conf.jconf".to_string());

    loop {
        let result = jw_reciver.recv().unwrap();
        println!("{}", result);
    }
}
