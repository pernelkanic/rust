use std::{error::Error, io::Read};


fn main() -> Result<(), Box<dyn Error>> {
    
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    println!("{:#?}" , res.headers());
    println!("{}" , res.status());
    println!("{}" , body);
    Ok(())
    

}
