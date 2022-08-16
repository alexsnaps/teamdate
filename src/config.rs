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

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Config {
  pub teams: HashMap<String, Vec<Member>>,
}

#[derive(Deserialize)]
pub struct Member {
  pub name: String,
  pub location: chrono_tz::Tz,
}

#[cfg(test)]
mod tests {
  use crate::config::Config;

  #[test]
  fn reads_config_alright() {
    let config: Config = toml::from_str(
      r#"
    [[teams.wcgw]]
    name = "Alex"
    location = "America/Montreal"

    [[teams.wcgw]]
    name = "John Doe"
    location = "Europe/Dublin"

    [[teams.managers]]
    name = "John Doe"
    location = "Europe/Dublin"
    "#,
    )
    .unwrap();

    assert_eq!(config.teams.len(), 2);
    assert!(config.teams.contains_key("wcgw"));
    assert!(config.teams.contains_key("managers"));

    assert_eq!(config.teams.get("wcgw").unwrap().len(), 2);
    assert_eq!(config.teams.get("managers").unwrap().len(), 1);

    assert_eq!(config.teams.get("managers").unwrap().first().unwrap().name, "John Doe");
    assert_eq!(
      config.teams.get("managers").unwrap().first().unwrap().location,
      chrono_tz::Tz::Europe__Dublin
    );

    assert_eq!(config.teams.get("wcgw").unwrap()[0].name, "Alex");
    assert_eq!(
      config.teams.get("wcgw").unwrap()[0].location,
      chrono_tz::Tz::America__Montreal
    );

    assert_eq!(config.teams.get("wcgw").unwrap()[1].name, "John Doe");
    assert_eq!(
      config.teams.get("wcgw").unwrap()[1].location,
      chrono_tz::Tz::Europe__Dublin
    );
  }
}
