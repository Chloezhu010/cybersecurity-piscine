// use clap::Parser;
use scraper:: {Html, Selector};


// /// Simple program to greet a person
// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// struct Args {
//     /// Name of the person to greet
//     #[arg(short, long)]
//     name: String,

//     /// Number of times to greet
//     #[arg(short, long, default_value_t = 1)]
//     count: u8,
// }

// fn main() {
//     let args = Args::parse();

//     for _ in 0..args.count {
//         println!("Hello {}!", args.name);
//     }
// }

// fn main() -> Result<(), reqwest::Error> {
//     let body = reqwest::blocking::get("https://www.rust-lang.or")?
//     .error_for_status()?
//     .text()?;

//     // println!("Status: {}", body.status());
//     // println!("Content-type: {:?}", body.headers().get("content-type"));

//     // let html = body.text()?;
//     println!("Body bytes:{}", body.len());

    
//     Ok(())
// }

fn main() {
    let html = r#"
          <html><body>
              <img src="cat.jpg">
              <img src="/img/dog.png" alt="dog">
              <img src="https://cdn.example.com/bird.gif">
              <a href="page2.html"><img src="../mouse.bmp"></a>
              <img>   <!-- no src at all -->
          </body></html>
    "#;

    // let html = r#"
    //     <!DOCTYPE html>
    //     <meta charset="utf-8">
    //     <title>Hello, world!</title>
    //     <h1 class="foo">Hello, <i>world!</i></h1>
    // "#;
    
    let doc = Html::parse_document(html);
    // println!("Parsed HTML document: {:#?}", doc);
    let selector = Selector::parse("img").unwrap();
    for element in doc.select(&selector) {
        let src = element.value().attr("src");
        println!("found img, src = {:?}", src);
    }

}