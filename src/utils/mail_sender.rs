use actix_web::HttpResponse;
use reqwest::Client;
use serde_json::json;
use dotenv_codegen::dotenv;

use crate::core::{error::APIError, response::ResponseBody, types::AppResult};

async fn send_email(
    email: &str,
    name: &str,
    subject: &str,
    text_content: Option<&str>,
    html_content: Option<&str>,
) -> AppResult<HttpResponse> {
    let client = Client::new();

    let mut message = json!({
      "From":{
        "Email": "lzyct@lazycatlabs.com",
        "Name": "LazyCatLabs"
      },
      "To": [
        {
          "Email": email,
          "Name": name,
        }
      ],
      "Subject": subject,
    });

    // check if text_content is provided
    if let Some(text) = text_content {
        message["TextPart"] = json!(text);
    }

    // check if html_content is provided
    if let Some(html) = html_content {
        message["HTMLPart"] = json!(html);
    }

    let body = json!({
        "Messages": [message]
    });

    client
        .post("https://api.mailjet.com/v3.1/send")
        .basic_auth(dotenv!("MAIL_API_KEY"),Some(dotenv!("MAIL_SECRET_KEY")))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|_| APIError::InternalError)?;

    Ok(ResponseBody::<()>::success(None).into())
}