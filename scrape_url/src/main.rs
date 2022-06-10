use std::fs;

// fn main() {
//     let url = "https://www.rust-lang.org/";
//     let output = "rust.md";

//     print!("Fetching url {}...", url);
//     let body = reqwest::blocking::get(url).unwrap().text().unwrap();

//     println!("Converting html to markdown...");
//     let md = html2md::parse_html(&body);

//     fs::write(output, md.as_bytes()).unwrap();
//     println!("Converted markdown has been saved in {}", output);
// }

// main() 返回 Result<T,E>
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    print!("Fetching url {}...", url);

    // 使用 ? 传播错误
    let body = reqwest::blocking::get(url).unwrap().text()?;

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}", output);

    // 返回 T 正确类型
    Ok(())
}
