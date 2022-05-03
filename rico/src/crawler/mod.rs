use select::document::Document;
use select::{node::Node, predicate::Attr};

use crate::error::Errable;

pub mod fixture;
pub mod league;
pub mod season;

pub fn create_client() -> reqwest::Client {
    reqwest::Client::new()
}

const FBREF_HOMEPAGE: &str = "https://www.fbref.com";

pub fn fbref_local(href: &str) -> String {
    format!("{FBREF_HOMEPAGE}{href}")
}

pub async fn request_page(
    client: &reqwest::Client,
    url: &str,
) -> Errable<Document> {
    let val = client
        .get(url)
        .header(
            "User-Agent",
            env!(
                "USER_AGENT",
                "\nREQUIRED
    ==> the environment variable `USER_AGENT` is not set,
    ==> set it to continue, either in a .env file or
    ==> within the terminal like so `export USER_AGENT=\"{the user_agent}\",
    ==> without the braces `{}`, but the quotes is required \"\""
            ),
        )
        .send()
        .await?
        .text()
        .await?;

    Ok(Document::from(val.as_str()))
}

pub fn data_stat(node: &Node, data_stat: &str) -> String {
    node.find(Attr("data-stat", data_stat))
        .next()
        .unwrap()
        .text()
        .trim()
        .into()
}
