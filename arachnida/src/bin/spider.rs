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

fn main(){
    let args = Args::parse();

    println!("URL: {}", args.url);
    println!("Recursive: {}", args.recursive);
    println!("Max Recursion Depth: {}", args.length);
    println!("Output Path: {}", args.path);
}