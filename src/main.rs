extern crate reqwest;
mod tracker;
mod config;
use reqwest::header::USER_AGENT;
use tracker::Tracker;
use config::Config;
fn main() {
    run();
}
fn run(){
    let configs = Config::new();
    let track = Tracker::new();
    let client = reqwest::blocking::Client::new();
    for i in configs.getrepos(){
        let url = format!("https://api.github.com/repos/{}/releases/latest",i);
        let access_token = configs.getgitoken();
        let token_header = format!("token{}",access_token);
        let req = client.get(url)
                .header(USER_AGENT,"Tracker")
                .header("Authorization",&token_header)
                .send()
                .unwrap();
        let status = &req.status();
        let status_str = status.as_str();
        let content = req.text().ok();
        let json_text = track.parse_resp_json(content,status_str);
        let chat_id = configs.getchatid().to_owned();
        let token = configs.gettoken().to_owned();
        match json_text{
            Some(text)=>{
                let filename = i.replace("/", "_");
                let (updatable,message) = track.parse_json_message(text,filename);
                if updatable{
                    track.post_to_telegram(&message,&token,&chat_id);
                }
            },
            None=>println!("Failed to send the message")//track.post_to_telegram("Failed to get the message".to_string(),&token,&chat_id),
        } 
    }  
}
