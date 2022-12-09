// Get user input of csv file path and package name DONE
// Parse file to get the repo name and repourl
// Get repo file details
// Make request for the contents of a Cargo.toml file from the repo
// Find package in Cargo.toml
// If present, output
// else keep going
use std::error::Error;
use std::env;
use std::fs;
use url::{Url};
use toml::Value;

async fn get_text(text: &str) -> Result<String, Box<dyn Error>> {
    let resp = &reqwest::get(text)
        .await?
        .text()
        .await?;
    println!("{:#?}", resp);
    Ok(resp.to_string())
}
 
#[tokio::main]
async fn main() {
    let raw = Url::parse("https://raw.githubusercontent.com/").expect("Unable to parse URL");
    let master_cargo: &str = "/master/Cargo.toml";
    let main_cargo: &str = "/main/Cargo.toml";
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return println!("Some arguments are missing...");
    }

    let file_path = &args[1];
    let package = &args[2];

    // println!("Searching for '{}'", package);
    // println!("Through the 'Cargo.toml' of the repos in the csv file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong while reading the file...");
    let mut links: Vec<&str> = contents.split("\n").collect();

    // Remove last element as it is always empty and will cause issues with url parsing
    links.remove(links.len() - 1);

    if links.len() <= 0 {
        return println!("No links found in file!");
    }

    // Working with one link first
    let link = &links[0];
    let cur = Url::parse(link).expect("Unable to parse URL");
    let cur_path: &str = cur.path();

    // Merge the paths
    let cur_path_master = format!("{}{}", cur_path, master_cargo).to_owned();
    let cur_path_main = format!("{}{}", cur_path, main_cargo).to_owned();

    // Get the raw urls for both
    let raw_url_master = raw.join(&cur_path_master).expect("Unable to parse URL");
    let raw_url_main = raw.join(&cur_path_main).expect("Unable to parse URL");

    // Get the raw url strs for both
    let raw_url_master_str = raw_url_master.as_str();
    let raw_url_main_str = raw_url_main.as_str();

    // Get the contents of the Cargo.toml of both
    let cargo_from_master = get_text(raw_url_master_str).await.unwrap();
    let cargo_from_main = get_text(raw_url_main_str).await.unwrap();

    // Parse the tomls
    let parsed = if cargo_from_master == "404: Not Found" { cargo_from_main.parse::<Value>().unwrap() } else { cargo_from_master.parse::<Value>().unwrap() };

    println!("{}", parsed["dependencies"]);
    // println!("{:#?}", cargo_from_master);

    // println!("{}\n{}", cargo_from_master, cargo_from_main);
    // println!("{}\n{}", raw_url_master, raw_url_main);

    // for link in &links {
        // let cur = Url::parse(link).expect("parsed path").path();
        // let path = cur.path();
        // println!("{}: {}", link, cur.expect("something").path());
    // }

    // println!("{:?}", links);
}
