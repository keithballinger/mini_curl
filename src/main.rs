//use std::io::{self, Write};
use std::io::{self};
use clap::{Parser, Subcommand};
use reqwest::blocking::Client;
use url::Url;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Get { 
        url: String, 
        #[clap(short, long)]
        output: Option<String>, 
    },
    #[clap(name = "POST")] 
    Post { 
        url: String, 
        #[clap(short, long)] 
        output: Option<String>, 
        #[clap(short='d', long)] // Add a -d flag for data
        data: Option<String>
    } 
    // You can add more subcommands later like Put, Delete, etc.
}




fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Get { url, output } => process_get_request(url, output)?,
        Commands::Post { url, output, data } => process_post_request(url, output, data)?,
        // ... (add cases for other methods as you implement them)
    }

    Ok(())
}

fn process_get_request(url_str: &str, output: &Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(url_str)?;
    let client = Client::new();
    let mut response = client.get(url).send()?; 

    if let Some(filename) = output {
        let mut file = std::fs::File::create(filename)?;
        io::copy(&mut response, &mut file)?;
    } else {
        io::copy(&mut response, &mut io::stdout())?;
    }

    Ok(())
}

fn process_post_request(url_str: &str, output: &Option<String>, data: &Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(url_str)?;
    let client = Client::new();

    let mut response = if let Some(data) = data {
        client.post(url).body(data.clone()).send()?
    } else {
        client.post(url).send()?
    };

    if let Some(filename) = output {
        let mut file = std::fs::File::create(filename)?;
        io::copy(&mut response, &mut file)?; 
    } else {
        io::copy(&mut response, &mut io::stdout())?; 
    }

    Ok(())
}


