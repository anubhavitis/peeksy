#![allow(deprecated)]
use log::info;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use std::{fs::File, io::Read, path::PathBuf};

#[derive(Debug, Clone)]
pub struct OpenAI {
    api_key: String,
    prompt: String,
    model: String,
}

impl OpenAI {
    pub fn new(api_key: String, prompt: String, model: String) -> Self {
        Self {
            api_key,
            prompt,
            model,
        }
    }

    pub async fn get_name(&self, image_path: &PathBuf) -> String {
        info!("Getting name for image: {:?}", image_path.display());
        // Read the image file and base64-encode it
        let mut file = File::open(image_path).expect("Failed to open image file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .expect("Failed to read image file");
        let encoded_image = base64::encode(&buffer);

        // Create the JSON payload
        let payload = json!({
            "model": self.model,
            "messages": [
                    {
                        "role": "system",
                        "content": r#"You are a filename generation bot. You must return only a filename based on the attached image. No explanations.
                         No descriptions. No punctuation. No quotes. No code blocks. Just a lowercase hyphenated filename of 3 to 8 words in plain text."#
                    },
                    {
                        "role": "user",
                        "content": [
                    {
                        "type": "text",
                        "text": self.prompt
                    },
                    {
                        "type": "image_url",
                        "image_url": {
                            "url": format!("data:image/png;base64,{}", encoded_image),
                            "detail": "low"
                        }
                    }
                ]
            }
            ],
        });

        // Send the request to OpenAI API
        self.make_ai_request(&payload).await
    }

    async fn make_ai_request(&self, payload: &serde_json::Value) -> String {
        let response = reqwest::Client::new()
            .post("https://api.openai.com/v1/chat/completions")
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
            .header(CONTENT_TYPE, "application/json")
            .body(payload.to_string())
            .send()
            .await
            .expect("Failed to send request");

        // Parse and extract the filename
        let response_text = response.text().await.expect("Failed to get response text");
        let response_json: serde_json::Value =
            serde_json::from_str(&response_text).expect("Failed to parse response");

        let name = response_json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("unknown-name")
            .trim()
            .to_string();

        name
    }
}
