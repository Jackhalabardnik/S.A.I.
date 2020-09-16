use std::collections::HashMap;

use super::*;

#[test]
fn returns_control_words() {
    assert_eq!(
        ("A".to_string(), "B".to_string()),
        get_control_words("A;B".to_string())
    );
}

#[test]
#[should_panic]
fn panics_when_not_have_control_words() {
    get_control_words("AB".to_string());
}

#[test]
#[should_panic]
fn panics_when_have_empty_control_words() {
    get_control_words(";".to_string());
}

#[test]
#[should_panic]
fn panics_when_have_only_one_control_word() {
    get_control_words("A;".to_string());
}

#[test]
fn returns_pair_of_command_collections() {
    let file_content = "A;B\nC;D\nE;B\n".to_string();
    let vector = vec!["A".to_string(), "C".to_string(), "E".to_string()];
    let hash_map: HashMap<String, String> = [
        ("A".to_string(), "B".to_string()),
        ("C".to_string(), "D".to_string()),
        ("E".to_string(), "B".to_string()),
    ]
    .iter()
    .cloned()
    .collect();
    let (result_vector, result_hashmap) = get_command_collections(file_content.lines());
    assert_eq!(result_vector, vector);
    assert_eq!(result_hashmap, hash_map);
}

#[test]
#[should_panic]
fn panics_when_no_phase_word() {
    let file_content = "A;B\n;D\nE;B\n".to_string();
    get_command_collections(file_content.lines());
}

#[test]
#[should_panic]
fn panics_when_no_command_word() {
    let file_content = "A;B\n;D\nE;B\n".to_string();
    get_command_collections(file_content.lines());
}

#[test]
#[should_panic]
fn panics_when_invalid_line() {
    let file_content = "A;B\nAD\nE;B\n".to_string();
    get_command_collections(file_content.lines());
}

#[test]
#[should_panic]
fn panics_when_empty_line() {
    let file_content = "A;B\n\nE;B\n".to_string();
    get_command_collections(file_content.lines());
}

#[test]
#[should_panic]
fn panics_when_phases_repeats_line() {
    let file_content = "A;B\nC;D\nA;F\n".to_string();
    get_command_collections(file_content.lines());
}

#[test]
fn returns_parsed_config() {
    let file_content = "A;B\nC;D\nF;G\n".to_string();
    let ((invoking_word, sleep_word), (phases, phases_and_commands)) = parse_config(file_content);
    assert_eq!(invoking_word, "A".to_string());
    assert_eq!(sleep_word, "B".to_string());
    assert_eq!(phases, vec!["C".to_string(), "F".to_string()]);
    assert_eq!(
        phases_and_commands,
        [
            ("C".to_string(), "D".to_string()),
            ("F".to_string(), "G".to_string()),
        ]
        .iter()
        .cloned()
        .collect()
    );
}

#[test]
#[should_panic]
fn panics_with_empty_config() {
    let file_content = "".to_string();
    parse_config(file_content);
}
