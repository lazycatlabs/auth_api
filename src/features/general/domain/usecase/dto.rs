use crate::camel_case_struct;

camel_case_struct!(SendEmailParams {
  #[validate(length(min = 0, message = "Can't be empty"))]
   email: String,
  #[validate(length(min = 0, message = "Can't be empty"))]
  name: String,
  #[validate(length(min = 0, message = "Can't be empty"))]
  subject: String,
  text_content: Option<String>,
  html_content: Option<String>
});
