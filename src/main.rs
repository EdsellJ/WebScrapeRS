use reqwest;
use scraper::{Html, Selector};

#[feature(async_closure)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.wired.com/";
    // Make an HTTP GET request
    let body = reqwest::get(url)
        .await?
        .text()
        .await?;
    
    let document = Html::parse_document(&body);

    let selector_result = Selector::parse(r#"script[type="application/ld+json"]"#);
    let selector = match selector_result {
        Ok(sel) => sel,
        Err(err) => {
            eprintln!("Failed to parse: {:?}", err);
            return Ok(());
        }
    };

    for element in document.select(&selector) {
        println!("{}", element.text().collect::<String>());
    }
                                       
    Ok(())
}
