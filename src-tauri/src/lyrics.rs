use reqwest::Error;
use select::document::Document;
use select::predicate::{Class};

pub async fn get_song_url(query: &str) -> String {
    let song_query = query.to_lowercase().replace(" ", "+");

    let url = format!("https://www.musixmatch.com/search/{song_query}");

    let response = reqwest::get(url).await.unwrap().text().await.unwrap();

    let document = Document::from(response.as_str());
    let path = document
        .find(Class("title"))
        .take(1)
        .next()
        .unwrap()
        .attr("href")
        .unwrap();

    format!("https://www.musixmatch.com{}", path)
}

pub async fn fetch_lyrics(query: &str) -> Result<String, Error> {
    let song_url = get_song_url(query).await;

    let response = reqwest::get(song_url).await?.text().await?;
    let document = Document::from(response.as_str());

    let lyrics = document
        .find(Class("lyrics__content__ok"))
        .map(|x| x.text())
        .collect::<Vec<_>>()
        .join("\n");

    return Ok(lyrics);
}
