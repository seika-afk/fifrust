use std::io::Read;


fn main() -> Result<(),Box<dyn std::error::Error>>{


    let url:String=String::from("https://jsonplaceholder.typicode.com/users");

    let mut res=reqwest::blocking::get(&url)?;
    let mut body=String::new();

    res.read_to_string(&mut body)?;
    println!("Status: {}",res.status());
    println!("------------");
    println!("Body : {}",body);
    
    Ok(())
}
