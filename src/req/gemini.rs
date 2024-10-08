use reqwest::Client;
use serde_json::Value;
use std::{env, result::Result};

pub fn get_content(v: &Value) -> Result<String, String> {
    // part.textを取得
    // 'part.text'を取得
    // println!("v: {:?}", v);
    // 値を一度に確認して、unwrapするときにエラーメッセージを指定します
    let result = v
        .get("candidates")
        .and_then(|candidates| candidates.get(0))
        .and_then(|first_candidate| first_candidate.get("content"))
        .and_then(|content| content.get("parts"))
        .and_then(|parts| parts.get(0))
        .and_then(|first_part| first_part.get("text"))
        .ok_or(format!(
            "part.text not found or not a string, error: {:?}",
            v
        ))?;

    let result = result
        .as_str()
        .expect("part.text is not a string")
        .to_string();

    // 値が見つかった場合は、文字列に変換して返します
    Ok(result)
}

pub async fn request(model: &str, body: Value) -> Result<Value, String> {
    let google_key = env::var("GOOGLE_GEMINI_API_KEY").unwrap();
    // println!("keys: {:?}", keys);

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model, google_key
    );

    // リクエストを送信
    let client = Client::new();
    let res = match client
        .post(url)
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => {
            return Err(format!("Request error: {}", err));
        }
    };

    match res.json().await {
        Ok(json) => Ok(json),
        Err(err) => Err(format!("JSON parse error: {}", err)),
    }
}
