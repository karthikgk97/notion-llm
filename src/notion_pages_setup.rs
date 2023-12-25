use reqwest::Client;
use std::collections::HashMap;

pub struct NotionPagesAPI{
    auth_headers: reqwest::header::HeaderMap,
    request_client: Client
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


impl NotionPagesAPI{
    pub fn new() -> Self{

        let client = reqwest::Client::new();

        let auth_headers = Self::setup_auth_headers();

        return NotionPagesAPI{
            auth_headers: auth_headers,
            request_client: client
        };
    }

    fn setup_auth_headers() -> reqwest::header::HeaderMap{
        println!("Setting up auth headers");
        let notion_api_key = std::env::var("NOTION_API_KEY").expect("Notion API Key not set in environment");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(reqwest::header::AUTHORIZATION, format!("Bearer {}", notion_api_key).parse().unwrap());
        headers.insert(reqwest::header::HeaderName::from_static("notion-version"), "2022-06-28".parse().unwrap());
        println!("Done setting up auth headers");
        return headers;
    }

    fn str_replacer(string_to_replace: &str) -> String{
        match string_to_replace{
            "heading_1" => String::from("#"),
            "heading_2" => String::from("##"),
            "bulleted_list_item" => String::from("-"),
            _ => String::from(string_to_replace)
        }
    }

    pub async fn get_children(&mut self, page_id: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>>{
        println!("Making async call to get Children for page {}", page_id);
        
        let response_url: String = format!("https://api.notion.com/v1/blocks/{}/children?page_size=100", page_id);

        let response = self.request_client.get(response_url)
        .headers(self.auth_headers.clone())
        .send();


        match response.await {
            Ok(response) => {
                if response.status().is_success(){
                    let response_body = response.text();
                    
                    match response_body.await{
                        Ok(response_body) => {
                            let json_body: serde_json::Value = serde_json::from_str(&response_body).expect("Failed to parse JSON");
                            let results = &json_body["results"];
                            let child_pages: HashMap<String, String> = results.as_array().unwrap().iter().filter_map(|x| {
                                if x["has_children"].as_bool() == Some(true) {
                                    return Some(
                                        (
                                            x["child_page"]["title"].as_str().unwrap().to_string(),
                                            x["id"].as_str().unwrap().to_string()
                                        )
                                    );
                                }
                                else{
                                    return None;
                                }
                            }).collect();

                            return Ok(child_pages);
                        }
                        Err(err) => {
                            println!("Errored due to err {}", err);
                            return Err(Box::new(err));
                        }
                    }
                }
                else{
                    return Err(format!("Received unsuccessful response status: {:?}", response.status()).into())
                }
            }
            Err(err) => {
                println!("Errored due to the error {:?}", err);
                return Err(Box::new(err));
            }
        }
    }

    pub async fn get_page_content(&mut self, page_id: &str) -> Result<String, Box<dyn std::error::Error>>{
        println!("Making async call to get page contents for page {}", page_id);
        
        let response_url: String = format!("https://api.notion.com/v1/blocks/{}/children?page_size=100", page_id); 

        let response = self.request_client.get(response_url)
        .headers(self.auth_headers.clone())
        .send();

        match response.await {
            Ok(response) => {
                if response.status().is_success(){
                    let response_body = response.text();
                    
                    match response_body.await{
                        Ok(response_body) => {
                            let json_body: serde_json::Value = serde_json::from_str(&response_body).expect("Failed to parse JSON");
                            let results = &json_body["results"];

                            let page_content_str: String = results.as_array().unwrap().iter().map(|x| {
                                let tmp_type = &x["type"];
                                let text_value = x[tmp_type.as_str().unwrap()]["rich_text"][0]["plain_text"].as_str().unwrap_or_default().to_string();

                                let result_string = format!(
                                    "{}{}\n",
                                    &Self::str_replacer(tmp_type.as_str().unwrap()),
                                    &text_value
                                );
                                return result_string;
                            }).collect();

                            return Ok(page_content_str);
                        }
                        Err(err) => {
                            println!("Errored due to err {}", err);
                            return Err(Box::new(err));
                        }
                    }
                }
                else{
                    return Err(format!("Received unsuccessful response status: {:?}", response.status()).into())
                }
            }
            Err(err) => {
                println!("Errored due to the error {:?}", err);
                return Err(Box::new(err));
            }
        }

    }
}