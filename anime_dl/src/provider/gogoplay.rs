use log::*;
use scraper::*;

use super::Provider;
use crate::types::{Anime, EpisodeUrl, Quality};

pub struct GoGoPlay {
    base_url:          String,
    download_base_url: String,
}

impl GoGoPlay {
    fn get_title_from_url(&self, url: &str) -> Result<String, &'static str> {
        debug!("Url: {}", url);

        let start = &url.rfind('/');
        let end = &url.rfind("-episode");

        debug!("Begin: {:?}, End: {:?}", start, end);

        if let Some(begin) = start {
            if let Some(ending) = end {
                let name = url[*begin + 1..*ending].to_string();
                debug!("Name: {}", name);
                return Ok(name);
            } else {
                debug!("Ending is None")
            }
        } else {
            debug!("Begin is None")
        }

        Err("Could not parse the name")
    }
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
    fn anime_episode(&self, mut anime: Anime, episode: u32) -> Result<Anime, &'static str> {
        debug!("Looking episode {} for {:?}", episode, anime);
        let mut links: Vec<String> = Vec::new();

        if let Some(val) = anime.max_episode {
            if val < episode {
                error!("Episode {} is higher than {}", episode, val);
                return Err("That high episode number is not available");
            }
        } else {
            return Err("Could not find episodes for that series");
        }

        let base_url_len = self.base_url.len();

        let base_url = anime.root_url[base_url_len..].to_string();

        let url = format!("{}{}-{}", self.base_url, base_url, episode);
        debug!("Fetching page with url {}", url);
        let resp = reqwest::blocking::get(url.as_str()).unwrap().text().unwrap();
        let document = Html::parse_document(resp.as_str());
        let selector = Selector::parse("iframe").unwrap();
        for element in document.select(&selector) {
            // TODO: Rewrite this with regex, I don't know what came to me to write it like this.
            let source = &element.value().attr("src").unwrap()[2..];
            let begin = &source.find("streaming.php").unwrap();
            let e = begin + String::from("streaming.php").len();
            let end = &source[e..];

            let vid_download_url = format!("{}{}", self.download_base_url, end);

            let d_resp = reqwest::blocking::get(vid_download_url.as_str()).unwrap().text().unwrap();
            // TODO: Rewrite this.
            let d_document = Html::parse_document(d_resp.as_str());
            let d_selector = Selector::parse("div.dowload").unwrap();
            let d_link_selector = Selector::parse("a").unwrap();
            for d_element in d_document.select(&d_selector) {
                for link in d_element.select(&d_link_selector) {
                    links.push(link.value().attr("href").unwrap().to_string());
                }
            }
        }

        let mut qualities: Vec<EpisodeUrl> = vec![];

        // TODO: Fix this... why I even thought this will be good way to handle.
        for each in &links {
            if each.contains("1080p") {
                qualities.push(EpisodeUrl {
                    quality: Quality::P1080,
                    url:     each.into(),
                });
                continue;
            } else if each.contains("HDP") {
                debug!("Episode url: {}, Quality {:?}", each, Quality::P1080);
                qualities.push(EpisodeUrl {
                    quality: Quality::P1080,
                    url:     each.into(),
                });
                continue;
            } else if each.contains("720p") {
                debug!("Episode url: {}, Quality {:?}", each, Quality::P720);
                qualities.push(EpisodeUrl {
                    quality: Quality::P720,
                    url:     each.into(),
                });
                continue;
            } else if each.contains("480p") {
                debug!("Episode url: {}, Quality {:?}", each, Quality::P480);
                qualities.push(EpisodeUrl {
                    quality: Quality::P480,
                    url:     each.into(),
                });
                continue;
            } else if each.contains("360p") {
                debug!("Episode url: {}, Quality {:?}", each, Quality::P360);
                qualities.push(EpisodeUrl {
                    quality: Quality::P360,
                    url:     each.into(),
                });
                continue;
            } else {
                debug!("Episode url: {}, Quality {:?}", each, Quality::Unknown);
                qualities.push(EpisodeUrl {
                    quality: Quality::Unknown,
                    url:     each.into(),
                });
                continue;
            }
        }

        if qualities.is_empty() {
            return Err("No link found");
        }

        anime.qualities = Some(qualities);
        Ok(anime)
    }

    fn search_anime(&self, what: &str) -> Result<Vec<Anime>, &'static str> {
        let search_url: String = format!("{}/search.html?keyword={}", self.base_url, what);
        let resp = reqwest::blocking::get(search_url.as_str()).unwrap().text().unwrap();
        let document = Html::parse_document(resp.as_str());
        let selector = Selector::parse("ul.listing").unwrap();
        let video_selector = Selector::parse("a").unwrap();
        let mut animes: Vec<Anime> = Vec::new();

        // TODO: Fix this mess, you know you can make these much nicer and implemente them only for gogo if needed?
        // This trait only needs to return what is expected, not to do everything required to come in that solution.
        for element in document.select(&selector) {
            for video in element.select(&video_selector) {
                let mut tmp = Anime::default();
                let link = video.value().attr("href").unwrap().to_string();

                let episode_str = "-episode-";
                let last_dash = link.rfind(episode_str).unwrap();
                let episode_number = &link[last_dash + episode_str.len()..];
                debug!("{}", episode_number);
                tmp.max_episode = Some(episode_number.parse::<u32>().unwrap());
                tmp.root_url = format!("{}{}", self.base_url, link[..last_dash].to_string());

                tmp.title = self.get_title_from_url(&link).expect("Error while reading title");

                if let Ok(val) = episode_number.to_string().parse::<u32>() {
                    tmp.max_episode = Some(val)
                }

                animes.push(tmp);
            }
        }
        if animes.is_empty() {
            return Err("There is no anime with that name");
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
        assert!(gogo.search_anime(&"asdf".to_string()).is_err());
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

    // TODO: Token in url is generated on load, so cannot test this as for now. Look way to only partial comparsion in string.
    #[ignore]
    #[test]
    fn test_anime_episode() {
        let gogo = GoGoPlay::default();
        let test_anime = Anime {
            title:       "bleach-movie-2-the-diamond-dust-rebellion".to_string(),
            root_url:    "https://gogo-play.net/videos/bleach-movie-2-the-diamond-dust-rebellion".to_string(),
            max_episode: Some(1),
            qualities:   None,
        };

        assert_eq!(
            gogo.anime_episode(test_anime, 1).unwrap(),
            Anime {
                title: "bleach-movie-2-the-diamond-dust-rebellion".to_string(),
                root_url:
                    "https://gogo-play.net/videos/bleach-movie-2-the-diamond-dust-rebellion"
                        .to_string(),
                max_episode: Some(1),
                qualities: Some(
                    vec![EpisodeUrl{ quality: Quality::P360, url: "https://cdn6.cloud9xx.com/user1342/d41902e8565730403c9a5e6a66468d24/EP.1.360p.mp4?token=_MHHryK3ACnYdiH8HhwPMQ&expires=1616792244&id=40445".to_string()},
                         EpisodeUrl{ quality: Quality::Unknown, url: "https://streamsb.net/d/1uif2b9szp8n.html".to_string()},
                         EpisodeUrl{ quality: Quality::Unknown, url: "https://streamtape.com/v/Dj8MPk03xqtka69/bleach-movie-2-the-diamond-dust-rebellion-episode-1.mp4".to_string()},
                         EpisodeUrl{ quality: Quality::Unknown, url: "https://dood.to/d/8mqh9jsx8l4u".to_string()},
                         EpisodeUrl{ quality: Quality::Unknown, url: "https://fcdn.stream/f/zyvn-nnz8o1".to_string()},
                         EpisodeUrl{ quality: Quality::Unknown, url: "https://mixdrop.co/f/7rrvjlvvaedprw".to_string()},
                         EpisodeUrl{ quality: Quality::Unknown, url: "http://www.mp4upload.com/yweqp5bwon2s".to_string()}]
                )
            }
        );
    }
}
