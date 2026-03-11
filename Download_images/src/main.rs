use reqwest;
use std::fs::File;
use std::io::copy;
use tempfile::Builder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let tmp_dir = Builder::new().prefix("Example").tempdir()?;    
    let target = "https://httpbin.org/image/png";

    let response = reqwest::get(target).await?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: {}", fname);

        let fname = tmp_dir.path().join(fname);

        println!("will be located at location: {:?}", fname);

        File::create(fname)?
    };

    let content = response.bytes().await?;
    copy(&mut content.as_ref(), &mut dest)?;

    println!("download complete!");

    // Prevent the temp directory from being deleted
    let kept_path = tmp_dir.keep();
    println!("file kept in directory: {:?}", kept_path);

    Ok(())
}
