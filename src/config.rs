use std::env;
pub struct Config {
    chat_id: String,
    token: String,
    repos: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        let chatid = match env::var("CHAT_ID") {
            Ok(val) => val,
            Err(_) => dotenv::var("CHAT_ID").expect("CHAT_ID is Still Empty"),
        };
        let tok_en = match env::var("BOT_TOKEN") {
            Ok(val) => val,
            Err(_) => dotenv::var("BOT_TOKEN").expect("BOT_TOKEN is still Empty"),
        };
        let getrepo = match env::var("REPO_LIST") {
            Ok(val) => val,
            Err(_) => dotenv::var("REPO_LIST").expect("REPO_LIST is still empty"),
        };
        let repo_s = getrepo
            .split(",")
            .map(|a| a.to_string())
            .collect::<Vec<String>>();
        Self {
            chat_id: chatid,
            token: tok_en,
            repos: repo_s,
        }
    }
    pub fn getchatid(&self) -> &String {
        &self.chat_id
    }
    pub fn gettoken(&self) -> &String {
        &self.token
    }
    pub fn getrepos(&self) -> &Vec<String> {
        &self.repos
    }
}
