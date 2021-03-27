use std::process::{Command, Stdio};

use anime_dl::provider::{GoGoPlay, Provider};
use clap::Clap;
use log::info;
use simplelog::*;

pub struct QuerySet {
    pub provider: Option<Box<dyn Provider>>,
    pub series:   String,
    pub episode:  Option<u32>,
    pub ignore:   Option<String>,
}

impl Default for QuerySet {
    fn default() -> Self {
        Self {
            provider: None,
            series:   String::default(),
            episode:  None,
            ignore:   None,
        }
    }
}

#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Opts {
    #[clap(long, short, default_value = "gogoplay", about = "Select provider what to use, defaults to gogoplay", possible_values = &["gogoplay"])]
    provider: String,
    #[clap(long, short, about = "What series trying to find?")]
    series:   String,
    #[clap(long, short, about = "Episode number, only whole number episodes are possible")]
    episode:  Option<u32>,
    // #[clap(long, short, about = "WIP: What string to ignore while searching animes ex. `dub`")]
    // ignore:   Option<String>,
    #[clap(short, about = "Open in mpv episode what is found.")]
    watch:    bool,
    #[clap(long, short, parse(from_occurrences), about = "I tell you a story, a sad one.")]
    verbose:  u8,
}

fn main() {
    let opts: Opts = Opts::parse();

    let loggerconfig = ConfigBuilder::new()
        .add_filter_ignore("selectors".to_string())
        .add_filter_ignore("html5ever".to_string())
        .build();

    let verb = match opts.verbose {
        0 => {
            if cfg!(debug_assertions) {
                eprintln!("Debuggin on, if you don't need debug logging build release");
                LevelFilter::Debug
            } else {
                LevelFilter::Error
            }
        }
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    CombinedLogger::init(vec![TermLogger::new(verb, loggerconfig, TerminalMode::Mixed, ColorChoice::Auto)])
        .expect("Error setting simplelogger.");

    let mut search = QuerySet::default();

    match opts.provider.as_ref() {
        "gogoplay" => {
            info!("GoGoPlay provider selected");
            search.provider = Some(Box::new(GoGoPlay::default()));
        }
        _ => {
            info!("GoGoPlay provider selected by default");
            search.provider = Some(Box::new(GoGoPlay::default()))
        }
    }

    search.series = opts.series;
    search.episode = opts.episode;
    // search.ignore = opts.ignore;

    let mut found = vec![];
    if let Some(provider) = &search.provider {
        found = match provider.search_anime(&search.series) {
            Ok(val) => val,
            Err(_) => {
                eprintln!("404 Anime not found");
                std::process::exit(10);
            }
        }
    }

    if search.episode.is_none() {
        for each in found {
            println!("{}", each);
        }

        std::process::exit(0)
    } else {
        let link = search
            .provider
            .expect("Provider is not defined?")
            .anime_episode(found[0].clone(), search.episode.unwrap())
            .expect("Episode with that number not found");

        if let Some(qualities) = link.qualities {
            let mut highest = &qualities[0];

            for each in &qualities {
                if highest.quality > each.quality {
                    continue;
                } else {
                    highest = each;
                }
            }

            if opts.watch {
                let mut cmd = Command::new("mpv")
                    .arg(&highest.url)
                    .stdout(Stdio::inherit())
                    .stdin(Stdio::inherit())
                    .spawn()
                    .expect("Problem starting mpv");

                let status = cmd.wait();

                println!("Exited mpv with code {}\nHave a nice day", status.unwrap())
            } else {
                println!("{}", &highest.url);
            }
        }
    }
}
