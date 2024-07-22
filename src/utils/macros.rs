#[macro_export]
macro_rules! camel_case_struct {
    ($name:ident { $( $field:ident: $type:ty ),* }) => {
        #[derive(serde::Serialize, serde::Deserialize, Debug,Clone)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            $( pub $field: $type ),*
        }
    };

      (
          $name:ident {
              $(
                  $(#[$field_attr:meta])*
                  $field:ident: $type:ty
              ),+ $(,)?
          }
      ) => {
          #[derive(serde::Serialize, serde::Deserialize, validator::Validate, Debug,Clone)]
          #[serde(rename_all = "camelCase")]
          pub struct $name {
              $(
                  $(#[$field_attr])*
                  pub $field: $type,
              )+
          }
      };

}
