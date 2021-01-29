use super::*;

#[test]
fn test_contains_wakeup_word_word() {
    let parser = Parser::new(
        "COMPUTER".to_string(),
        "STOP".to_string(),
        vec!["A".to_string()],
    );
    assert_eq!(parser.contains_wakeup_word_word(&"WORD".to_string()), false);
    assert_eq!(
        parser.contains_wakeup_word_word(&"COMPUTER".to_string()),
        true
    );
    assert_eq!(
        parser.contains_wakeup_word_word(&"COMPUTER RUN".to_string()),
        true
    );
}

#[test]
fn test_contains_sleep_word() {
    let parser = Parser::new(
        "COMPUTER".to_string(),
        "STOP".to_string(),
        vec!["A".to_string()],
    );
    assert_eq!(parser.contains_sleep_word(&"WORD".to_string()), false);
    assert_eq!(parser.contains_sleep_word(&"STOP".to_string()), true);
    assert_eq!(parser.contains_sleep_word(&"STOP RUN".to_string()), true);
}

#[test]
fn test_simple_sentence_contains_phase() {
    let parser = Parser::new(
        "COMPUTER".to_string(),
        "STOP".to_string(),
        vec!["AAA".to_string(), "BB".to_string()],
    );
    assert_eq!(
        parser.sentence_contains_phase(&"AAA".to_string()),
        Some("AAA".to_string())
    );
    assert_eq!(parser.sentence_contains_phase(&"AA".to_string()), None);
    assert_eq!(parser.sentence_contains_phase(&"BBB".to_string()), None);
    assert_eq!(
        parser.sentence_contains_phase(&"BB BBB".to_string()),
        Some("BB".to_string())
    );
}

#[test]
fn test_complex_sentence_contains_phase() {
    let parser = Parser::new(
        "COMPUTER".to_string(),
        "STOP".to_string(),
        vec!["AAA".to_string(), "EEE FFF".to_string()],
    );
    assert_eq!(
        parser.sentence_contains_phase(&"AA BB CC".to_string()),
        None
    );
    assert_eq!(
        parser.sentence_contains_phase(&"EEE er FFF".to_string()),
        Some("EEE FFF".to_string())
    );

    assert_eq!(
        parser.sentence_contains_phase(&"AA EEE erDD FFF DD".to_string()),
        Some("EEE FFF".to_string())
    );
    assert_eq!(parser.sentence_contains_phase(&"EEEFFFF".to_string()), None);
    assert_eq!(
        parser.sentence_contains_phase(&"FFF er EEE".to_string()),
        None
    );
}

#[test]
fn test_contains_quit_message() {
    let parser = Parser::new(
        "COMPUTER".to_string(),
        "STOP".to_string(),
        vec!["AAA".to_string(), "EEE FFF".to_string()],
    );
    assert_eq!(parser.contains_quit_message(&"00000".to_string()), true);
    assert_eq!(parser.contains_quit_message(&"000000".to_string()), false);
    assert_eq!(parser.contains_quit_message(&"asd".to_string()), false);
}
