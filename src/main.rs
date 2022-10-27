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

use crate::config::{Config, Member};
use chrono::{DateTime, Local};
use chrono_english::parse_date_string;
use clap::ArgAction;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use wyz::exit;

#[macro_use]
extern crate lazy_static;

mod config;

lazy_static! {
  static ref CFG: String = default_config();
  static ref VERSION: String = full_version();
}

fn main() {
  let command = clap::Command::new("teamdate")
    .about("Tracking team mates across timezones")
    .author("Alex Snaps <alex@wcgw.dev>")
    .version(&VERSION.as_str())
    .arg(
      clap::Arg::new("CONFIG")
        .short('c')
        .long("config")
        .help("The config file to use")
        .display_order(4)
        .default_value(&CFG.as_str()),
    )
    .arg(
      clap::Arg::new("LOCATIONS")
        .short('l')
        .long("by-location")
        .help("Group by locations")
        .display_order(3)
        .action(ArgAction::SetTrue),
    )
    .arg(
      clap::Arg::new("ALL")
        .long("all")
        .help("Print all teams")
        .display_order(2)
        .action(ArgAction::SetTrue),
    )
    .arg(
      clap::Arg::new("TEAM")
        .short('t')
        .long("team")
        .help("Print specific team")
        .display_order(1)
        .conflicts_with("ALL"),
    )
    .trailing_var_arg(true)
    .arg(
      clap::Arg::new("DATE")
        .num_args(0..)
        .help("Date to parse")
        .default_value("now"),
    );
  let matches = command.get_matches();

  let cfg_src: &String = matches.get_one("CONFIG").unwrap();
  let cfg: Config = match File::open(cfg_src) {
    Ok(mut file) => {
      let mut data = String::with_capacity(1024);
      match file.read_to_string(&mut data) {
        Ok(_) => match toml::from_str(&data) {
          Ok(cfg) => cfg,
          Err(err) => exit!(1, "Couldn't parse config file '{}': {}", cfg_src, err),
        },
        Err(err) => exit!(1, "Couldn't read config file '{}': {}", cfg_src, err),
      }
    },
    Err(err) => exit!(1, "Couldn't open config file '{}': {}", cfg_src, err),
  };

  let location_grouping = matches.contains_id("LOCATIONS");

  let date = if let Some(date) = matches.get_many::<String>("DATE") {
    let date_string = date.cloned().collect::<String>();
    match parse_date_string(&date_string, Local::now(), cfg.dialect()) {
      Ok(date) => date,
      Err(err) => exit!(2, "Couldn't parse date '{}': {}", date_string, err),
    }
  } else {
    Local::now()
  };

  let team = if matches.contains_id("ALL") {
    None
  } else if let Some(team) = matches.get_one::<String>("TEAM") {
    cfg.teams.get(team)
  } else {
    cfg.default_team()
  };

  let left_header = if location_grouping { "Location" } else { "Team member" };
  if let Some(members) = team {
    let lines = team_to_lines(&cfg, location_grouping, date, members);
    print_timezones(left_header, "Time", lines);
  } else {
    for (team, members) in &cfg.teams {
      println!("\n => Team {}", team);
      let lines = team_to_lines(&cfg, location_grouping, date, members);
      print_timezones(left_header, "Time", lines);
    }
  }
}

fn team_to_lines(
  cfg: &Config,
  location_grouping: bool,
  date: DateTime<Local>,
  members: &Vec<Member>,
) -> Vec<(String, String)> {
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
        date.with_timezone(&location).format(cfg.date_format()).to_string(),
      ));
    }
  } else {
    for member in members {
      lines.push((
        format!("{} ({})", member.name, member.location),
        date
          .with_timezone(&member.location)
          .format(cfg.date_format())
          .to_string(),
      ));
    }
  }
  lines
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
