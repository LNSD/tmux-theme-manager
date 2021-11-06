use std::path::PathBuf;
use std::process::exit;

use clap::{App, Arg, ArgMatches};

use tmux::{Tmux, TmuxCliApi};
use tmux::var;
use tmux::window_options::parse;
use ttm::{Error, load_theme, set_theme, Theme};
use ttm::theme::file::get_theme_path;
use ttm::theme::window_options::{get_theme, get_theme_window_options};

fn main() {
    env_logger::init();

    let matches = arg_parser();

    if !is_tmux_running() {
        eprintln!("No tmux session available"); // TODO Improve error message
        exit(-1);
    } else {
        log::debug!("Tmux env var is set")
    }

    // Build Tmux API
    let tmux = TmuxCliApi::default();

    let theme = match matches.value_of(THEME_CLI_ARG) {
        Some(path) => load_theme_from_file(&path),
        None => load_theme_and_override(&tmux),
    };

    log::debug!("Theme to set (window_options): {:?}", theme.window_options);

    let res = set_theme(&tmux, &theme);
    if res.is_err() {
        if let Error::TmuxError(tmux::Error::CommandExitStatusError(rc)) = res.unwrap_err() {
            // TODO Improve error message
            eprintln!("An error occurred while setting window options to TMUX");
            exit(rc);
        }

        // TODO Improve error message
        eprintln!("An unknown error occurred while setting window options to TMUX");
        exit(-1);
    }

    exit(0);
}

fn is_tmux_running() -> bool {
    var().is_ok()
}

fn load_theme_from_file(path: &str) -> Theme {
    log::debug!("Load theme from file: {:?}", path);

    let theme_path = PathBuf::from(path);
    load_theme(&theme_path).unwrap_or_else(|e| {
        // TODO Improve error message
        eprintln!("An error occurred while loading the theme file: {:?}", e);
        exit(-1);
    })
}

fn load_theme_and_override(tmux: &TmuxCliApi) -> Theme {
    // Get theme name from window_options
    let w_opts = tmux.show_options().unwrap_or_else(|e| {
        // TODO Improve error message
        eprintln!("An error occurred while connecting to TMUX: {:?}", e);
        exit(-1);
    });

    log::debug!("Window options retrieved: {:?}", w_opts);
    let raw_window_options = parse(&w_opts);

    let mut theme = Theme::default();

    // Load theme
    if let Some(theme_name) = get_theme(&raw_window_options) {
        log::trace!("Get theme name from window_options: {:?}", theme_name);
        let theme_path = get_theme_path(&theme_name).unwrap_or_else(|| {
            // TODO Improve error message
            eprintln!("Theme file path could not be built");
            exit(-1);
        });

        log::debug!("Load theme from path: {:?}", theme_path);
        if let Ok(loaded) = load_theme(&theme_path) {
            theme.update(&loaded.window_options);
        }
    }

    // Override theme options
    let theme_override = get_theme_window_options(&raw_window_options);
    theme.update(&theme_override);

    theme
}

const THEME_CLI_ARG: &'static str = "THEME";

fn arg_parser<'a>() -> ArgMatches<'a> {
    App::new("Tmux Theme Manager")
        .version("1.0")
        .author("Lorenzo Delgado (LNSD) <lorenzodelgado.dev@gmail.com")
        .arg(
            Arg::with_name(THEME_CLI_ARG)
                .help("Theme file path")
                .required(false)
                .index(1),
        )
        .get_matches()
}
