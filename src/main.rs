// mod anime;
// mod provider;

use clap::App;
use clap::Arg;
use log::info;
use simplelog::*;

use anime_dl::provider::gogoplay::GoGoPlay;
use anime_dl::provider::Provider;

use std::process::{Command, Stdio};

pub struct QuerySet {
    pub provider: Option<Box<dyn Provider>>,
    pub series: String,
    pub episode: Option<u32>,
    pub ignore: Option<String>,
}

impl Default for QuerySet {
    fn default() -> Self {
        Self {
            provider: None,
            series: String::default(),
            episode: None,
            ignore: None,
        }
    }
}

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("provider")
                .short('p')
                .long("provider")
                .takes_value(true)
                .about("Select provider, defaults to GoGoPlay for now."),
        )
        .arg(
            Arg::new("series")
                .short('s')
                .long("series")
                .about("Search series")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("episode")
                .short('e')
                .long("episode")
                .about("episode number, selects first anime what provider gives.")
                .takes_value(true),
        )
        .arg(
            Arg::new("ignore")
                .short('i')
                .long("ignore")
                .about("TODO: string value what can be ignored from name, ex dub")
                .takes_value(true),
        )
        .arg(
            Arg::new("watch")
                .short('w')
                .long("watch")
                .about("If this is given mpv is spawn to watch"),
        )
        .get_matches();

    let loggerconfig = ConfigBuilder::new()
        .add_filter_ignore("selectors".to_string())
        .add_filter_ignore("html5ever".to_string())
        .build();

    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        loggerconfig,
        TerminalMode::Mixed,
    )])
    .expect("Error setting simplelogger.");

    let mut search = QuerySet::default();

    if let Some(provider) = matches.value_of("provider") {
        match provider {
            "gogoplay" => {
                info!("GoGoPlay provider selected");
                search.provider = Some(Box::new(GoGoPlay::default()));
            }
            _ => {
                info!("GoGoPlay provider selected by default");
                search.provider = Some(Box::new(GoGoPlay::default()))
            }
        }
    }

    if search.provider.is_none() {
        search.provider = Some(Box::new(GoGoPlay::default()));
    }

    if let Some(series) = matches.value_of("series") {
        info!("Series was given {}", series);
        search.series = series.to_string();
    }

    if let Some(episode) = matches.value_of("episode") {
        info!("Episode number was given {}", episode);
        // TODO: Should this actually be f32? There is quite few episodes with weird numbering.
        // TODO: NO. Episodes are whole numbered, why the hell I thought that.
        search.episode = Some(
            episode
                .parse::<u32>()
                .expect("You should give positive whole number"),
        );
    }

    if let Some(ignore) = matches.value_of("ignore") {
        search.ignore = Some(ignore.to_string());
    }

    // TODO: Isn't there some sexier way to make this? Somehow feels clunky to first define variable and then write result to it.
    let mut found = vec![];
    if let Some(provider) = &search.provider {
        found = provider
            .search_anime(&search.series)
            .expect("Could not find that series");
    }

    if search.episode == None {
        for each in found {
            println!("{}", each);
            std::process::exit(0)
        }
    } else {
        let link = search
            .provider
            .expect("Provider is not defined?")
            .anime_episode(found[0].clone(), search.episode.unwrap())
            .expect("Episode with that number not found");

        if matches.is_present("watch") {
            let mut cmd = Command::new("mpv")
                .arg(link.to_string())
                .stdout(Stdio::inherit())
                .stdin(Stdio::inherit())
                .spawn()
                .expect("Problem starting mpv");

            let status = cmd.wait();

            println!("Exited with status {:?}\nHave a nice day", status);
        } else {
            println!("{}", link);
        }
    }
}
