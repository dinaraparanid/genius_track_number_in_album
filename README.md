**Genius Track Number Fetcher**
-------------------------------

![crates.io](https://img.shields.io/crates/v/track_album_number_genius.svg)
[![Rust](https://img.shields.io/badge/rust-1.73.0-orange.svg?logo=rust)](https://www.rust-lang.org)

## **Developer**
[Paranid5](https://github.com/dinaraparanid)

## **About Library**

Fetches track's number (starting from zero) in album by album's URL

### **Example**

```Rust
extern crate tokio;
use track_album_number_genius::get_track_number_in_album;

#[tokio::main]
async fn main() {
    assert!(
        "{}", 
        get_track_number_in_album(
            "https://genius.com/albums/The-weeknd/After-hours", 
            "Too Late"
        ).await, 
        Some(1)
    ) 
}
```
