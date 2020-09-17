use std::sync;

pub struct Parser {
    invoking_word: String,
    sleep_word: String,
    phases: Vec<String>,
    quit_word: String,
}

impl Parser {
    pub fn new(invoking_word: String, sleep_word: String, phases: Vec<String>) -> Parser {
        Parser {
            invoking_word: invoking_word,
            sleep_word: sleep_word,
            phases: phases,
            quit_word: "00000".to_string(),
        }
    }

    pub fn parse_commands(
        &self,
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

            if self.contains_quit_message(&command) {
                panic!("Julius needs proper config file");
            }

            if is_active {
                if is_new_command {
                    if self.contains_invoking_word(&command) {
                        time = std::time::Instant::now();
                        println!("SAI hears you");
                    } else if self.contains_sleep_word(&command) {
                        is_active = false;
                        println!("SAI falls asleep...");
                    } else {
                        match self.sentence_contains_phase(&command) {
                            Some(phase) => match command_output.send(phase) {
                                Err(problem) => {
                                    println!("{}", problem);
                                }
                                Ok(_) => {
                                    println!("SAI got command");
                                    time = std::time::Instant::now();
                                }
                            },
                            None => {}
                        }
                    }
                }
                if time.elapsed() > standard_duration {
                    is_active = false;
                    println!("SAI falls asleep...");
                }
            } else if is_new_command {
                if self.contains_invoking_word(&command) {
                    is_active = true;
                    println!("SAI is active!");
                    time = std::time::Instant::now();
                }
            }
        }
    }

    fn contains_invoking_word(&self, word: &String) -> bool {
        word.contains(self.invoking_word.as_str())
    }

    fn contains_sleep_word(&self, word: &String) -> bool {
        word.contains(self.sleep_word.as_str())
    }

    fn contains_quit_message(&self, sentence: &String) -> bool {
        match sentence_has_word(sentence, self.quit_word.clone()) {
            Some(_) => true,
            None => false,
        }
    }

    fn sentence_contains_phase(&self, sentence: &String) -> Option<String> {
        let phases = self.phases.clone();
        for phase in phases {
            let mut sentence_contains_phase = true;
            let mut cloned_sentence = sentence.clone();
            let subphases = phase.split_whitespace();

            for subphase in subphases {
                match sentence_has_word(&cloned_sentence, subphase.to_string()) {
                    Some(end_of_found_word) => {
                        cloned_sentence.drain(..end_of_found_word);
                    }
                    None => {
                        sentence_contains_phase = false;
                        break;
                    }
                }
            }
            if sentence_contains_phase {
                return Some(phase);
            }
        }
        None
    }
}

fn sentence_has_word(sentence: &String, word: String) -> Option<usize> {
    let (has_space_before, has_space_after);
    match sentence.find(&word) {
        Some(number) => {
            if number == 0 {
                has_space_before = true;
            } else {
                has_space_before = sentence.get((number - 1)..number).unwrap() == " ";
            }

            if number + word.len() == sentence.len() {
                has_space_after = true;
            } else {
                has_space_after = sentence
                    .get((number + word.len())..(number + word.len() + 1))
                    .unwrap()
                    == " ";
            }

            if has_space_before && has_space_after {
                return Some(number + word.len()); // end of found word
            } else {
                None
            }
        }
        None => None,
    }
}

#[cfg(test)]
mod test;
