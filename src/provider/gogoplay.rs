// Anime page: https://gogo-play.net/videos/bleach-episode-1
// Anime Search Page: https://gogo-play.net/search.html?keyword=Bleach
// Anime Download page: https://gogo-stream.com/download?id=MTE3MTU=&title=Bleach&typesub=SUB&sub=eyJlbiI6bnVsbCwiZXMiOm51bGx9&cover=aW1hZ2VzL2FuaW1lL0IvYmxlYWNoLmpwZw==&refer=https://gogo-play.net/videos/bleach-episode-366

use log::*;

use scraper::*;

use super::Anime;
use super::Provider;
use crate::anime::Quality;
use crate::anime::QualityUrl;

pub struct GoGoPlay {
    base_url: String,
    download_base_url: String,
}

impl Default for GoGoPlay {
    fn default() -> Self {
        Self {
            base_url: String::from("https://gogo-play.net"),
            download_base_url: String::from("https://gogo-stream.com/download"),
        }
    }
}

impl Provider for GoGoPlay {
    fn anime_episode(&self, mut anime: Anime, episode: u32) -> Result<Anime, &'static str> {
        debug!("Looking episode {} for {:?}", episode, anime);
        let mut links: Vec<String> = Vec::new();

        match anime.max_episode {
            Some(val) => {
                if val < episode {
                    error!("Episode {} is higher than {}", episode, val);
                    return Err("That high episode number is not available");
                }
            }
            _ => {}
        }

        let url = format!("{}{}-{}", self.base_url, anime.root_url, episode);
        debug!("Fetching page with url {}", url);
        let resp = reqwest::blocking::get(url.as_str())
            .unwrap()
            .text()
            .unwrap();
        let document = Html::parse_document(resp.as_str());
        let selector = Selector::parse("iframe").unwrap();
        for element in document.select(&selector) {
            let source = &element.value().attr("src").unwrap()[2..];
            let begin = &source.find("streaming.php").unwrap();
            let e = begin + String::from("streaming.php").len();
            let end = &source[e..];
            let vid_download_url = format!("{}{}", self.download_base_url, end);

            let d_resp = reqwest::blocking::get(vid_download_url.as_str())
                .unwrap()
                .text()
                .unwrap();
            // println!("{}", d_resp);
            let d_document = Html::parse_document(d_resp.as_str());
            let d_selector = Selector::parse("div.dowload").unwrap();
            let d_link_selector = Selector::parse("a").unwrap();
            for d_element in d_document.select(&d_selector) {
                for link in d_element.select(&d_link_selector) {
                    links.push(link.value().attr("href").unwrap().to_string());
                }
            }
        }

        let mut qualities: Vec<QualityUrl> = vec![];

        // TODO: Fix this... why I even thought this will be good way to handle.
        for each in &links {
            if each.contains("1080p") {
                qualities.push(QualityUrl {
                    quality: Quality::P1080,
                    url: each.into(),
                });
                continue;
            } else if each.contains("HDP") {
                debug!("Episode url: {}, Quality {:?}", each, Quality::P1080);
                qualities.push(QualityUrl {
                    quality: Quality::P1080,
                    url: each.into(),
                });
                continue;
            } else if each.contains("720p") {
                debug!("Episode url: {}, Quality {:?}", each, Quality::P720);
                qualities.push(QualityUrl {
                    quality: Quality::P720,
                    url: each.into(),
                });
                continue;
            } else if each.contains("480p") {
                debug!("Episode url: {}, Quality {:?}", each, Quality::P480);
                qualities.push(QualityUrl {
                    quality: Quality::P480,
                    url: each.into(),
                });
                continue;
            } else if each.contains("360p") {
                debug!("Episode url: {}, Quality {:?}", each, Quality::P360);
                qualities.push(QualityUrl {
                    quality: Quality::P360,
                    url: each.into(),
                });
                continue;
            } else {
                debug!("Episode url: {}, Quality {:?}", each, Quality::Unknown);
                qualities.push(QualityUrl {
                    quality: Quality::Unknown,
                    url: each.into(),
                });
                continue;
            }
        }

        if qualities.len() == 0 {
            return Err("No link found");
        }

        anime.qualities = Some(qualities);
        return Ok(anime);
    }

    fn search_anime(&self, what: &String) -> Result<Vec<Anime>, &'static str> {
        let search_url: String = format!("{}/search.html?keyword={}", self.base_url, what);
        let resp = reqwest::blocking::get(search_url.as_str())
            .unwrap()
            .text()
            .unwrap();
        let document = Html::parse_document(resp.as_str());
        let selector = Selector::parse("ul.listing").unwrap();
        let video_selector = Selector::parse("a").unwrap();
        let mut animes: Vec<Anime> = Vec::new();

        // TODO: Fix this mess, you know you can make these much nicer and implemente them only for gogo if needed?
        // This trait only needs to return what is expected, not to do everything required to come in that solution.
        for element in document.select(&selector) {
            for video in element.select(&video_selector) {
                let mut tmp_anime = Anime::default();
                let anime_name = video.value().attr("href").unwrap().to_string();
                let last_dash = anime_name.rfind('-').unwrap();
                let episode_number = &anime_name[last_dash + 1..];
                tmp_anime.max_episode = Some(episode_number.parse::<u32>().unwrap());
                tmp_anime.root_url = anime_name[..last_dash].to_string();

                let begin = &tmp_anime.root_url.rfind('/').unwrap();
                let end = &tmp_anime.root_url.rfind("-episode").unwrap();

                tmp_anime.title = tmp_anime.root_url.clone()[*begin + 1..*end].to_string();

                match episode_number.to_string().parse::<u32>() {
                    Ok(val) => tmp_anime.max_episode = Some(val),
                    _ => {}
                }
                animes.push(tmp_anime);
            }
        }

        Ok(animes)
    }
}
