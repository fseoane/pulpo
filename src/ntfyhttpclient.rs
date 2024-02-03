use serde::Deserialize;
// use reqwest::blocking::Response;
// use reqwest::blocking::get;
// use reqwest::Error;
use reqwest;

#[derive(Debug, Deserialize)]
struct Message {
    tags: String,
    title: String,
    message: String,
}

pub async fn print_messages() {
    let url_to_request = format!("{}/{}/json","https://yecla.mooo.com:20590","Alerts,Backup,General,Deployment,MediaClassifier");
    // let response = reqwest::blocking::get(url_to_request.as_str()).unwrap();
    
    // let var: Message = response.json().unwrap();
    // println!("{:?}", var);
    
    let resp = match reqwest::get(url_to_request.as_str()).await {
        Ok(resp) => resp.text().await.unwrap(),
        Err(err) => panic!("Error: {}", err)
    };
    println!("{}", resp)

}