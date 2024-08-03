#[derive(PartialEq)]
pub enum ComponentType {
    Option,
    Command,
}

pub struct Component<'a> {
    pub name: &'a str,
    pub alias: &'a str,
    pub description: &'a str,
    pub variant: ComponentType,
}

impl <'a> Component <'a> {
    pub const fn new(
        name: &'a str,
        alias: &'a str,
        description: &'a str,
        variant: ComponentType
    ) -> Self {
        Self { name, alias, description, variant }
    }
}
