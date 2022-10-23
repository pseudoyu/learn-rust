pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic Formatting
    println!("{} is from {}", "Yu", "China");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Yu", "China", "code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "Yu",
        activity = "Rust"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}