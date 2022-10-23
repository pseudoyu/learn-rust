trait Formatter {
    fn format(&self, input: &mut String) -> bool;
}

struct RustFormatter;
impl Formatter for RustFormatter {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("\nformatted with rust formatter");
        true
    }
}

struct HtmlFormatter;
impl Formatter for HtmlFormatter {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("\nformatted with html formatter");
        true
    }
}

fn format(input: &mut String, formatters: Vec<&dyn Formatter>) {
    for formatter in formatters {
        formatter.format(input);
    }
}

pub fn run() {
    // 动态分派
    println!("\n>>> Dynamic dispatching start...");
    let mut text = "Hello world!".to_string();
    let rust: &dyn Formatter = &RustFormatter;
    let html: &dyn Formatter = &HtmlFormatter;
    let formatters = vec![rust, html];
    format(&mut text, formatters);

    println!("text: {}", text);
}