use log::debug;
use scraper::{Html, Selector};

use super::Provider;
use crate::error::Result;
use crate::types::{Anime, Episode, Quality};

pub struct GoGoPlay {
    base_url:          String,
    download_base_url: String,
}

fn get_title_from_url(url: &str) -> Result<String> {
    debug!("Url: {}", url);

    let start = &url.rfind('/');
    let end = &url.rfind("-episode");

    debug!("Begin: {:?}, End: {:?}", start, end);

    if let Some(begin) = start {
        if let Some(ending) = end {
            let name = url
                .get(*begin + 1..*ending)
                .expect("There is some error while getting name.")
                .to_string();
            debug!("Name: {}", name);
            return Ok(name);
        }
    };

    panic!("Could not parse name")
}

impl Default for GoGoPlay {
    fn default() -> Self {
        Self {
            base_url:          String::from("https://gogo-play.net"),
            download_base_url: String::from("https://gogo-stream.com/download"),
        }
    }
}

impl Provider for GoGoPlay {
    // TODO: Fix this, too complex and annoying to read.
    #[allow(clippy::cognitive_complexity)]
    fn anime_episode(&self, mut anime: Anime, episode: u32) -> Result<Anime> {
        debug!("Looking episode {} for {:?}", episode, anime);
        let mut links: Vec<String> = Vec::new();

        let base_url_len = self.base_url.len();
        let base_url = anime.root_url.get(base_url_len..).expect("Error while getting root url").to_owned();
        let url = format!("{}{}-episode-{}", self.base_url, base_url, episode);
        debug!("Fetching page with url {}", url);
        let resp = reqwest::blocking::get(url.as_str())?.text()?;

        let document = Html::parse_document(resp.as_str());
        let selector = Selector::parse("iframe").expect("Failed to build 'iframe' selector");

        for element in document.select(&selector) {
            let source = match &element.value().attr("src") {
                Some(v) => v.get(2..).expect("Error while getting src block").to_owned(),
                None => panic!("Failed to get element src"),
            };
            debug!("source: {:?}", source);

            let begin = match &source.find("streaming.php") {
                Some(v) => v.to_owned(),
                None => panic!("Failed to find streaming.php"),
            };

            let e = begin + "streaming.php".len();
            let end = &source.get(e..).expect("Error while getting end part of link");

            let vid_download_url = format!("{}{}", self.download_base_url, end);
            debug!("Video download url: {}", vid_download_url);

            let d_resp = reqwest::blocking::get(vid_download_url.as_str())?.text()?;

            let d_document = Html::parse_document(d_resp.as_str());

            let d_selector = Selector::parse("div.dowload").expect("Failed to build 'div.download' selector");
            debug!("d_selector: {:?}", d_selector);

            let d_link_selector = Selector::parse("a").expect("Failed to build 'a' selector.");
            debug!("d_link_selector: {:?}", d_link_selector);

            for d_element in d_document.select(&d_selector) {
                for link in d_element.select(&d_link_selector) {
                    if let Some(v) = link.value().attr("href") {
                        links.push(v.to_owned());
                    }
                }
            }
        }

        debug!("Links len: {}", links.len());
        let mut qualities: Vec<Episode> = vec![];
        for each in &links {
            debug!("Link: {:?}", each);
            if each.contains("1080p") {
                debug!("Episode url: {}, Quality {:?}", each, Quality::P1080);
                qualities.push(Episode {
                    quality: Quality::P1080,
                    url:     each.into(),
                });
                continue;
            } else if each.contains("HDP") {
                debug!("Episode url: {}, Quality {:?}", each, Quality::P1080);
                qualities.push(Episode {
                    quality: Quality::P1080,
                    url:     each.into(),
                });
                continue;
            } else if each.contains("720p") {
                debug!("Episode url: {}, Quality {:?}", each, Quality::P720);
                qualities.push(Episode {
                    quality: Quality::P720,
                    url:     each.into(),
                });
                continue;
            } else if each.contains("480p") {
                debug!("Episode url: {}, Quality {:?}", each, Quality::P480);
                qualities.push(Episode {
                    quality: Quality::P480,
                    url:     each.into(),
                });
                continue;
            } else if each.contains("360p") {
                debug!("Episode url: {}, Quality {:?}", each, Quality::P360);
                qualities.push(Episode {
                    quality: Quality::P360,
                    url:     each.into(),
                });
                continue;
            }

            debug!("Episode url: {}, Quality {:?}", each, Quality::Unknown);
            qualities.push(Episode {
                quality: Quality::Unknown,
                url:     each.into(),
            });
        }

        anime.qualities = Some(qualities);
        Ok(anime)
    }

