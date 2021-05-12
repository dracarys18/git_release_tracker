extern crate reqwest;
mod tracker;
mod config;
mod telegram;
use reqwest::header::USER_AGENT;
use std::thread::sleep;
use std::time::Duration;
use tracker::Tracker;
use telegram::TelegramClient;
use config::Config;
#[tokio::main]
async fn main() {
    loop{
        run().await;
        sleep(Duration::from_secs(120));
    }
}
async fn run(){
    let configs = Config::new();
    let track = Tracker::new();
    let client = reqwest::Client::new();
    for i in configs.getrepos(){
        let url = format!("https://api.github.com/repos/{}/releases/latest",i);
        let req = client.get(url)
                .header(USER_AGENT,"Tracker")
                .send()
                .await
                .unwrap();
        let status = &req.status();
        let status_str = status.as_str();
        let content = req.text().await.ok();
        let json_text = track.parse_resp_json(content,status_str);
        let chat_id = configs.getchatid().to_owned();
        let token = configs.gettoken().to_owned();
        let tgclient = TelegramClient::new(&chat_id, &token);
        let reponame = i.split("/").map(|a| a).collect::<Vec<&str>>()[1];
        match json_text{
            Some(text)=>{
                let filename = i.replace("/", "_");
                let (updatable,message) = track.parse_json_message(text,filename,reponame);
                if updatable{
                    tgclient.post_to_telegram(&message).await;
                }
            },
            None=>println!("Failed to send the message")//track.post_to_telegram("Failed to get the message".to_string(),&token,&chat_id),
        } 
    }  
}
