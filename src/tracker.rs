use serde_json::Value;
use std::fs;
use std::path::Path;
pub struct Tracker {}
impl Tracker {
    pub fn new() -> Self {
        Self {}
    }
    pub fn parse_resp_json(&self, txt: Option<String>, status: &str) -> Option<serde_json::Value> {
        if status != "200" {
            return None;
        } else {
            let json_text: Value = serde_json::from_str(&txt.unwrap()).unwrap();
            return Some(json_text);
        }
    }
    pub fn write_file(&self, path: &str, name: &str) {
        fs::write(&path, &name).expect("Unable to write the file");
    }
    pub fn parse_json_message(
        &self,
        json_text: serde_json::Value,
        s_filename: String,
        reponame: &str,
    ) -> (bool, String) {
        let changelog = json_text.get("body").unwrap().as_str().unwrap();
        let tag_name = json_text.get("tag_name").unwrap().as_str().unwrap();
        let release_name = json_text.get("name").unwrap().as_str().unwrap();
        let releases = json_text.get("assets").unwrap().get(0).unwrap();
        let download_url = releases
            .get("browser_download_url")
            .unwrap()
            .as_str()
            .unwrap();
        let file_name = releases.get("name").unwrap().as_str().unwrap();
        let uploader_name = releases
            .get("uploader")
            .unwrap()
            .get("login")
            .unwrap()
            .as_str()
            .unwrap();
        let path = format!("src/data/{}", s_filename);
        let mut isupdatable: bool = false;
        if !Path::new(&path).exists() {
            self.write_file(&path, &tag_name);
            isupdatable = true;
        } else {
            let current_ver = fs::read_to_string(&path).unwrap();
            let updated_ver = String::from(tag_name);
            if current_ver != updated_ver {
                isupdatable = true;
                self.write_file(&path, &tag_name);
            } else {
                println!("We are up to date");
            }
        }
        let message = format!(
        "<strong>New <a href='https://github.com/{}/{}'>{}</a> Update is out</strong>\n<strong>\rAuthor:</strong><a href='https://github.com/{}'>{}</a>\n<strong>Release Name:</strong><code>{}</code>\n<strong>Release Tag:</strong><code>{}</code>\n<strong>Changelogs:</strong>\n<code>{}</code>\n<strong>Download:</strong><a href='{}'>{}</a>\n#{} #{}",
        &uploader_name,
        &reponame,
        &reponame,
        &uploader_name,
        &uploader_name,
        &release_name,
        &tag_name,
        &changelog,
        &download_url,
        &file_name,
        &tag_name,
        &reponame,
        );
        return (isupdatable, message);
    }
}
