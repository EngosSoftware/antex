use super::*;
use antex::{ColorMode, StyledText, Text};

#[test]
fn text_add_default_should_work() {
    let a = Text::default().s("Hello");
    let b = Text::default().s(" world!");
    let text = a + b;
    assert_eq!("Hello world!", text.to_string());
    assert_eq!("Hello world!", format!("{text}"));
}

#[test]
fn text_add_off_should_work() {
    let a = Text::new(ColorMode::Off).s("Hello");
    let b = Text::new(ColorMode::Off).s(" world!");
    let text = a + b;
    assert_eq!("Hello world!", text.to_string());
    assert_eq!("Hello world!", format!("{text}"));
}

#[test]
fn text_add_on_should_work() {
    let a = Text::new(ColorMode::On).s("Hello");
    let b = Text::new(ColorMode::On).s(" world!");
    let text = a + b;
    assert_eq!("Hello world!", text.to_string());
    assert_eq!("Hello world!", format!("{text}"));
}

#[test]
fn text_add_on_coloured_should_work() {
    let a = Text::new(ColorMode::On).yellow().s("Hello");
    let b = Text::new(ColorMode::On).green().s(" world!");
    let text = a + b;
    assert_eq!("\x1b[33mHello\x1b[32m world!", text.to_string());
    assert_eq!("\x1b[33mHello\x1b[32m world!", format!("{text}"));
}

#[test]
fn text_add_on_coloured_clear_should_work() {
    let a = Text::new(ColorMode::On).yellow().s("Hello").reset();
    let b = Text::new(ColorMode::On).s(" world!");
    let text = a + b;
    assert_eq!("\x1b[33mHello\x1b[0m world!", text.to_string());
    assert_eq!("\x1b[33mHello\x1b[0m world!", format!("{text}"));
}

#[test]
fn adding_multiple_types_should_work() {
    let hello = never().s("hello");
    let world = never().s("world");
    let exclamation_chr = '!';
    let exclamation_str = "!".to_string();
    let smiley = "☺";

    let greeting = hello.clone() + world.clone();
    assert_eq!("helloworld", greeting.to_string());

    let greeting = hello.clone() + &world;
    assert_eq!("helloworld", greeting.to_string());

    let greeting = hello.clone() + exclamation_chr;
    assert_eq!("hello!", greeting.to_string());

    let greeting = hello.clone() + exclamation_str.as_str();
    assert_eq!("hello!", greeting.to_string());

    let greeting = hello.clone() + " " + world.clone() + exclamation_chr;
    assert_eq!("hello world!", greeting.to_string());

    let greeting = hello.clone() + " " + &world + exclamation_str.clone();
    assert_eq!("hello world!", greeting.to_string());

    let greeting = exclamation_chr + &hello;
    assert_eq!("!hello", greeting.to_string());

    let greeting = exclamation_str.clone() + &hello;
    assert_eq!("!hello", greeting.to_string());

    let greeting = exclamation_chr + hello.clone();
    assert_eq!("!hello", greeting.to_string());

    let greeting = &exclamation_str + hello.clone();
    assert_eq!("!hello", greeting.to_string());

    let greeting = exclamation_str.clone() + hello.clone();
    assert_eq!("!hello", greeting.to_string());

    let greeting = hello.clone() + &exclamation_str;
    assert_eq!("hello!", greeting.to_string());

    let greeting = smiley + hello.clone();
    assert_eq!("☺hello", greeting.to_string());

    let greeting = hello + smiley;
    assert_eq!("hello☺", greeting.to_string());
}
