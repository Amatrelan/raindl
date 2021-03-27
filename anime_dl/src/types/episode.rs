use std::fmt;

use super::quality::Quality;

// This is global, so what all I need?
// Provider should be able to reconstruct whole.

#[derive(Clone, PartialEq)]
pub struct Episode {
    pub url:     String,
    pub quality: Quality,
}

impl fmt::Debug for Episode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.quality, self.url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_episode_url_print_debug() {
        let test = Episode {
            url:     "https://example.com/some-link-to-website".to_string(),
            quality: Quality::P1080,
        };

        assert_eq!(
            format!("{:?}", test),
            "1080p: \"https://example.com/some-link-to-website\"".to_string()
        );
    }
}
