use dialoguer::{Editor, Input, Select, theme::{ ColorfulTheme}};
use reqwest;


fn print_head(){
let norm= r#"
██████╗  ██████╗ ███████╗████████╗ ██████╗██╗     ██╗
██╔══██╗██╔═══██╗██╔════╝╚══██╔══╝██╔════╝██║     ██║
██████╔╝██║   ██║███████╗   ██║   ██║     ██║     ██║
██╔═══╝ ██║   ██║╚════██║   ██║   ██║     ██║     ██║
██║     ╚██████╔╝███████║   ██║   ╚██████╗███████╗██║
╚═╝      ╚═════╝ ╚══════╝   ╚═╝    ╚═════╝╚══════╝╚═╝
"#;

let mono=r#"

 ▄▄▄▄▄▄      ▄▄▄▄      ▄▄▄▄    ▄▄▄▄▄▄▄▄     ▄▄▄▄   ▄▄         ▄▄▄▄▄▄  
 ██▀▀▀▀█▄   ██▀▀██   ▄█▀▀▀▀█   ▀▀▀██▀▀▀   ██▀▀▀▀█  ██         ▀▀██▀▀  
 ██    ██  ██    ██  ██▄          ██     ██▀       ██           ██    
 ██████▀   ██    ██   ▀████▄      ██     ██        ██           ██    
 ██        ██    ██       ▀██     ██     ██▄       ██           ██    
 ██         ██▄▄██   █▄▄▄▄▄█▀     ██      ██▄▄▄▄█  ██▄▄▄▄▄▄   ▄▄██▄▄  
 ▀▀          ▀▀▀▀     ▀▀▀▀▀       ▀▀        ▀▀▀▀   ▀▀▀▀▀▀▀▀   ▀▀▀▀▀▀  
                                                                     
    "#;

println!("{}",norm);


}


fn open_output(txt :&str)->Result<(),Box<dyn std::error::Error>>{
    let  _= Editor::new()
        .edit(txt)?; 

    println!("Bie Bie :>");
Ok(())
}


#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>> {
    print_head();
    let url: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Url : ")
        .default("http://localhost:3000".to_string())
        .interact()?;

    let items = vec!["GET", "POST","PUT","DELETE","EXIT"];
    let choice = Select::with_theme(&ColorfulTheme::default())  
        .with_prompt("Choose Method")
        .items(&items)
        .interact()?;
    match items[choice]{
        "GET"=> handle_get(&url).await?,
        "POST"=>handle_post(&url).await?,
        "DELETE"=>handle_delete(&url).await?,
        "PUT"=>handle_put(&url).await?,
        "EXIT"=>return Ok(()),
        _ =>{}

    }
Ok(())

}

async fn handle_get(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get(url)
        .await?
        .error_for_status()?
        .text()
        .await?;

    open_output(&body)?;
    Ok(())
}
async fn handle_put(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = Editor::new()
        .edit("Name=John\nBirthdate=2000-01-01")?;

    if let Some(input) = content {
        let mut name = "";
        let mut birthdate = "";

        for line in input.lines() {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() == 2 {
                match parts[0].trim() {
                    "Name" => name = parts[1].trim(),
                    "Birthdate" => birthdate = parts[1].trim(),
                    _ => {}
                }
            }
        }

        if name.is_empty() || birthdate.is_empty() {
            println!("Missing Name or Birthdate");
            return Ok(());
        }

        let client = reqwest::Client::new();

        let res = client
            .put(url)
            .query(&[
                ("Name", name),
                ("Birthdate", birthdate),
            ])
            .send()
            .await?;

        let status = res.status();
        let text = res.text().await?;

        println!("Status: {}", status);
        println!("Response: {}", text);

        open_output(&text)?;
    }

    Ok(())
}

async fn handle_post(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = Editor::new()
        .edit("Name=John\nBirthdate=2000-01-01")?;

    if let Some(input) = content {
        let mut name = "";
        let mut birthdate = "";

        for line in input.lines() {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() == 2 {
                match parts[0].trim() {
                    "Name" => name = parts[1].trim(),
                    "Birthdate" => birthdate = parts[1].trim(),
                    _ => {}
                }
            }
        }

        let client = reqwest::Client::new();

        let res = client
            .post(url)
            .query(&[
                ("Name", name),
                ("Birthdate", birthdate),
            ])
            .send()
            .await?;

        let text = res.text().await?;


        open_output(&text)?;
    }

    Ok(())
}

use dialoguer::Input;

async fn handle_delete(base_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let id: String = Input::new()
        .with_prompt("Enter ID to delete")
        .interact()?;

    let url = format!("{}/{}", base_url, id);

    let client = reqwest::Client::new();

    let res = client
        .delete(&url)
        .send()
        .await?;

    let status = res.status();
    let text = res.text().await?;

    println!("Status: {}", status);
    println!("Response: {}", text);

    open_output(&text)?;

    Ok(())
}
