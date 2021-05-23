extern crate reqwest;
use serde_json::Value;

pub struct TelegramClient<'tel> {
    token: &'tel str,
    chat_id: &'tel str,
}

impl<'tel> TelegramClient<'tel> {
    pub fn new(chat_id: &'tel str, token: &'tel str) -> Self {
        Self {
            token: token,
            chat_id: chat_id,
        }
    }
    /// Get the userid of the bot
    pub async fn get_bot_id(&self) -> String {
        let url = format!("https://api.telegram.org/bot{}/getMe", self.token);
        let req = reqwest::Client::new();
        let resp = req.post(url).send().await.unwrap();
        let json: Value = resp.json().await.unwrap();
        let bot_id = json
            .get("result")
            .unwrap()
            .get("id")
            .unwrap()
            .as_u64()
            .unwrap()
            .to_string();
        return bot_id;
    }
    pub async fn get_chat_type(&self) -> String {
        let url = format!("https://api.telegram.org/bot{}/getChat", self.token);
        let req = reqwest::Client::new();
        let param = [("chat_id", self.chat_id)];
        let resp = req.post(url).form(&param).send().await.unwrap();
        let json: Value = resp.json().await.unwrap();
        let chat_type = match json.get("result").unwrap().get("type").unwrap().as_str() {
            Some(val) => val.to_string(),
            None => "".to_string(),
        };
        return chat_type;
    }
    /// Check if the bot can pin the messages or not in the group/channel
    pub async fn can_pin_messages(&self) -> bool {
        let url = format!("https://api.telegram.org/bot{}/getChatMember", self.token);
        let req = reqwest::Client::new();
        let botid = self.get_bot_id().await;
        let param = [("chat_id", self.chat_id), ("user_id", &botid)];
        let resp = req.post(url).form(&param).send().await.unwrap();
        let json: Value = resp.json().await.unwrap();
        let chatype = self.get_chat_type().await;
        let can_pin_message;
        if &chatype != "channel" {
            can_pin_message = match json
                .get("result")
                .unwrap()
                .get("can_pin_messages")
                .unwrap()
                .as_bool()
            {
                Some(val) => val,
                None => false,
            };
        } else {
            can_pin_message = match json
                .get("result")
                .unwrap()
                .get("can_edit_messages")
                .unwrap()
                .as_bool()
            {
                Some(val) => val,
                None => false,
            };
        }
        return can_pin_message;
    }
    pub async fn send_message(&self, text: &str) {
        let params = [
            ("chat_id", self.chat_id),
            ("text", text),
            ("parse_mode", "HTML"),
            ("disable_web_page_preview", "yes"),
        ];
        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.token);
        let m = reqwest::Client::new();
        let resp = m.post(url).form(&params).send().await.unwrap();
        let status = &resp.status().as_str().to_owned();
        let json: Value = resp.json().await.unwrap();
        let msg_id = json
            .get("result")
            .unwrap()
            .get("message_id")
            .unwrap()
            .as_i64()
            .unwrap()
            .to_string();
        if status == "200" {
            if self.can_pin_messages().await {
                self.pin_message(&msg_id).await;
                println!("Message Sent and Pinned");
            } else {
                println!("Message sent");
            }
        } else {
            println!("Not sent Error Code\n{}", status);
        }
    }
    pub async fn pin_message(&self, msg_id: &str) {
        let url = format!("https://api.telegram.org/bot{}/pinChatMessage", self.token);
        let client = reqwest::Client::new();
        let form = [
            ("chat_id", self.chat_id),
            ("message_id", msg_id),
            ("disable_notification", "False"),
        ];
        client.post(url).form(&form).send().await.unwrap();
    }
}
