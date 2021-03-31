mod gogoplay;
pub use gogoplay::GoGoPlay;

use crate::error::Result;
use crate::types::Anime;

/// Is core of all providers, it's required to implement for all providers.
pub trait Provider {
    /// Returns anime episode's url for video.
    /// # Errors
    /// This can return multiple different error types. Error is Boxed for now.
    fn anime_episode(&self, anime: Anime, episode: u32) -> Result<Anime>;
    /// String search anime, for ex. Bleach "should" return one for Bleach and all movies and ovas.
    /// # Errors
    /// This can return multiple different error types. Error is Boxed for now.
    fn search_anime(&self, what: &str) -> Result<Vec<Anime>>;
}
