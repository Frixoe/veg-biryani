// Rough thinking
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

// Get the html content of the page being requested
async fn get_text(text: &str) -> Result<String, Box<dyn Error>> {
    let resp = &reqwest::get(text)
        .await?
        .text()
        .await?;
    Ok(resp.to_string())
}
 
#[tokio::main]
async fn main() {
    // The github raw url
    let raw = Url::parse("https://raw.githubusercontent.com/").expect("Unable to parse URL");

    // Some repos use master as the prod branch while others use main so trying both
    let master_cargo: &str = "/master/Cargo.toml";
    let main_cargo: &str = "/main/Cargo.toml";

    // Collecting args
    let args: Vec<String> = env::args().collect();

    // Some args might be missing because user ret
    if args.len() < 3 {
        return println!("Some arguments are missing...");
    }

    // Get the file path
    let file_path = &args[1];

    // Get the package name
    let package = &args[2];

    // Read the contents of the file
    let contents = fs::read_to_string(file_path).expect("Something went wrong while reading the file...");

    // Collect the links from the file
    let mut links: Vec<&str> = contents.split("\n").collect();

    // Remove last element as it is always empty and will cause issues with url parsing
    links.remove(links.len() - 1);

    // If no links then exit
    if links.len() <= 0 {
        return println!("No links found in file!");
    }

    // Iterate through every link in the file
    for link in &links {
        // Parse the link
        let cur = Url::parse(link).expect("Unable to parse URL");

        // Get the route
        let cur_path: &str = cur.path();

        // Merge the paths
        let cur_path_master = format!("{}{}", cur_path, master_cargo).to_owned();
        let cur_path_main = format!("{}{}", cur_path, main_cargo).to_owned();

        // Get the raw github urls for both
        let raw_url_master = raw.join(&cur_path_master).expect("Unable to parse URL");
        let raw_url_main = raw.join(&cur_path_main).expect("Unable to parse URL");

        // Get the raw github url strs for both
        let raw_url_master_str = raw_url_master.as_str();
        let raw_url_main_str = raw_url_main.as_str();

        // Get the contents of the Cargo.toml of both
        let cargo_from_master = get_text(raw_url_master_str).await.unwrap();
        let cargo_from_main = get_text(raw_url_main_str).await.unwrap();

        // Parse the tomls
        let parsed = if cargo_from_master == "404: Not Found" { cargo_from_main.parse::<Value>().unwrap() } else { cargo_from_master.parse::<Value>().unwrap() };

        // Convert the dependencies section to a string to be queried
        let dep_str = parsed["dependencies"].to_string();

        // Strings to search for 
        let search_1 = format!("{} ", package);
        let search_2 = format!("[{}]", package);

        // If the package is contained in the dependencies tell the user
        if dep_str.contains(&search_1) || dep_str.contains(&search_2) {
            // Package found
            println!("===> {} found in {}", package, link);
        } else {
            // lol
            println!("{} not found in {}", package, link);
        }
    }
}
