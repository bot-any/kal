use darling::FromField;

#[derive(FromField)]
#[darling(attributes(argument))]
pub struct ArgumentConfig {
    pub name: String,
    #[darling(default)]
    pub take_rest: bool,
}
