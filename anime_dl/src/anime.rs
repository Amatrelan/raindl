use std::fmt;

// This is global, so what all I need?
// Provider should be able to reconstruct whole.
#[derive(Clone, PartialEq)]
pub enum Quality {
    P360,
    P480,
    P720,
    P1080,
    Unknown,
}
impl fmt::Debug for Quality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Quality::P360 => "360p".to_string(),
            Quality::P480 => "480p".to_string(),
            Quality::P720 => "720p".to_string(),
            Quality::P1080 => "1080p".to_string(),
            Quality::Unknown => "Unknown".to_string(),
        };

        write!(f, "{}", value)
    }
}

#[derive(Clone, PartialEq)]
pub struct QualityUrl {
    pub url: String,
    pub quality: Quality,
}

impl fmt::Debug for QualityUrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.quality, self.url)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Anime {
    /// Name of the series ex: `Bleach`
    pub title: String,
    /// Root url.
    pub root_url: String,
    /// Episode count what is known, or None
    pub max_episode: Option<u32>,

    pub qualities: Option<Vec<QualityUrl>>,
}

impl fmt::Display for Anime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Title: {}\nRoot URL: {}\nMax episode: {:?}\nQualities: {:?}\n------",
            self.title, self.root_url, self.max_episode, self.qualities
        )
    }
}

impl Default for Anime {
    fn default() -> Anime {
        Anime {
            root_url: String::default(),
            title: String::default(),
            max_episode: None,
            qualities: None,
        }
    }
}
