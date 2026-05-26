use clap::Parser;

/// Spider, a simple web scrapper written in Rust
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    /// URL to scrape
    url: String,

    /// Follow links recursively
    #[arg(short = 'r', long, default_value_t = false)]
    recursive: bool,

    /// Max recursion depth
    #[arg(short = 'l', long, default_value_t = 5)]
    length: usize,

    /// Output path for scraped data
    #[arg(short = 'p', long, default_value = "./data/")]
    path: String,
}

fn parse_args() -> Args {
    Args::parse()
}

fn fetch_page(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::blocking::get(url)?.error_for_status()?;
    let body = response.text()?;
    Ok(body)
}

fn extract_image_urls(html: &str, base_url: &Url) -> Vec<Url> {
    
}

fn main(){
    let args = parse_args();

    println!("URL: {}", args.url);
    println!("Recursive: {}", args.recursive);
    println!("Max Recursion Depth: {}", args.length);
    println!("Output Path: {}", args.path);

    match fetch_page(&args.url) {
        Ok(content) => println!("Fetched content ({} bytes)", content.len()),
        Err(e) => eprintln!("Failed to fetch page: {}", e),
    }
}