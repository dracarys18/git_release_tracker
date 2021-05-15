extern crate reqwest;
mod config;
mod telegram;
mod tracker;
use config::Config;
use reqwest::header::USER_AGENT;
use std::thread::sleep;
use std::time::Duration;
use telegram::TelegramClient;
use tracker::Tracker;
#[tokio::main]
async fn main() {
    loop {
        run().await;
        sleep(Duration::from_secs(120));
    }
}
async fn run() {
    let configs = Config::new();
    let chat_id = configs.getchatid().to_owned();
    let token = configs.gettoken().to_owned();
    let track = Tracker::new();
    let client = reqwest::Client::new();
    let tgclient = TelegramClient::new(&chat_id, &token);
    for i in configs.getrepos() {
        let url = format!("https://api.github.com/repos/{}/releases/latest", i);
        let req = client
            .get(url)
            .header(USER_AGENT, "Tracker")
            .send()
            .await
            .unwrap();
        let status = &req.status().as_str().to_owned();
        let content = req.text().await.ok();
        let json_text = track.parse_resp_json(content, status);
        let reponame = match i.split("/").map(|a| a).collect::<Vec<&str>>().get(1){
            Some(val)=>*val,
            None=>panic!("The repo name is empty that means you havent defined it in proper format\nThe format is <username>/<reponame>"),
        };
        match json_text {
            Some(text) => {
                let filename = i.replace("/", "_");
                let (updatable, message) = track.parse_json_message(text, filename, reponame);
                if updatable {
                    tgclient.send_message(&message).await;
                }
            }
            None => println!("Failed to send the message"), //track.post_to_telegram("Failed to get the message".to_string(),&token,&chat_id),
        }
    }
}
