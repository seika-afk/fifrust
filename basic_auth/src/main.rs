use reqwest::blocking::Client;


fn main()->Result<(),Box<dyn std::error::Error>>{


    let client= Client::new();
    let user:String = "testuser".into();
    let passwd: Option<String>=None;


    let response= client
                    .get("http://httpbin.org/get")
                    .basic_auth(user,passwd)
                    .send()?; 
    println!("{:?}",response);

    Ok(())

}
