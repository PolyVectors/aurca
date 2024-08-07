const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NAME: &'static str = env!("CARGO_PKG_NAME");

pub fn get() -> String {
    format!("{} v{}", NAME, VERSION)
}
