use actix_web::HttpResponse;
use dotenv_codegen::dotenv;
use reqwest::Client;
use serde_json::json;

use crate::core::{error::APIError, response::ResponseBody, types::AppResult};
use crate::features::general::domain::usecase::dto::SendEmailParams;

async fn send_email(
    email: &str,
    name: &str,
    subject: &str,
    text_content: Option<&str>,
    html_content: Option<&str>,
) -> AppResult<HttpResponse> {
pub async fn send_email(params: SendEmailParams) -> AppResult<HttpResponse> {
    let client = Client::new();

    let mut message = json!({
      "From":{
        "Email": "lzyct@lazycatlabs.com",
        "Name": "LazyCatLabs"
      },
      "To": [
        {
          "Email": &params.email,
          "Name": &params.name,
        }
      ],
      "Subject": &params.subject,
    });

    // check if text_content is provided
    if let Some(text) = &params.text_content {
        message["TextPart"] = json!(text);
    }

    // check if html_content is provided
    if let Some(html) = &params.html_content {
        message["HTMLPart"] = json!(html);
    }

    let body = json!({
        "Messages": [message]
    });

    let result = client
        .post("https://api.mailjet.com/v3.1/send")
        .basic_auth(dotenv!("MAIL_API_KEY"), Some(dotenv!("MAIL_SECRET_KEY")))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|_| APIError::InternalError)?;

    println!("Email sent to {}", &params.email);
    println!("Body: {:?}", body);
    println!("Response: {:?}", result);
    if result.status().is_client_error() {
        return Err(APIError::BadRequest {
            message: format!("Failed to send email to {}", &params.email).to_string(),
        });
    }

    Ok(ResponseBody::<()>::success(None).into())
}
