mod colors;
mod text;
mod tree;

use colors::*;
use std::env;
use std::fmt::Write;
use text::*;
use tree::*;

fn display_colors_8() {
    println!("Foreground 8 colors:\n");
    let text = Text::default()
        .black()
        .str(" 0 ")
        .red()
        .str(" 1 ")
        .green()
        .str(" 2 ")
        .yellow()
        .str(" 3 ")
        .blue()
        .str(" 4 ")
        .magenta()
        .str(" 5 ")
        .cyan()
        .str(" 6 ")
        .white()
        .str(" 7 ")
        .clear();
    println!("{}", text);
    println!("{}", Text::default().bold() + text);
    println!();
}

fn display_background_colors_8() {
    println!("Background 8 colors:\n");
    let text = Text::default()
        .bg_black()
        .str(" 0 ")
        .bg_red()
        .str(" 1 ")
        .bg_green()
        .str(" 2 ")
        .bg_yellow()
        .str(" 3 ")
        .bg_blue()
        .str(" 4 ")
        .bg_magenta()
        .str(" 5 ")
        .bg_cyan()
        .str(" 6 ")
        .bg_white()
        .str(" 7 ")
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
            text = text.color(code).str(&format!(" {code:>3} "));
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
            text = text.bg_color(code).str(&format!(" {code:>3} "));
        }
        text = text.clear().nl()
    }
    println!("{}", text);
    println!("{}", Text::default().bold() + text);
    println!();
}

fn display_some_text() {
    Text::default()
        .str("Hello ")
        .red()
        .str("world!")
        .clear()
        .print();
    Text::default()
        .str("Hello ")
        .red()
        .str("world")
        .chr('!')
        .clear()
        .println();
    println!(
        "{}",
        Text::default()
            .str("Hello ")
            .color(69)
            .str("world!")
            .bold()
            .str("world!")
            .clear()
            .rgb(100, 230, 100)
            .bg_rgb(120, 120, 120)
            .str("again!")
            .clear()
            .italic()
            .str(" and now some italic ")
            .clear()
            .underline()
            .str("and underlined")
            .clear()
    );
}

fn display_tree() {
    let cm = ColorMode::default();
    let root = TreeNode::node()
        .line(Text::new(cm).blue().str("node 4").clear())
        .child(
            TreeNode::node()
                .line(Text::new(cm).str("node 1"))
                .child(
                    TreeNode::leaf()
                        .line(Text::new(cm).str("line 1_1"))
                        .line(Text::new(cm).str("line 1_2"))
                        .line(Text::new(cm).str("line 1_3"))
                        .line(Text::new(cm).str("line 1_4"))
                        .done(),
                )
                .child(
                    TreeNode::leaf()
                        .line(Text::new(cm).str("only one line"))
                        .done(),
                )
                .done(),
        )
        .child(
            TreeNode::node()
                .line(Text::new(cm).str("node 2"))
                .child(
                    TreeNode::leaf()
                        .line(Text::new(cm).str("only one line"))
                        .done(),
                )
                .child(
                    TreeNode::leaf()
                        .line(Text::new(cm).str("line 2_1"))
                        .line(Text::new(cm).str("line 2_2"))
                        .line(Text::new(cm).str("line 2_3"))
                        .line(Text::new(cm).str("line 2_4"))
                        .done(),
                )
                .child(
                    TreeNode::leaf()
                        .line(Text::new(cm).str("only one line"))
                        .done(),
                )
                .done(),
        )
        .child(
            TreeNode::node()
                .line(Text::new(cm).str("node 3"))
                .child(
                    TreeNode::node()
                        .line(Text::new(cm).str("node 1"))
                        .child(
                            TreeNode::leaf()
                                .line(Text::new(cm).str("line 3_1_1"))
                                .line(Text::new(cm).str("line 3_1_2"))
                                .line(Text::new(cm).str("line 3_1_3"))
                                .line(Text::new(cm).str("line 3_1_4"))
                                .done(),
                        )
                        .child(
                            TreeNode::leaf()
                                .line(Text::new(cm).str("only one line"))
                                .done(),
                        )
                        .done(),
                )
                .child(
                    TreeNode::leaf()
                        .line(Text::new(cm).str("line 3_1"))
                        .line(Text::new(cm).str("line 3_2"))
                        .line(Text::new(cm).str("line 3_3"))
                        .line(Text::new(cm).str("line 3_4"))
                        .done(),
                )
                .child(
                    TreeNode::leaf()
                        .line(Text::new(cm).str("only one line"))
                        .done(),
                )
                .done(),
        )
        .done();

    let tree = Tree::new(root);
    println!("{}", tree);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        Text::default()
            .red()
            .str("Invalid number of arguments!")
            .clear()
            .println();
        return;
    }
    match args[1].to_lowercase().trim() {
        "1" => display_colors_8(),
        "2" => display_background_colors_8(),
        "3" => display_colors_256(),
        "4" => display_background_colors_256(),
        "5" => display_some_text(),
        "6" => display_tree(),
        _ => {
            Text::default()
                .red()
                .str("Unknown command: ")
                .clear()
                .str(args[1].clone())
                .println();
        }
    }
}
