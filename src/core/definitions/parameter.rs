#[derive(PartialEq)]
pub enum ParameterType {
    Option,
    Command,
}

pub struct Parameter<'a> {
    pub name: &'a str,
    pub alias: &'a str,
    pub description: &'a str,
    pub variant: ParameterType,
}

impl<'a> Parameter<'a> {
    pub const fn new(
        name: &'a str,
        alias: &'a str,
        description: &'a str,
        variant: ParameterType,
    ) -> Self {
        Self {
            name,
            alias,
            description,
            variant,
        }
    }
}
