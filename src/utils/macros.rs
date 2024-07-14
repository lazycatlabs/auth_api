#[macro_export]
macro_rules! camel_case_struct {
    ($name:ident { $( $field:ident: $type:ty ),* }) => {
        #[derive(Serialize, Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            $( pub $field: $type ),*
        }
    };
}
