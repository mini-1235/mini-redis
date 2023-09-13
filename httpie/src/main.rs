use reqwest::Error;
use colored::*;
async fn get_request() -> Result<(), Error> {
    //user input url
    let mut url = String::new();
    println!("Enter URL: ");
    std::io::stdin().read_line(&mut url).expect("Error reading URL");
    //send request
    let response = reqwest::get(&url).await?;
    //print protocol
    println!("{:?}", response.version());
    //print status code
    println!("{:?}", response.status());
    //print reason phrase
    println!("{:?}", response.status().canonical_reason());
    //print headers
    println!("Headers:\n{:#?}", response.headers());
    //print body
    let body = response.text().await?;
    println!("Body:\n{}", body);


    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    get_request().await?;
    Ok(())
}