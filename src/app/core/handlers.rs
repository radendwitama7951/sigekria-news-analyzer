use crate::app::core::http_client::HttpClient;
use crate::app::core::models::NewsContent;

pub async fn analyze_news(
    url: String,
    http_client: HttpClient,
) -> Result<NewsContent, reqwest::Error> {
    let analyzed_news = NewsContent {
        id: None,
        title: "".into(),
        authors: "".into(),
        publication_date: None,
        content: None,
        url: "".into(),
        summary: None,
    };

    println!("__analyze_news__: url -> {}", url);

    // Source
    const REMOTE_SOURCE: &str = "https://main-bvxea6i-r6ymxwool6opu.us.platformsh.site/api/v0/e686f24b-88fb-4cc2-9fec-11b56c3b40fd/news-contents/parse-news-url";

    const LOCAL_SOURCE: &str =
        "http://localhost:8000/api/v0/fa160d0b-2922-496e-a7b0-abc133c48ca7/news-contents/parse-news-url";

    Ok(http_client
        .post(LOCAL_SOURCE)
        .query(&[("news_url", url)])
        .json(&analyzed_news)
        .send()
        .await?
        .json()
        .await?)
}
