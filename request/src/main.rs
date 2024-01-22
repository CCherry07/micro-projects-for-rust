use error_chain::error_chain;
use std::io::Read;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
    let mut body = String::new();
    // 处理响应体
    res.read_to_string(&mut body)?;

    println!("status::{:?}", res.status());
    println!("header::{:?}", res.headers());
    println!("body::{}", body);

    Ok(())
}
