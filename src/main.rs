use std::collections::HashMap;
use notion_llm::notion_pages_setup::NotionPagesAPI;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let mut npi = NotionPagesAPI::new();

    let child_pages_api_call = npi.get_children("34e444a4324c4e98b5f01d0965580fb2").await;

    let child_pages: Option<HashMap<String, String>>;

    match child_pages_api_call{
        Ok(child_pages_api_call) => {
            println!("child pages are {:?}", &child_pages_api_call);
            child_pages = Some(child_pages_api_call);
            
        }
        Err(err) => {
            child_pages = None;
            println!("Errored due to {:?}", err);
        }
    }

    let mut page_contents: HashMap<String, String> = HashMap::new();

    if let Some(child_pages) = &child_pages {
        for (page_key, page_id) in child_pages.iter(){
            let content = npi.get_page_content(page_id).await;

            match content{
                Ok(content) => {
                    page_contents.insert(page_key.clone(), content);
                    // println!("Page content is {:?}", page_content);

                }
                Err(err) => {
                    println!("Errored due to {:?}", err);
                }
            }
        }
    }
    

    println!("Contents of pages are {:?}", page_contents);

    

}
