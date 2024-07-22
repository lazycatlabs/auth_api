use crate::camel_case_struct;

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
