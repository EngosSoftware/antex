use antex::{leaf, node, Color, ColorMode, StyledText, Text};

const EXPECTED: &str = r#"
 [30m[47mnode 1[0m
[33m ├─[0m [1m[40mA[41mB[42mC[43mD[44mE[45mF[46mG[47mH[43mI[48;5;134mJ[48;2;60;130;87mK[46mL[0mM[0m
[33m │ [0m ├─[0m line 1_1_1[0m
[33m │ [0m │ [0m line 1_1_2[0m
[33m │ [0m │ [0m line 1_1_3[0m
[33m │ [0m │ [0m line 1_1_4[0m
[33m │ [0m │ [0m last line!
[33m │ [0m └─[0m [1m[40mo[41mn[42ml[43my[44m [45mo[46mn[47me [43ml[48;5;134mi[48;2;60;130;87mn[46me[0m.[0m
[33m ├─[0m [3mnode 1_2[0m
[33m │ [0m[34m ├─[0m only one line[0m
[33m │ [0m[34m ├─[0m line 1_2_1[0m
[33m │ [0m[34m │ [0m line 1_2_2[0m
[33m │ [0m[34m │ [0m line 1_2_3[0m
[33m │ [0m[34m │ [0m line 1_2_4[0m
[33m │ [0m[34m └─[0m [3monly one line...[0m
[33m └─[0m [4mnode 1_3...with multiple characters[0m
[33m   [0m[35m ├─[0m [30ma[31mb[32mc[33md[34me[35mf[36mg[37mh[33mi[38;5;134mj[38;2;60;130;87mk[33ml[0mm[0m
[33m   [0m[35m │ [0m[31m ├─[0m line 1_3_1_1[0m
[33m   [0m[35m │ [0m[31m │ [0m line 1_3_1_2[0m
[33m   [0m[35m │ [0m[31m │ [0m line 1_3_1_3[0m
[33m   [0m[35m │ [0m[31m │ [0m line 1_3_1_4[0m
[33m   [0m[35m │ [0m[31m └─[0m [4monly one line[0m
[33m   [0m[35m ├─[0m line 1_3_1[0m
[33m   [0m[35m │ [0m line 1_3_2[0m
[33m   [0m[35m │ [0m line 1_3_3[0m
[33m   [0m[35m │ [0m line 1_3_4[0m
[33m   [0m[35m └─[0m [30mo[31mn[32ml[33my[34m|[35mo[36mn[37me[33m|[38;5;134ml[38;2;60;130;87mi[33mne[0m with multiple characters[0m
"#;

#[test]
fn building_styled_tree_should_work() {
  let cm = ColorMode::On;

  let last_line = Text::new(cm).s("last line!");
  let mut leaf_node = leaf(cm)
    .line()
    .s("line 1_1_1")
    .end()
    .line()
    .s("line 1_1_2")
    .end()
    .line()
    .s("line 1_1_3")
    .end()
    .line()
    .s("line 1_1_4")
    .end();
  leaf_node.add_line(last_line);

  let root = node(Color::Yellow, cm)
    .line()
    .black()
    .bg_color(Color::White)
    .s("node 1")
    .end()
    .child(
      node(Color::None, cm)
        .line()
        .bold()
        .bg_black()
        .s("A")
        .bg_red()
        .s("B")
        .bg_green()
        .s("C")
        .bg_yellow()
        .s("D")
        .bg_blue()
        .s("E")
        .bg_magenta()
        .s("F")
        .bg_cyan()
        .s("G")
        .bg_white()
        .s("H")
        .bg_color_8(3)
        .s("I")
        .bg_color_256(134)
        .s("J")
        .bg_color_rgb((60, 130, 87))
        .s("K")
        .bg_color(Color::Cyan)
        .s("L")
        .normal()
        .s("M")
        .end()
        .child(leaf_node.end())
        .child(
          leaf(cm)
            .line()
            .bold()
            .bg_black()
            .s("o")
            .bg_red()
            .s("n")
            .bg_green()
            .s("l")
            .bg_yellow()
            .s("y")
            .bg_blue()
            .s(" ")
            .bg_magenta()
            .s("o")
            .bg_cyan()
            .s("n")
            .bg_white()
            .s("e")
            .s(" ")
            .bg_color_8(3)
            .s("l")
            .bg_color_256(134)
            .s("i")
            .bg_color_rgb((60, 130, 87))
            .s("n")
            .bg_color(Color::Cyan)
            .s("e")
            .normal()
            .s(".")
            .end()
            .end(),
        )
        .end(),
    )
    .child(
      node(Color::Blue, cm)
        .line()
        .italic()
        .s("node 1_2")
        .end()
        .child(leaf(cm).line().s("only one line").end().end())
        .child(
          leaf(cm)
            .line()
            .bg_color(Color::None)
            .s("line 1_2_1")
            .end()
            .line()
            .s("line 1_2_2")
            .end()
            .line()
            .s("line 1_2_3")
            .end()
            .line()
            .s("line 1_2_4")
            .end()
            .end(),
        )
        .child(leaf(cm).line().italic().s("only one line").repeat('.', 3).end().end())
        .end(),
    )
    .child(
      node(Color::Magenta, cm)
        .line()
        .underline()
        .s("node 1_3")
        .repeat('.', 3)
        .plural("with multiple character", 2)
        .end()
        .child(
          node(Color::Red, cm)
            .line()
            .black()
            .s("a")
            .red()
            .s("b")
            .green()
            .s("c")
            .yellow()
            .s("d")
            .blue()
            .s("e")
            .magenta()
            .s("f")
            .cyan()
            .s("g")
            .white()
            .s("h")
            .color_8(3)
            .s("i")
            .color_256(134)
            .s("j")
            .color_rgb((60, 130, 87))
            .s("k")
            .color(Color::Yellow)
            .s("l")
            .normal()
            .s("m")
            .end()
            .child(
              leaf(cm)
                .line()
                .s("line 1_3_1_1")
                .end()
                .line()
                .s("line 1_3_1_2")
                .end()
                .line()
                .s("line 1_3_1_3")
                .end()
                .line()
                .s("line 1_3_1_4")
                .end()
                .end(),
            )
            .child(leaf(cm).line().underline().s("only one line").end().end())
            .end(),
        )
        .child(
          leaf(cm)
            .line()
            .s("line 1_3_1")
            .end()
            .line()
            .s("line 1_3_2")
            .end()
            .line()
            .s("line 1_3_3")
            .end()
            .line()
            .s("line 1_3_4")
            .end()
            .end(),
        )
        .child(
          leaf(cm)
            .line()
            .black()
            .s("o")
            .red()
            .s("n")
            .green()
            .s("l")
            .yellow()
            .s("y")
            .blue()
            .s("|")
            .magenta()
            .s("o")
            .cyan()
            .s("n")
            .white()
            .s("e")
            .color_8(3)
            .s("|")
            .color_256(134)
            .s("l")
            .color_rgb((60, 130, 87))
            .s("i")
            .color(Color::Yellow)
            .s("n")
            .s("e")
            .normal()
            .plural(" with multiple character", 100)
            .end()
            .end(),
        )
        .end(),
    )
    .end();

  assert_eq!(EXPECTED, format!("\n{}", root));
}
