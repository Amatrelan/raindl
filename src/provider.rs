pub mod gogoplay;

use crate::anime::Anime;

/// Is core of all providers, it's required to implement for all providers.
pub trait Provider {
    /// Returns anime episode's url for video.
    fn anime_episode(&self, anime: Anime, episode: u32) -> Result<Anime, &'static str>;
    /// String search anime, for ex. Bleach "should" return one for Bleach and all movies and ovas.
    fn search_anime(&self, what: &String) -> Result<Vec<Anime>, &'static str>;
}