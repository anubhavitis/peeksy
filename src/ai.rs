use dotenv::dotenv;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use std::{fs::File, io::Read, path::PathBuf};

pub struct OpenAI {
    api_key: String,
    prompt: String,
}

impl OpenAI {
    pub fn new() -> Self {
        // Load .env file
        dotenv().ok();

        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
        let prompt_file_path =
            std::env::var("OPENAI_PROMPT_FILE").expect("OPENAI_PROMPT_FILE must be set");
        let prompt_file = std::path::Path::new(&prompt_file_path);
        let prompt = std::fs::read_to_string(prompt_file).expect("Failed to read prompt file");
        Self { api_key, prompt }
    }

    pub async fn get_name(&self, image_path: &PathBuf) -> String {
        dbg!(&image_path.display());
        // Read the image file and base64-encode it
        let mut file = File::open(image_path).expect("Failed to open image file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .expect("Failed to read image file");
        let encoded_image = base64::encode(&buffer);

        // Create the JSON payload
        let payload = json!({
            "model": "gpt-4.1",
            "messages": [{
                "role": "user",
                "content": [
                    { "type": "text", "text": self.prompt },
                    {
                        "type": "image_url",
                        "image_url": {
                            "url": format!("data:image/png;base64,{}", encoded_image),
                            "detail": "low"
                        }
                    }
                ]
            }],
        });

        // Send the request to OpenAI API
        self.make_ai_request(&payload).await
    }

    async fn make_ai_request(&self, payload: &serde_json::Value) -> String {
        println!("making request to openai");
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

        dbg!(&response_json);
        let name = response_json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("unknown-name")
            .trim()
            .to_string();

        name
    }
}
