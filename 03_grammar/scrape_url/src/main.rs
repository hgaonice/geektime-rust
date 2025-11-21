fn main() {
    let url = "https://www.rust-lang.org";
    let output = "rust.md";
    println!("Scraping {} to {}", url, output);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    let md = html2md::parse_html(&body);
    std::fs::write(output, md.as_bytes()).unwrap();
    println!("saved to {},successfully", output);

    println!("applying square to 5: {}", apply(5, square));
    println!("applying cube to 5: {}", apply(5, cube));
}

fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn square(value: i32) -> i32 {
    value * value
}

fn cube(value: i32) -> i32 {
    value * value * value
}

fn _pi() -> f64 {
    3.14159265358979323846
}
