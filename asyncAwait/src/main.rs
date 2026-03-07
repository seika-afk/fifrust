use reqwest;




#[tokio::main]
async fn main()-> Result<(),Box<dyn std::error::Error>> {

    let res= reqwest::get("http://httpbin.org/get").await?;

    println!("status : {}",res.status());

    println!("Headers : {:#?}",res.headers());

    let body = res.text().await?;

    println!("Body : {}",body);

    Ok(())
}

