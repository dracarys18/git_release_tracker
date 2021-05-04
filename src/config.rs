use std::env;
pub struct Config{
    chat_id : String,
    token : String,
    repos : Vec<String>,
    git_token:String,
}

impl Config{
    pub fn new()->Self{
        let chatid = env::var("CHAT_ID").expect("CHAT_ID is not detected");
        let tok_en = env::var("BOT_TOKEN").expect("BOT_TOKEN not detected");
        let gitoken = env::var("GIT_API_TOKEN").expect("GIT_API_TOKEN not detected");
        let repo_s = vec!["dracarys18/NotKernel".to_string()];
        Config{
            chat_id:chatid,
            token:tok_en,
            repos:repo_s,
            git_token:gitoken,
        }
    }
    pub fn getchatid(&self)->&String{
        &self.chat_id   
    }
    pub fn gettoken(&self)->&String{
        &self.token
    }
    pub fn getrepos(&self)->&Vec<String>{
        &self.repos
    }
    pub fn getgitoken(&self)->&String{
        &self.git_token
    } 
}
