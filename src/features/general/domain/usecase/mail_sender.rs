use dotenv_codegen::dotenv;
use reqwest::Client;
use serde_json::json;

use crate::camel_case_struct;
use crate::core::{error::APIError, types::AppResult};

camel_case_struct!(SendEmailParams {
  #[validate(
    required(message = "field is required"),
    length(min = 1, message = "Can't be empty"),
  )]
   email: Option<String>,
  #[validate(
    required(message = "field is required"),
    length(min = 1, message = "Can't be empty"),
  )]
  name: Option<String>,
  #[validate(
    required(message = "field is required"),
    length(min = 1, message = "Can't be empty"),
  )]
  subject: Option<String>,
  text_content: Option<String>,
  html_content: Option<String>
});

pub async fn send_email(params: SendEmailParams) -> AppResult<String> {
    let client = Client::new();

    let mut message = json!({
      "From":{
        "Email": "lzyct@lazycatlabs.com",
        "Name": "Lazycat Labs"
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

    let email = &params.email.unwrap_or("Unknown".to_string());
    println!("Email sent to {}", &email);
    println!("Body: {:#?}", body);
    println!("Response: {:#?}", result);
    if result.status().is_client_error() {
        return Err(APIError::BadRequest {
            message: format!("Failed to send email to {}", &email).to_string(),
        });
    }

    Ok("Email sent successfully".to_string())
}
