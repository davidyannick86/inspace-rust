use colored::*;
use figlet_rs::FIGfont;
use serde::Deserialize;

// * URL to fetch the data
static URL: &'static str = "http://api.open-notify.org/astros.json";

// * Struct to deserialize the JSON data
#[derive(Debug, Deserialize)]
struct People {
    craft: String,
    name: String,
}

// * Struct to deserialize the JSON data
#[derive(Debug, Deserialize)]
struct Inhabitant {
    people: Vec<People>,
    number: i32,
}

// * Display the header
fn display_header(title: &str) {
    // * Create the FIGfont
    let font = FIGfont::standard().unwrap();
    // * Convert the title to FIGlet
    let figure = font.convert(title);

    println!("{}", figure.unwrap().to_string().blue());
    println!("---------------------------------");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // * Fetch the data from the URL
    let client = reqwest::Client::new();
    // * Send the request
    let res = client.get(URL.to_owned()).send().await?;

    // * Deserialize the JSON data
    let people: Inhabitant = res.json().await?;

    display_header("Now In Space");

    println!("People in space : {} \n", people.number.to_string().green());

    println!("---------------------------------");

    // * Display the people in space
    people.people.iter().for_each(|p| {
        println!("{} is in the {} craft", p.name.cyan(), p.craft.red());
    });

    Ok(())
}
