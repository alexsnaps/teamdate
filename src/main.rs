/*
 * timedate - tracking team mates across timezones
 * Copyright (C) 2022  Alex Snaps <alex@wcgw.dev>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::config::Config;
use chrono::Local;
use chrono_english::{parse_date_string, Dialect};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use wyz::exit;

mod config;

fn main() {
  let matches = clap::App::new("teamdate")
    .about("Tracking team mates across timezones")
    .author("Alex Snaps <alex@wcgw.dev>")
    .version(full_version().as_str())
    .arg(
      clap::Arg::new("CONFIG")
        .short('c')
        .long("config")
        .help("The config file to use")
        .takes_value(true)
        .default_value(default_config().as_str()),
    )
    .arg(
      clap::Arg::new("LOCATIONS")
        .short('l')
        .long("by-location")
        .help("Group by locations")
        .takes_value(false),
    )
    .trailing_var_arg(true)
    .arg(clap::Arg::new("DATE").multiple_occurrences(true).help("Date to parse"))
    .get_matches();

  let config_src = matches.value_of("CONFIG").unwrap();
  let config: Config = match File::open(config_src) {
    Ok(mut file) => {
      let mut data = String::with_capacity(1024);
      match file.read_to_string(&mut data) {
        Ok(_) => match toml::from_str(&data) {
          Ok(cfg) => cfg,
          Err(err) => exit!(1, "Couldn't parse config file '{}': {}", config_src, err),
        },
        Err(err) => exit!(1, "Couldn't read config file '{}': {}", config_src, err),
      }
    },
    Err(err) => exit!(1, "Couldn't open config file '{}': {}", config_src, err),
  };

  let teams = config.teams.len();
  let location_grouping = matches.is_present("LOCATIONS");

  let date = if let Some(date) = matches.values_of("DATE") {
    let date_string = date.collect::<Vec<&str>>().join(" ");
    match parse_date_string(&date_string, Local::now(), Dialect::Us) {
      Ok(date) => date,
      Err(err) => exit!(2, "Couldn't parse date '{}': {}", date_string, err),
    }
  } else {
    Local::now()
  };

  for (team, members) in config.teams {
    if teams > 1 {
      println!("\n => Team {}", team);
    }
    let mut lines: Vec<(String, String)> = Vec::new();
    if location_grouping {
      let locations: HashSet<chrono_tz::Tz> = members.iter().map(|m| m.location).collect();
      let mut locations: Vec<chrono_tz::Tz> = locations.into_iter().collect();
      locations.sort_by(|a, b| {
        let one = date.with_timezone(a);
        let two = date.with_timezone(b);
        match one.date_naive().cmp(&two.date_naive()) {
          Ordering::Less => Ordering::Less,
          Ordering::Equal => one.time().cmp(&two.time()),
          Ordering::Greater => Ordering::Greater,
        }
      });
      for location in locations {
        lines.push((
          location.to_string(),
          date.with_timezone(&location).format("%a %b %d %H:%M").to_string(),
        ));
      }
    } else {
      for member in members {
        lines.push((
          format!("{} ({})", member.name, member.location),
          date
            .with_timezone(&member.location)
            .format("%a %b %d %H:%M")
            .to_string(),
        ));
      }
    }
    let left_header = if location_grouping { "Location" } else { "Team member" };
    print_timezones(left_header, "Time", lines);
  }
}

fn full_version() -> String {
  format!("v{} ({})", env!("CARGO_PKG_VERSION"), env!("TDATE_GIT_HASH"),)
}

fn default_config() -> String {
  // todo probably not best here
  format!(
    "{}/.config/teamdate/teams.toml",
    dirs::home_dir().unwrap().to_str().unwrap()
  )
}

fn print_timezones(h1: &str, h2: &str, lines: Vec<(String, String)>) {
  let (l_width, r_width) = lines
    .iter()
    .map(|(l, r)| (l.len(), r.len()))
    .fold((h1.len(), h2.len()), |(m1, m2), (l, r)| (m1.max(l), m2.max(r)));

  println!("┏━{0:━>w1$}━┯━{0:━^w2$}━┓", "━", w1 = l_width, w2 = r_width,);
  println!("┃ {0: ^w1$} │ {1: ^w2$} ┃", h1, h2, w1 = l_width, w2 = r_width,);
  println!("┠─{0:─>w1$}─┼─{0:─^w2$}─┨", "─", w1 = l_width, w2 = r_width,);
  lines.iter().for_each(|(left, right)| {
    println!("┃ {0: >w1$} │ {1: <w2$} ┃", left, right, w1 = l_width, w2 = r_width,);
  });
  println!("┗━{0:━>w1$}━┷━{0:━^w2$}━┛", "━", w1 = l_width, w2 = r_width,);
}