    fn search_anime(&self, what: &str) -> Result<Vec<Anime>> {
        let search_url: String = format!("{}/search.html?keyword={}", self.base_url, what);
        let resp = reqwest::blocking::get(search_url.as_str())?.text()?;
        let document = Html::parse_document(resp.as_str());
        let selector = Selector::parse("ul.listing").expect("Failed to create 'ul.listing' selector");
        let video_selector = Selector::parse("a").expect("Failed to create 'a' selector.");
        let mut animes: Vec<Anime> = Vec::new();

        for element in document.select(&selector) {
            for video in element.select(&video_selector) {
                let mut tmp = Anime::default();

                let link = match video.value().attr("href") {
                    Some(val) => val,
                    None => panic!("Could not find href attribute"),
                };

                let episode_str = "-episode-";
                let last_dash = match link.rfind(episode_str) {
                    Some(val) => val,
                    None => panic!("Could not find -episode-"),
                };

                let mut episode_number = link
                    .get(last_dash + episode_str.len()..)
                    .expect("Error while getting episode number")
                    .to_owned();

                debug!("Episode number from title: {}", episode_number);
                if episode_number.parse::<u32>().is_err() {
                    episode_number = episode_number.replace('-', "");
                }

                let number = episode_number.parse::<u32>()?;

                tmp.max_episode = Some(number);

                tmp.root_url = format!(
                    "{}{}",
                    self.base_url,
                    link.get(..last_dash).expect("Error while formatting root url").to_string()
                );

                tmp.title = get_title_from_url(&link)?;

                if let Ok(val) = episode_number.to_string().parse::<u32>() {
                    debug!("val: {}", val);
                    tmp.max_episode = Some(val)
                }

                animes.push(tmp);
            }
        }

        Ok(animes)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fetch_animes() {
        let gogo = GoGoPlay::default();

        assert!(gogo.search_anime(&"Bleach".to_string()).is_ok());
        assert!(gogo.search_anime(&"Naruto".to_string()).is_ok());
    }

    #[test]
    fn test_anime_struct_data() {
        let gogo = GoGoPlay::default();

        assert_eq!(
            gogo.search_anime(&"Bleach diamond dust".to_string()).unwrap(),
            vec![
                Anime {
                    title:       "bleach-movie-2-the-diamond-dust-rebellion".to_string(),
                    root_url:    "https://gogo-play.net/videos/bleach-movie-2-the-diamond-dust-rebellion".to_string(),
                    max_episode: Some(1),
                    qualities:   None,
                },
                Anime {
                    title:       "bleach-the-movie-2-the-diamonddust-rebellion-dub".to_string(),
                    root_url:    "https://gogo-play.net/videos/bleach-the-movie-2-the-diamonddust-rebellion-dub".to_string(),
                    max_episode: Some(1),
                    qualities:   None,
                }
            ]
        );
    }

    #[test]
    fn test_anime_struct_data_slime() {
        let gogo = GoGoPlay::default();

        assert_eq!(
            gogo.search_anime(&"slime".to_string()).unwrap(),
            vec![
                Anime {
                    title:       "tensei-shitara-slime-datta-ken".to_string(),
                    root_url:    "https://gogo-play.net/videos/tensei-shitara-slime-datta-ken".to_string(),
                    max_episode: Some(249),
                    qualities:   None,
                },
                Anime {
                    title:       "tensei-shitara-slime-datta-ken-ova".to_string(),
                    root_url:    "https://gogo-play.net/videos/tensei-shitara-slime-datta-ken-ova".to_string(),
                    max_episode: Some(5),
                    qualities:   None,
                },
                Anime {
                    title:       "tensei-shitara-slime-datta-ken-dub".to_string(),
                    root_url:    "https://gogo-play.net/videos/tensei-shitara-slime-datta-ken-dub".to_string(),
                    max_episode: Some(249),
                    qualities:   None,
                },
                Anime {
                    title:       "tensei-shitara-slime-datta-ken-ova-dub".to_string(),
                    root_url:    "https://gogo-play.net/videos/tensei-shitara-slime-datta-ken-ova-dub".to_string(),
                    max_episode: Some(5),
                    qualities:   None,
                },
                Anime {
                    title:       "tensei-shitara-slime-datta-ken-2nd-season".to_string(),
                    root_url:    "https://gogo-play.net/videos/tensei-shitara-slime-datta-ken-2nd-season".to_string(),
                    max_episode: Some(12),
                    qualities:   None,
                },
                Anime {
                    title:       "tensei-shitara-slime-datta-ken-2nd-season-dub".to_string(),
                    root_url:    "https://gogo-play.net/videos/tensei-shitara-slime-datta-ken-2nd-season-dub".to_string(),
                    max_episode: Some(4),
                    qualities:   None,
                }
            ]
        );
    }

    #[test]
    fn test_anime_episode() {
        let gogo = GoGoPlay::default();
        let test_anime = Anime {
            title:       "bleach-movie-2-the-diamond-dust-rebellion".to_string(),
            root_url:    "https://gogo-play.net/videos/bleach-movie-2-the-diamond-dust-rebellion".to_string(),
            max_episode: Some(1),
            qualities:   None,
        };

        assert!(gogo.anime_episode(test_anime, 1).is_ok());
    }
}
