use crate::{
    camel_case_struct,
    core::constants::{MESSAGE_SUCCESS, STATUS_SUCCESS},
};
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

camel_case_struct!(Diagnostic {
    status: String,
    message: String
});

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
pub struct ResponseBody<T> {
    pub diagnostic: Diagnostic,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> From<ResponseBody<T>> for HttpResponse
where
    T: Serialize,
{
    fn from(val: ResponseBody<T>) -> Self {
        HttpResponse::Ok().json(val)
    }
}

impl<T> ResponseBody<T> {
    pub fn new(diagnostic: Diagnostic, data: Option<T>) -> ResponseBody<T> {
        let data = data;
        ResponseBody { diagnostic, data }
    }

    pub fn success(data: Option<T>) -> ResponseBody<T> {
        let data = data;
        ResponseBody {
            diagnostic: Diagnostic::new(STATUS_SUCCESS, MESSAGE_SUCCESS),
            data,
        }
    }
    pub fn success_with_message(data: Option<T>, message: &str) -> ResponseBody<T> {
        let data = data;
        ResponseBody {
            diagnostic: Diagnostic::new(STATUS_SUCCESS, message),
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

// impl<T> Page<T> {
//     pub fn new(page_number: i32, page_size: i32, total: i64, data: Vec<T>, diagnostic: Diagnostic) -> Page<T> {
//         Page {
//             page_number,
//             page_size,
//             total,
//             data,
//             diagnostic,
//         }
//     }
// }
