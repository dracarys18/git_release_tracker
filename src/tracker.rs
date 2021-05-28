use pretty_bytes::converter::convert;
use serde_json::Value;
use std::fs;
use std::path::Path;
pub struct Tracker {}
impl Tracker {
    pub fn new() -> Self {
        Self {}
    }
    /// Parses the Ok(json) into readable form if the response is not empty returns None if the
    /// respone is empty.
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
    pub fn escape_html(&self, txt: &str) -> String {
        let mut tmp = String::from("");
        for i in txt.chars() {
            match i {
                '&' => tmp.push_str("&amp;"),
                '\"' => tmp.push_str("&quot;"),
                '<' => tmp.push_str("&lt;"),
                '>' => tmp.push_str("&gt;"),
                _ => tmp.push_str(&i.to_string()),
            }
        }
        return tmp;
    }
    pub fn get_assets(&self, asset_json: &Vec<serde_json::Value>) -> String {
        let mut asset_str = String::from("<strong>Downloads: </strong>\n");
        for i in asset_json {
            let download = i.get("browser_download_url").unwrap().as_str().unwrap();
            let name = i.get("name").unwrap().as_str().unwrap();
            let size = i.get("size").unwrap().as_f64().unwrap();
            let txt = format!(
                "•<a href='{}'>{}</a> (<strong>{}</strong>)\n",
                download,
                name,
                convert(size)
            );
            asset_str.push_str(&txt);
        }
        return asset_str;
    }
    /// Parses the json into telegram message in the form:-
    /// ```
    /// New <repo-name> Update is out
    /// Author: //author name who released the build
    /// Release Name: //release name
    /// Release Tag: //tag name
    /// Changelogs: //release changelog
    /// Downloads: //URLs of download assets
    /// ```
    pub fn parse_json_message(
        &self,
        json_text: serde_json::Value,
        s_filename: String,
        reponame: &str,
    ) -> (bool, String) {
        let body = json_text.get("body").unwrap().as_str().unwrap();
        let changelog;
        if body.len() != 0 {
            changelog = self.escape_html(body);
        } else {
            changelog = "<strong>No Changelogs</strong>".to_string();
        }
        let tag_name = self.escape_html(json_text.get("tag_name").unwrap().as_str().unwrap());
        let release_name = self.escape_html(json_text.get("name").unwrap().as_str().unwrap());
        let uploader_name = json_text
            .get("author")
            .unwrap()
            .get("login")
            .unwrap()
            .as_str()
            .unwrap();
        let release = json_text.get("assets").unwrap().as_array().unwrap();
        let download_text;
        if release.len() != 0 {
            download_text = self.get_assets(release);
        } else {
            download_text = "".to_string();
        }
        let path = format!("src/data/{}", s_filename);
        let mut isupdatable: bool = false;
        if !Path::new(&path).exists() {
            self.write_file(&path, &tag_name);
            isupdatable = true;
        } else {
            let current_ver = fs::read_to_string(&path).unwrap();
            let updated_ver = String::from(&tag_name);
            if current_ver != updated_ver {
                isupdatable = true;
                self.write_file(&path, &tag_name);
            } else {
                println!("We are up to date");
            }
        }
        let message = format!(
        "<strong>New <a href='https://github.com/{}/{}'>{}</a> Update is out</strong>\n<strong>Author: </strong><a href='https://github.com/{}'>{}</a>\n<strong>Release Name: </strong><code>{}</code>\n<strong>Release Tag: </strong><code>{}</code>\n<strong>Changelogs: </strong>\n<code>{}</code>\n{}#{} #{}",
        &uploader_name,
        &reponame,
        &reponame,
        &uploader_name,
        &uploader_name,
        &release_name,
        &tag_name,
        &changelog,
        &download_text,
        &tag_name,
        &reponame,
        );
        return (isupdatable, message);
    }
}
