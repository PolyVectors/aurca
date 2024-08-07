pub mod core {
    pub mod log;
    pub mod parser;

    pub mod definitions {
        pub mod cli;
        pub mod parameter;
        pub mod search_response;
    }
}

pub mod parameters {
    pub mod help;
    pub mod search;
    pub mod version;
}
