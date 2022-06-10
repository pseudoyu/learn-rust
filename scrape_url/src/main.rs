use std::{fs, process::Output};

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
    let args = std::env::args().collect::<Vec<String>>();

    // 常规解决方案
    // if args.len() != 3 {
    //     eprintln!("Usage: {} <url> <output>", args[0]);
    // }

    // let url = &args[1];
    // let output = &args[2];

    // println!("Fetching url {} and export to {}", url, output);

    // // 使用 ? 传播错误
    // let body = reqwest::blocking::get(url).unwrap().text()?;

    // println!("Converting html to markdown...");
    // let md = html2md::parse_html(&body);

    // fs::write(output, md.as_bytes())?;
    // println!("Converted markdown has been saved in {}", output);

    // 模式匹配解决方案
    if let [_, url, output] = args.as_slice() {
        println!("Fetching url {} and export to {}", url, output);

        // 使用 ? 传播错误
        let body = reqwest::blocking::get(url).unwrap().text()?;

        println!("Converting html to markdown...");
        let md = html2md::parse_html(&body);

        fs::write(output, md.as_bytes())?;
        println!("Converted markdown has been saved in {}", output);
    } else {
        eprintln!("Invalid arguments");
    }

    // 返回 T 正确类型
    Ok(())
}
