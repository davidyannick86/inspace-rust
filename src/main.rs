use colored::*;
use figlet_rs::FIGfont;
use serde::Deserialize;

static URL: &'static str = "http://api.open-notify.org/astros.json";

#[derive(Debug, Deserialize)]
struct People {
    craft: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Inhabitant {
    people: Vec<People>,
    number: i32,
}

fn display_header() {
    let font = FIGfont::standard().unwrap();
    let figure = font.convert("Now In Space");
    println!("{}", figure.unwrap().to_string().blue());
    println!("---------------------------------");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client.get(URL.to_owned()).send().await?;

    let people: Inhabitant = res.json().await?;

    display_header();

    println!("People in space : {} \n", people.number.to_string().green());

    println!("---------------------------------");

    people.people.iter().for_each(|p| {
        println!("{} is in the {} craft", p.name.cyan(), p.craft.red());
    });

    Ok(())
}
