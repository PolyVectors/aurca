use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SearchResult {
    pub description: Option<String>,
    maintainer: Option<String>,
    pub name: String,
    popularity: f32,

    #[serde(rename = "URL")]
    url: Option<String>,

    #[serde(rename = "URLPath")]
    url_path: Option<String>,

    pub version: String,
}

#[derive(Deserialize)]
pub struct SearchResponse {
    #[serde(rename = "resultcount")]
    pub result_count: usize,

    pub results: Vec<SearchResult>,

    #[serde(rename = "type")]
    variant: Option<String>,
    version: u32,
}
