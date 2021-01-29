use std::collections::HashMap;

use super::*;

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
fn return_control_phases() {
    let phases = vec![
        "A".to_string(),
        "B".to_string(),
        "C".to_string(),
        "D".to_string(),
        "E".to_string(),
    ];
    let first_cut_phases = vec!["C".to_string(), "D".to_string(), "E".to_string()];
    let second_cut_phases = vec!["E".to_string()];
    assert_eq!(
        ("A".to_string(), "B".to_string(), first_cut_phases.clone()),
        get_control_phases(phases)
    );
    assert_eq!(
        ("C".to_string(), "D".to_string(), second_cut_phases),
        get_control_phases(first_cut_phases)
    );
}

#[test]
#[should_panic]
fn panics_when_no_two_control_words() {
    get_control_phases(vec!["A".to_string()]);
}

#[test]
fn returns_parsed_config() {
    let file_content = "A;Q\nB;D\nF;G\n".to_string();
    let ((wakeup_word, sleep_word, phases), phases_and_commands) = parse_config(file_content);
    assert_eq!(wakeup_word, "A".to_string());
    assert_eq!(sleep_word, "B".to_string());
    assert_eq!(phases, vec!["F".to_string()]);
    assert_eq!(
        phases_and_commands,
        [
            ("A".to_string(), "Q".to_string()),
            ("B".to_string(), "D".to_string()),
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
