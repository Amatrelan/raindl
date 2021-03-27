use std::fmt;

#[derive(Clone, PartialEq, PartialOrd)]
pub enum Quality {
    Unknown,
    P720,
    P360,
    P480,
    P1080,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_print() {
        assert_eq!(format!("{:?}", Quality::Unknown), "Unknown".to_string());
        assert_eq!(format!("{:?}", Quality::P1080), "1080p".to_string());
        assert_eq!(format!("{:?}", Quality::P720), "720p".to_string());
        assert_eq!(format!("{:?}", Quality::P480), "480p".to_string());
        assert_eq!(format!("{:?}", Quality::P360), "360p".to_string());
    }
}
