use reqwest::{Client, Error};
use serde_json::Value;
use async_recursion::async_recursion;
use tokio::fs::File as TokioFile;
use tokio::io::AsyncWriteExt;
use std::path::Path;
use llmchain::{DocumentPath, GithubRepoLoader, DocumentLoader};
/*#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();
    let base_url = "https://api.github.com/repos/Uniswap/docs/contents/";
    let start_path = "docs/contracts";
    let dir_name = "Uniswap"; // The directory where all files will be saved

    tokio::fs::create_dir_all(dir_name).await.expect("error"); // Create the directory

    retrieve_contents(&client, base_url, dir_name,start_path).await?;

    Ok(())
}*/
use std::io::Read;
use std::fs::File;
fn read_a_file() -> std::io::Result<Vec<u8>> {
    let mut file = File::open("TheGraph/website/pages/en/billing.mdx");

    let mut data = Vec::new();
    file?.read_to_end(&mut data);
    println!("{:?}", data); 
    return Ok(data);
}

#[tokio::main]
async fn main() ->  Result<(), Error> {
    //env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // documents
    let documents = GithubRepoLoader::create()
        .load(DocumentPath::from_string(
            "https://github.com/Uniswap/docs",
        ))
        .await.expect("failed to load documents");

    let documents2 = DocumentLoader.load(DocumentPath::from_string(
            "TheGraph/website/pages/en/billing.mdx",
        ))
        .await.expect("failed to load documents");

    println!("{:?}", documents.tokens());
    Ok(())
}

#[async_recursion]
async fn retrieve_contents(client: &Client, base_url: &str, root_dir: &str, path: &str) -> Result<(), Error> {
    let url = format!("{}{}", base_url, path);
    println!("Retrieving contents for path: {}", url); 
    let resp = client.get(&url)
        .header("User-Agent", "request")
        .send()
        .await?;
    println!("status: {}", resp.status()); 
    if resp.status().is_success() {
        let contents = resp.text().await?;
        let files: Value = serde_json::from_str(&contents).expect("JSON was not well-formatted");

        if let Some(array) = files.as_array() {
            for file in array {
                let file_name = file["name"].as_str().unwrap_or_default();
                let file_path = file["path"].as_str().unwrap_or_default();
                let file_type = file["type"].as_str().unwrap_or_default();

                let local_file_path = Path::new(root_dir).join(file_path);

                if file_type == "dir" {
                    tokio::fs::create_dir_all(&local_file_path).await.expect("create dir failed");
                    retrieve_contents(client, base_url, root_dir, file_path).await?;
                } else if file_type == "file" && is_text_or_code_file(file_name) {
                    let download_url = file["download_url"].as_str().unwrap_or_default();
                    println!("Downloading file: {}", download_url); 
                    let file_content = client.get(download_url).send().await?.text().await?;
                    let mut file = TokioFile::create(&local_file_path).await.expect("create file failed");
                    file.write_all(file_content.as_bytes()).await.expect("write file failed");
                }
            }
        }
    } else {
        println!("Failed to retrieve contents for path: {}", path);
    }

    Ok(())
}



fn is_text_or_code_file(file_name: &str) -> bool {
    // Add or remove extensions based on your needs
    let text_or_code_extensions = ["txt", "md", "json", "rs", "py", "js", "html", "css", "yaml", "toml", "xml"];
    file_name.split('.').last().map_or(false, |ext| text_or_code_extensions.contains(&ext))
}


