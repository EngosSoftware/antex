mod colors;
mod text;

use text::*;

fn display_colors_8() {
    println!("Foreground 8 colors:\n");
    let text = Text::default()
        .black()
        .append(" 0 ")
        .red()
        .append(" 1 ")
        .green()
        .append(" 2 ")
        .yellow()
        .append(" 3 ")
        .blue()
        .append(" 4 ")
        .magenta()
        .append(" 5 ")
        .cyan()
        .append(" 6 ")
        .white()
        .append(" 7 ")
        .clear();
    println!("{}", text);
    println!("{}", Text::default().bold() + text);
    println!();
}

fn display_background_colors_8() {
    println!("Background 8 colors:\n");
    let text = Text::default()
        .bg_black()
        .append(" 0 ")
        .bg_red()
        .append(" 1 ")
        .bg_green()
        .append(" 2 ")
        .bg_yellow()
        .append(" 3 ")
        .bg_blue()
        .append(" 4 ")
        .bg_magenta()
        .append(" 5 ")
        .bg_cyan()
        .append(" 6 ")
        .bg_white()
        .append(" 7 ")
        .clear();
    println!("{}", text);
    println!("{}", Text::default().bold() + text);
    println!();
}

fn display_colors_256() {
    println!("Foreground 256 colors:\n");
    let mut text = Text::default();
    for i in 0..16 {
        for j in 0..16 {
            let code = i * 16 + j;
            text = text.color(code).append(&format!(" {code:>3} "));
        }
        text = text.clear().nl()
    }
    println!("{}", text);
    println!("{}", Text::default().bold() + text);
    println!();
}

fn display_background_colors_256() {
    println!("Background 256 colors:\n");
    let mut text = Text::default();
    for i in 0..16 {
        for j in 0..16 {
            let code = i * 16 + j;
            text = text.bg_color(code).append(&format!(" {code:>3} "));
        }
        text = text.clear().nl()
    }
    println!("{}", text);
    println!("{}", Text::default().bold() + text);
    println!();
}

fn display_some_text() {
    Text::default()
        .append("Hello ")
        .red()
        .append("world!")
        .clear()
        .print();
    println!(
        "{}",
        Text::default()
            .append("Hello ")
            .red()
            .append("world!")
            .clear()
    );
    println!(
        "{}",
        Text::default()
            .append("Hello ")
            .color(69)
            .append("world!")
            .bold()
            .append("world!")
            .clear()
            .rgb(100, 230, 100)
            .bg_rgb(120, 120, 120)
            .append("again!")
            .clear()
            .italic()
            .append(" and now some italic ")
            .clear()
            .underline()
            .append("and underlined")
            .clear()
    );
}

fn display_all() {
    display_colors_8();
    display_background_colors_8();
    display_colors_256();
    display_background_colors_256();
    display_some_text();
}

fn main() {
    println!();
    display_all();
}
