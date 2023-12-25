use std::collections::HashMap;
use env_logger::Builder;
use notion_llm::notion_pages_setup::NotionPagesAPI;
use notion_llm::qdrantdb::QdrantDBStruct;

#[tokio::main]
async fn main() {
    Builder::new().filter_level(log::LevelFilter::Info).init();
    println!("Hello, world!");

    let qdb = QdrantDBStruct::new(None, None);

    let _ = qdb.create_collection(&String::from("notion-llm-cooking")).await;

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

    // let mut qdrant_docs: Vec<String> = Vec::new();
    // let mut qdrant_metadata: Vec<HashMap<String, String>> = Vec::new();

    if let Some(child_pages) = &child_pages {
        for (page_key, page_id) in child_pages.iter(){
            let content = npi.get_page_content(page_id).await;

            match content{
                Ok(content) => {
                    page_contents.insert(page_key.clone(), content.clone());
                    let qdrant_docs = vec![content];
                    let qdrant_metadata = vec![HashMap::from([("dish_name".to_string(), page_key.to_string())])];
                    let id_for_stuff = QdrantDBStruct::create_ids(vec![page_key.to_string()]);

                    let _ = qdb.add_stuff_to_collection(
                        "notion-llm-cooking",
                        qdrant_docs.clone(),
                        id_for_stuff,
                        qdrant_metadata.clone()
                    ).await;

                }
                Err(err) => {
                    println!("Errored due to {:?}", err);
                }
            }
        }
    }
    
    println!("Contents of pages are {:?}", page_contents);
}
