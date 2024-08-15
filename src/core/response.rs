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

camel_case_struct!(PageInfo {
    page_number: i64,
    per_page: i64,
    last_page: i64,
    total: i64
});

impl Diagnostic {
    pub fn new(status: &str, message: &str) -> Diagnostic {
        Diagnostic {
            status: status.to_string(),
            message: message.to_string(),
        }
    }
}

impl PageInfo {
    pub fn new(page_number: i64, per_page: i64, total: i64) -> PageInfo {
        PageInfo {
            page_number,
            per_page,
            last_page: (total as f64 / per_page as f64).ceil() as i64,
            total,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody<T> {
    pub diagnostic: Diagnostic,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<PageInfo>,
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
        ResponseBody {
            diagnostic,
            data,
            page: None,
        }
    }

    pub fn success_pagination(data: Option<T>, page: PageInfo) -> ResponseBody<T> {
        ResponseBody {
            diagnostic: Diagnostic::new(STATUS_SUCCESS, MESSAGE_SUCCESS),
            data,
            page: Some(PageInfo::new(page.page_number, page.per_page, page.total)),
        }
    }

    pub fn success(data: Option<T>) -> ResponseBody<T> {
        let data = data;
        ResponseBody {
            diagnostic: Diagnostic::new(STATUS_SUCCESS, MESSAGE_SUCCESS),
            data,
            page: None,
        }
    }
    pub fn success_with_message(data: Option<T>, message: &str) -> ResponseBody<T> {
        let data = data;
        ResponseBody {
            diagnostic: Diagnostic::new(STATUS_SUCCESS, message),
            data,
            page: None,
        }
    }
}
