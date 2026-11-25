use antex::{StyledText, Text};

#[test]
fn _0001() {
    assert_eq!(
        "Hello worlds!",
        Text::default()
            .s("Hello ")
            .plural("world", 0)
            .s('!')
            .to_string()
    );
}

#[test]
fn _0002() {
    assert_eq!(
        "Hello world!",
        Text::default()
            .s("Hello ")
            .plural("world", 1)
            .s('!')
            .to_string()
    );
}

#[test]
fn _0003() {
    assert_eq!(
        "Hello worlds!",
        Text::default()
            .s("Hello ")
            .plural("world", 2)
            .s('!')
            .to_string()
    );
}
