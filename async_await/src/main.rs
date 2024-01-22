use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("http://httpbin.org/get").await?;
    
    println!("status::{:?}", res.status());
    println!("header::{:?}", res.headers());

    let body = res.text().await?;
    println!("body::{}", body);

    Ok(())
}
