use std::fmt;

use super::episode::Episode;

#[derive(Debug, Clone, PartialEq)]
pub struct Anime {
    /// Name of the series ex: `Bleach`
    pub title:       String,
    /// Root url.
    pub root_url:    String,
    /// Episode count what is known, or None
    pub max_episode: Option<u32>,
    /// This shoudl be removed?
    pub qualities:   Option<Vec<Episode>>,
}

impl Default for Anime {
    fn default() -> Anime {
        Anime {
            root_url:    String::default(),
            title:       String::default(),
            max_episode: None,
            qualities:   None,
        }
    }
}

impl fmt::Display for Anime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Title: {}, Root URL: {}, Max episode: {:?}, Qualities: {:?}",
            self.title, self.root_url, self.max_episode, self.qualities
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anime_print_display() {
        let test = Anime {
            root_url:    "https://example.anime".to_string(),
            title:       "Example Anime".to_string(),
            max_episode: None,
            qualities:   None,
        };

        assert_eq!(
            format!("{}", test),
            "Title: Example Anime, Root URL: https://example.anime, Max episode: None, Qualities: None".to_string()
        )
    }
}
