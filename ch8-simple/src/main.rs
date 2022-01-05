use std::error::Error;
use reqwest;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.rustinaction.com/";
    let mut response = reqwest::get(url).unwrap();
    let content = response.text().expect("failed to get text");
    println!("{}", content);

    Ok(())
}
