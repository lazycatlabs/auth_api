use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
    pub status: String,
    pub message: String,
}

impl Diagnostic {
    pub fn new(status: &str, message: &str) -> Diagnostic {
        Diagnostic {
            status: status.to_string(),
            message: message.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBodyNoData {
    pub diagnostic: Diagnostic,
}

impl ResponseBodyNoData {
    pub fn new(diagnostic: Diagnostic) -> ResponseBodyNoData {
        ResponseBodyNoData {
            diagnostic,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody<T> {
    pub diagnostic: Diagnostic,
    pub data: T,
}

impl<T> ResponseBody<T> {
    pub fn new(diagnostic: Diagnostic, data: T) -> ResponseBody<T> {
        ResponseBody {
            diagnostic,
            data,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub diagnostic: Diagnostic,
    pub data: Vec<T>,
    pub page_number: i32,
    pub page_size: i32,
    pub total: i64,
}

impl<T> Page<T> {
    pub fn new(page_number: i32, page_size: i32, total: i64, data: Vec<T>, diagnostic: Diagnostic) -> Page<T> {
        Page {
            page_number,
            page_size,
            total,
            data,
            diagnostic,
        }
    }
}
