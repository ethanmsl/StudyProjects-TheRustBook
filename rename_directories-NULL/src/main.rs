use std::fs;

fn main() -> std::io::Result<()> {
    fs::rename("/Users/eskowronski-lutz/Documents/Programming_Langs/Rust/book-projects-rust/boop.txt", "peeb.txt")?;
    Ok(())
}
