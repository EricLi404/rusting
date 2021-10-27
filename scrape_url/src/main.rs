use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut url = "https://www.rust-lang.org/";
    let mut output = "./target/rust.md";

    let args = std::env::args().collect::<Vec<String>>();
    if let [_path, _url, _output, ..] = args.as_slice() {
        output = _output;
        url = _url;
    } else {
        eprintln!("参数缺失");
    }
    println!("Fetching url: {}, output: {}", url, output);
    let body = reqwest::blocking::get(url)?.text()?;

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}.", output);

    Ok(())
}