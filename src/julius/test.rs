use super::*;

#[test]
fn test_line_has_sentence() {
    assert_eq!(line_has_sentence(&"sentence1: <s>".to_string()), true);
}

#[test]
fn test_line_has_not_sentence() {
    assert_eq!(line_has_sentence(&"pass 1 best:".to_string()), false);
    assert_eq!(line_has_sentence(&"".to_string()), false);
    assert_eq!(line_has_sentence(&"sentence".to_string()), false);
}

#[test]
fn test_trim_simple_julius_debug() {
    assert_eq!(
        trim_julius_debug("sentence1: COMPUTER".to_string()),
        "COMPUTER".to_string()
    );

    assert_eq!(
        trim_julius_debug("sentence1: MOVE".to_string()),
        "MOVE".to_string()
    );

    assert_eq!(
        trim_julius_debug("sentence1: COMPUTER MOVE".to_string()),
        "COMPUTER MOVE".to_string()
    );
}

#[test]
fn test_trim_julius_debug_with_silent_not_inside() {
    assert_eq!(
        trim_julius_debug("sentence1: <s> COMPUTER </s>".to_string()),
        "COMPUTER".to_string()
    );

    assert_eq!(
        trim_julius_debug("sentence1: <s> <s> COMPUTER </s>".to_string()),
        "COMPUTER".to_string()
    );

    assert_eq!(
        trim_julius_debug("sentence1: <s> <s> COMPUTER MOVE </s>".to_string()),
        "COMPUTER MOVE".to_string()
    );
}

#[test]
fn test_trim_julius_debug_with_silent_inside() {
    assert_eq!(
        trim_julius_debug("sentence1: <s> COMPUTER <s> MOVE </s>".to_string()),
        "COMPUTER MOVE".to_string()
    );
}

#[test]
fn test_trim_julius_debug_only_silent() {
    assert_eq!(
        trim_julius_debug("sentence1: <s> </s>".to_string()),
        "".to_string()
    );

    assert_eq!(
        trim_julius_debug("sentence1: <s> <s> </s>".to_string()),
        "".to_string()
    );
}
