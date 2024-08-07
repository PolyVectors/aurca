const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NAME: &'static str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

pub fn get() -> String {
    format!("{}/{} v{}\n{}", AUTHORS, NAME, VERSION, DESCRIPTION)
}
