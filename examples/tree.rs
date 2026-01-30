use antex::{leaf, node, Color, ColorMode, StyledText};

fn main() {
  // Collect the command-line arguments.
  let args = std::env::args().collect::<Vec<String>>();

  // The first argument may be the color mode, like: auto, never, always
  let color_mode = if args.len() > 1 { ColorMode::new(&args[1]) } else { ColorMode::default() };

  // The second argument should be the indentation, the tree will be shifted to the right.
  let indent = if args.len() > 2 { args[2].parse::<usize>().unwrap_or_default() } else { 0 };

  // Build the tree.
  let root = node(Color::Rgb((80, 12, 140)), color_mode)
    .line()
    .bold()
    .underline()
    .blue()
    .s("Quantum AI Core")
    .end()
    .child(
      node(Color::Yellow, color_mode)
        .line()
        .yellow()
        .s("Neural Processing Unit Alpha")
        .end()
        .child(
          leaf(color_mode)
            .line()
            .s("Synapse Array Load: 98%")
            .end()
            .line()
            .s("Memory Cache Temp: -12°C")
            .end()
            .line()
            .magenta()
            .s("Quantum Bit Flux: 0.0012")
            .end()
            .line()
            .bg_yellow()
            .color(Color::Rgb((10, 10, 10)))
            .s(" Neural Spike Rate: 14,000 Hz ")
            .end()
            .end(),
        )
        .child(leaf(color_mode).line().s("Optimization Status: ").green().bold().s("Perfect").end().end())
        .end(),
    )
    .child(
      node(Color::None, color_mode)
        .line()
        .bold()
        .blue()
        .s("Dark Matter Storage Array")
        .end()
        .child(leaf(color_mode).line().s("Energy Stabilizer: Online").end().end())
        .child(
          leaf(color_mode)
            .line()
            .s("Matter Density: 9.8×10^-27 kg/m³")
            .end()
            .line()
            .s("Containment Field: 99.99%")
            .end()
            .line()
            .bold()
            .red()
            .s("Particle Flux: 1.2×10^6 s^-1")
            .end()
            .line()
            .s("Storage Efficiency: 87%")
            .end()
            .end(),
        )
        .child(leaf(color_mode).line().s("Backup Node: ").green().bold().underline().s("Ready").end().end())
        .end(),
    )
    .child(
      node(Color::None, color_mode)
        .line()
        .bold()
        .blue()
        .s("Interstellar Communication Hub")
        .end()
        .child(
          node(Color::Yellow, color_mode)
            .line()
            .yellow()
            .s("Subspace Transmitter Delta")
            .end()
            .child(
              leaf(color_mode)
                .line()
                .s("Signal Strength: 999.9 TW")
                .end()
                .line()
                .cyan()
                .s("Latency: 0.00042 s")
                .end()
                .line()
                .s("Encryption Protocol: Quantum-256")
                .end()
                .line()
                .s("Active Channels: 42")
                .end()
                .end(),
            )
            .child(leaf(color_mode).line().s("Maintenance Mode: ").italic().bg_magenta().s("Idle").end().end())
            .end(),
        )
        .child(
          leaf(color_mode)
            .line()
            .cyan()
            .s("Cosmic Bandwidth Allocation: 12.7 PB/s")
            .end()
            .line()
            .s("Error Correction Status: Optimal")
            .end()
            .line()
            .s("Packet Loss: 0.0001%")
            .end()
            .line()
            .s("Routing Algorithm: Self-Adaptive AI")
            .end()
            .end(),
        )
        .child(leaf(color_mode).line().s("Hub Status: ").green().italic().bold().s("Fully Operational").end().end())
        .end(),
    )
    .end();

  // Print the tree with indentation.
  print!("{:1$}", root, indent);
}
