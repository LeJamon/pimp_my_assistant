use serde::{Deserialize, Serialize};
use tokio;
use scraper::{Html, Selector};
use spider::website::Website;
use url::Url;
use std::fmt::write;
use std::fs::{self, File};
use std::io::{Write, stdout};
use std::path::Path;
use regex::Regex;
use reqwest::{Client, RequestBuilder, Error};
use reqwest::{Request};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::str::FromStr; 
use serde_json::Value;
use serde_json::json; 
use reqwest::header::AUTHORIZATION;
use crate::fs::create_dir_all;
use async_recursion::async_recursion;

#[derive(Serialize, Deserialize, Debug)]
struct HuggingFaceResponse {
    answer: String,
    score: f32,
    start: u32,
    end: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct LibertAIRequest {
    prompt: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GenerationSettings {
    frequency_penalty: f64,
    grammar: String,
    ignore_eos: bool,
    logit_bias: Vec<f64>,
    min_p: f64,
    mirostat: i64,
    mirostat_eta: f64,
    mirostat_tau: f64,
    model: String,
    n_ctx: i64,
    n_keep: i64,
    n_predict: i64,
    n_probs: i64,
    penalize_nl: bool,
    penalty_prompt_tokens: Vec<String>,
    presence_penalty: f64,
    repeat_last_n: i64,
    repeat_penalty: f64,
    seed: u64,
    stop: Vec<String>,
    stream: bool,
    temperature: f64,
    tfs_z: f64,
    top_k: i64,
    top_p: f64,
    typical_p: f64,
    use_penalty_prompt_tokens: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Timings {
    predicted_ms: f64,
    predicted_n: i64,
    predicted_per_second: f64,
    predicted_per_token_ms: f64,
    prompt_ms: f64,
    prompt_n: i64,
    prompt_per_second: f64,
    prompt_per_token_ms: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct LibertAIResponse {
    content: String,
    generation_settings: GenerationSettings,
    model: String,
    prompt: String,
    slot_id: i64,
    stop: bool,
    stopped_eos: bool,
    stopped_limit: bool,
    stopped_word: bool,
    stopping_word: String,
    timings: Timings,
    tokens_cached: i64,
    tokens_evaluated: i64,
    tokens_predicted: i64,
    truncated: bool,
}

impl FromStr for LibertAIResponse {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}




/*async fn giga_scrap(folderName : String, url :String){
    use std::io::{Write, stdout};
    let url = "https://docs.lens.xyz/docs/what-is-lens";
    let mut website: Website = Website::new("url");


    let parsed_url = Url::parse(url)?;
    let host = parsed_url.host_str().ok_or("Invalid URL: Missing host")?;
    let project_name = host.split('.').next().ok_or("Invalid URL: Missing project name")?;
    let dir_path = format!("./{}", project_name);
    println!("dir_path: {}", dir_path);

    website.scrape().await;
    let mut lock = stdout().lock();
    

    let separator = "-".repeat(url.len());

    for page in website.get_pages().unwrap().iter() {
        let document = Html::parse_document(&page.get_html());
        let body_selector = Selector::parse("body").unwrap(); // Assuming you want text from <body>

        let mut text_content = String::new();
        for element in document.select(&body_selector) {
            text_content.push_str(&element.text().collect::<Vec<_>>().join(" "));
        }

        writeln!(lock, "{}\n{}\n\n{}\n\n{}", separator, page.get_url_final(), text_content, separator)
            .unwrap();

}
}*/

/*async fn callLibertAI(prompt : String){
    let client = reqwest::Client::new();
    let request = LibertAIRequest {
        prompt: prompt,
    };
    let response = client
        .post("https://liberai.org/api/gpt3")
        .header("Content-Type", "application/json")
        .body(&request)
        .send()
        .await
        .unwrap();
    let response: LibertAIResponse = response.json().await.unwrap();
    println!("Response: {:?}", response);
}*/

fn format_response(instruction: &str, input: &str) -> String {
    let a = format!(
        "Below is an instruction that describes a task, paired with an input that provides further context. Write a response that appropriately completes the request.### Instruction:{}### Input:{}### Response\n",
        instruction,
        input
    ); 
    println!("a: {}", a);
    a
    
}

#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {
    /*let client = Client::new();
    let url = "https://curated.aleph.cloud/vm/b950fef19b109ef3770c89eb08a03b54016556c171b9a32475c085554b594c94/completion";
    let prompt = format_response("Clean and summarize the following project documentation text. Remove all redundant, non-essential parts, and focus on preserving key information and concepts.", "Jump to Content Guides v1 v1.1 v2 Moon (Dark Mode) Sun (Light Mode) v 2 Search Discard Submit Suggested Edits Documentation View V1 Documentation What is Lens? 🌿 What's new in Lens V2? Overview Developer Quickstart Scaffold a new Lens app Deployed Contract Addresses Claiming a profile Major Concepts Profile Handle Publication Comment Quote Mirror Open Actions Collect Action Follow Profile Manager Referrals Token Guardian SDKs React Hooks SDK v2 LensClient SDK Lens API Introduction API links Public BigQuery Advanced API docs Querying from an Application LensClient SDK Apollo client URQL Authentication Login Refresh JWT Approved Authentications Revoke Authentication Verify JWT Helpers (LensClient) Gasless & Signless Broadcast Onchain Transaction Broadcast Momoka Transaction Lens Profile Manager Relay Queues Tracking Explore Explore profiles Explore publications Feed Feed Feed highlights Latest Paid Actions Follow Follow Unfollow Set Follow Module Following Followers Is Followed By Me? Is Following Me? Health Ping Indexer Lens Transaction Status TxId to TxHash Invites Is profile already invited Invited profiles Invite a profile Media Snapshots Metadata Standards Publication metadata Profile metadata NFTs Get Users NFTs NFT Galleries Notifications Profile notifications Subscribing to SNS notifications Nonce management typed data Profile Create Profile Get Profile Get Profiles Profile Statistics Default profile Recommended Profiles Who acted on a publication Set Profile Metadata Onchain Identity Profile Interests Profile Operations Link Handle To Profile Unlink Handle From Profile Report profile Profile Action History Block Profiles Unblock Profiles Publication Get publication Get publications Publication stats Publication tags Validate metadata Refresh metadata Create a post Create a post on Momoka Create a comment Create a comment on Momoka Create a mirror Create a mirror on Momoka Create a quote Create a quote on Momoka Hide publication Report publication Gated publication Publication operations Reactions Open Actions Open Actions Without A Profile Bookmarks Not Interested Collect (legacy) Protocol Get Lens Protocol Version Revenue Follow revenue Revenue from a publication Revenue from publications Search Search profiles Search publications Supported Modules Currencies Supported Open Action Modules Supported Reference Modules Supported Open Action Collect Modules Supported Follow Modules General information Why our own indexer? How do we index? How do we cache? What database do we use? What is the backend code? Why GraphQL? Modules and Open Actions Introduction to Modules Module Metadata Standard Module Verification Integrate Modules Registering a Module Creating a Follow Module Creating an Open Action (Publication Action) Smart Post End to End Guide Specification Lens contract book Suggest. The documents highlight some most of the core logic but if your looking for a more advanced view to see what the API offers you can use https://api-v2-docs.lens.xyz/. var is_hub = true; var is_hub2 = true; var is_hub_edit = true;"); 
    let request_body = format!(r#"{{"prompt": "{}"}}"#, prompt);

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let response = client.post(url)
        .headers(headers)
        .body(request_body)
        .send()
        .await?;

    println!("Status: {}", response.status());
    if response.status().is_success() {
        let response_body = response.text().await?;
        println!("Response Body: {}", response_body);
    } else {
        eprintln!("Request failed with status: {}", response.status());
    }

    Ok(())*/
    /*let client = Client::new();
    let api_key = "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"; // Replace with your OpenAI API key
    let url = "https://api.openai.com/v1/chat/completions";

    let prompt = "This is the text that need to be clean : Input:Jump to Content Guides v1 v1.1 v2 Moon (Dark Mode) Sun (Light Mode) v 2 Search Discard Submit Suggested Edits Documentation View V1 Documentation What is Lens? 🌿 What's new in Lens V2? Overview Developer Quickstart Scaffold a new Lens app Deployed Contract Addresses Claiming a profile Major Concepts Profile Handle Publication Comment Quote Mirror Open Actions Collect Action Follow Profile Manager Referrals Token Guardian SDKs React Hooks SDK v2 LensClient SDK Lens API Introduction API links Public BigQuery Advanced API docs Querying from an Application LensClient SDK Apollo client URQL Authentication Login Refresh JWT Approved Authentications Revoke Authentication Verify JWT Helpers (LensClient) Gasless & Signless Broadcast Onchain Transaction Broadcast Momoka Transaction Lens Profile Manager Relay Queues Tracking Explore Explore profiles Explore publications Feed Feed Feed highlights Latest Paid Actions Follow Follow Unfollow Set Follow Module Following Followers Is Followed By Me? Is Following Me? Health Ping Indexer Lens Transaction Status TxId to TxHash Invites Is profile already invited Invited profiles Invite a profile Media Snapshots Metadata Standards Publication metadata Profile metadata NFTs Get Users NFTs NFT Galleries Notifications Profile notifications Subscribing to SNS notifications Nonce management typed data Profile Create Profile Get Profile Get Profiles Profile Statistics Default profile Recommended Profiles Who acted on a publication Set Profile Metadata Onchain Identity Profile Interests Profile Operations Link Handle To Profile Unlink Handle From Profile Report profile Profile Action History Block Profiles Unblock Profiles Publication Get publication Get publications Publication stats Publication tags Validate metadata Refresh metadata Create a post Create a post on Momoka Create a comment Create a comment on Momoka Create a mirror Create a mirror on Momoka Create a quote Create a quote on Momoka Hide publication Report publication Gated publication Publication operations Reactions Open Actions Open Actions Without A Profile Bookmarks Not Interested Collect (legacy) Protocol Get Lens Protocol Version Revenue Follow revenue Revenue from a publication Revenue from publications Search Search profiles Search publications Supported Modules Currencies Supported Open Action Modules Supported Reference Modules Supported Open Action Collect Modules Supported Follow Modules General information Why our own indexer? How do we index? How do we cache? What database do we use? What is the backend code? Why GraphQL? Modules and Open Actions Introduction to Modules Module Metadata Standard Module Verification Integrate Modules Registering a Module Creating a Follow Module Creating an Open Action (Publication Action) Smart Post End to End Guide Specification Lens contract book Suggest. The documents highlight some most of the core logic but if your looking for a more advanced view to see what the API offers you can use https://api-v2-docs.lens.xyz/. var is_hub = true; var is_hub2 = true; var is_hub_edit = true;"; 

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(AUTHORIZATION, format!("Bearer {}", api_key).parse().unwrap());

    let request_body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [
      {
        "role": "system",
        "content": "You are a helpful assistant. Here is your instruction: Clean and summarize the following project documentation text. Remove all redundant, non-essential parts, and focus on preserving key information and concepts."
      },
      {
        "role": "user",
        "content": prompt
      }
      ]
    });

    let response = client.post(url)
        .headers(headers)
        .body(request_body.to_string())
        .send()
        .await?;
    println!("Status: {}", response.text().await?);
    */
    /*if response.status().is_success() {
        let response_text = response.text().await?;
        println!("Response: {}", response_text);
    } else {
        eprintln!("Error: {:?}", response.status());
    }
     Ok(())*/

   
    use std::io::{Write, stdout};
    let url = "https://github.com/Uniswap/docs/blob/main/docs/api/subgraph/overview.md";
    let mut website: Website = Website::new(url);



    let parsed_url = Url::parse(url)?;
    let host = parsed_url.host_str().ok_or("Invalid URL: Missing host")?;
    let project_name = host.split('.').next().ok_or("Invalid URL: Missing project name")?;
    let dir_path = format!("./{}", project_name);
    create_dir_all(&dir_path)?;
    
    println!("dir_path: {}", dir_path);
    website.scrape().await;
    let mut lock = stdout().lock();
    
    let re = Regex::new(r"\[\d+(,\d+)*\] \{.*\}")?;
    
    let re_js = Regex::new(r"!function\(\)\{.*?\}\(\)").unwrap(); // Regex to match the JavaScript function
    let re_css = Regex::new(r"\.css-\w+\{.*?\}").unwrap(); // Regex to match CSS styles
    let re_css_hover = Regex::new(r"\.css-\w+:hover\{.*?\}").unwrap(); // Regex to match CSS hover styles

for page in website.get_pages().unwrap().iter() {
    let document = Html::parse_document(&page.get_html());
    let body_selector = Selector::parse("body").unwrap();

    let mut text_content = String::new();
    for element in document.select(&body_selector) {
        let mut content = element.text().collect::<Vec<_>>().join(" ");
        content = re_js.replace_all(&content, "").to_string(); // Remove JavaScript
        content = re_css.replace_all(&content, "").to_string(); // Remove CSS styles
        content = re_css_hover.replace_all(&content, "").to_string(); // Remove CSS hover styles
        text_content.push_str(&content.trim());
    }
        let file_name = format!("{}/{}.txt", dir_path, page.get_url_final().split('/').last().unwrap_or("index"));
        let mut file = File::create(&file_name)?;
        writeln!(lock, "_______________________________________________________").unwrap();
        writeln!(lock, "{}", text_content).unwrap();
        writeln!(file, "{}", text_content)?;
    }
    println!("Done");
    Ok(()); 
    
    let client = Client::new();
    let base_url = "https://api.github.com/repos/graphprotocol/docs/contents/";
    let start_path = "website/pages/en";

    retrieve_contents(&client, base_url, start_path).await?;

    Ok(())
}

#[async_recursion]
async fn retrieve_contents(client: &Client, base_url: &str, path: &str) -> Result<(), Error> {
    let url = format!("{}{}", base_url, path);
    let resp = client.get(&url)
        .header("User-Agent", "request")
        .send()
        .await?;

    if resp.status().is_success() {
        let contents = resp.text().await?;
        let files: Value = serde_json::from_str(&contents).expect("error parsing json");

        if let Some(array) = files.as_array() {
            for file in array {
                let file_name = file["name"].as_str().unwrap_or_default();
                let file_path = file["path"].as_str().unwrap_or_default();
                let file_type = file["type"].as_str().unwrap_or_default();

                println!("{}: {}", file_type, file_path);

                if file_type == "dir" {
                    retrieve_contents(client, base_url, file_path).await?;
                }
                // Additional logic for handling files can be added here
            }
        }
    } else {
        println!("Failed to retrieve contents for path: {}", path);
    }

    Ok(())
}


    

