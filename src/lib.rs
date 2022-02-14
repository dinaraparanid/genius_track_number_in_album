extern crate html2text;
extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};

#[inline]
fn get_track_number_from_doc(doc: &str, track_title: &str) -> Option<usize> {
    html2text::from_read(
        Html::parse_document(doc)
            .select(&Selector::parse("div[class=chart_row-content]").unwrap())
            .fold(String::new(), |acc, e| {
                format!(
                    "{}{}\n",
                    acc,
                    e.text()
                        .fold(String::new(), |acc, x| format!("{}{}", acc, x))
                        .replace(" Lyrics", "")
                        .trim(),
                )
            })
            .as_bytes(),
        1000,
    )
    .split("\n")
    .position(|x| x.contains(track_title))
}

/// Gets track's number (starting from zero) in album by album's URL asynchronously
///
/// # Example
///
/// ```Rust
/// #[tokio::main]
/// async fn main() {
///     assert!(
///         "{}",
///         get_track_number_in_album(
///             "https://genius.com/albums/The-weeknd/After-hours",
///             "Alone Again"
///         )
///         .await,
///         Some(0)
///     )
/// }
/// ```

#[inline]
pub async fn get_track_number_in_album(album_url: &str, track_title: &str) -> Option<usize> {
    get_track_number_from_doc(
        match reqwest::get(album_url).await {
            Ok(x) => x,
            Err(_) => return None,
        }
        .text()
        .await
        .unwrap()
        .as_str(),
        track_title,
    )
}

/// **Blocks thread** and gets track's number (starting from zero) in album by album's URL
///
/// # Example
///
/// ```Rust
/// fn main() {
///     assert!(
///         "{}",
///         get_track_number_in_album_blocking(
///             "https://genius.com/albums/The-weeknd/After-hours",
///             "Alone Again"
///         ),
///         Some(0)
///     )
/// }
/// ```

#[inline]
pub fn get_track_number_in_album_blocking(url: &str, track_title: &str) -> Option<usize> {
    get_track_number_from_doc(
        match reqwest::blocking::get(url) {
            Ok(x) => x,
            Err(_) => return None,
        }
        .text()
        .unwrap()
        .as_str(),
        track_title,
    )
}
