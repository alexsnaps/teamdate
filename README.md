# Teamdate

Helps you keep track of time for team members across different timezones 
and other daylight saving changes based off their location. Because I know I can't do it!

[![crates.io](https://img.shields.io/crates/v/teamdate.svg)](https://crates.io/crates/teamdate)

## Usage

### Current time for your team members

```shell
$ tdate 
┏━━━━━━━━━━━━━━━━━━━━━━━━━━┯━━━━━━━━━━━━━━━━━━┓
┃       Team member        │       Time       ┃
┠──────────────────────────┼──────────────────┨
┃  Alex (America/Montreal) │ Mon Aug 15 21:18 ┃
┃ Jane Doe (Europe/Dublin) │ Tue Aug 16 02:18 ┃
┃ John Doe (Europe/Dublin) │ Tue Aug 16 02:18 ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━┷━━━━━━━━━━━━━━━━━━┛
```

### Some date by location

```shell
$ tdate -l Aug 28 3pm 
┏━━━━━━━━━━━━━━━━━━┯━━━━━━━━━━━━━━━━━━┓
┃     Location     │       Time       ┃
┠──────────────────┼──────────────────┨
┃ America/Montreal │ Sun Aug 28 15:00 ┃
┃    Europe/Dublin │ Sun Aug 28 20:00 ┃
┗━━━━━━━━━━━━━━━━━━┷━━━━━━━━━━━━━━━━━━┛

```

### Some date based of offset

```shell
$ tdate 3 weeks 10:30am
┏━━━━━━━━━━━━━━━━━━━━━━━━━━┯━━━━━━━━━━━━━━━━━━┓
┃       Team member        │       Time       ┃
┠──────────────────────────┼──────────────────┨
┃  Alex (America/Montreal) │ Mon Sep 05 10:30 ┃
┃ Jane Doe (Europe/Dublin) │ Mon Sep 05 15:30 ┃
┃ John Doe (Europe/Dublin) │ Mon Sep 05 15:30 ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━┷━━━━━━━━━━━━━━━━━━┛
```

Or try `tomorrow 3pm` or `next Monday 2pm` or … try it out!

### Full usage

```shell
teamdate v0.1.0 
Alex Snaps <alex@wcgw.dev>
Tracking team mates across timezones

USAGE:
    tdate [OPTIONS] [DATE]...

ARGS:
    <DATE>...    Date to parse [default: now]

OPTIONS:
    -t, --team <TEAM>        Print specific team
        --all                Print all teams
    -l, --by-location        Group by locations
    -c, --config <CONFIG>    The config file to use [default:
                             /Users/alexsnaps/.config/teamdate/teams.toml]
    -h, --help               Print help information
    -V, --version            Print version information
```

## Installation

Currently, this only works with [cargo](https://doc.rust-lang.org/cargo/)

### Steps

```shell
$ git clone https://github.com/alexsnaps/teamdate.git teamdate

$ cd teamdate

$ cargo install --path .
```

### Provide a config

```shell
$ cat ~/.config/teamdate/teams.toml
```

```toml
default_team = "wcgw"
date_format = "%c"

[[teams.wcgw]]
name = "Alex"
location = "America/Montreal"

[[teams.wcgw]]
name = "Jane Doe"
location = "Europe/Dublin"

[[teams.wcgw]]
name = "John Doe"
location = "Europe/Dublin"

[[teams.managers]]
name = "John Doe"
location = "Europe/Dublin"
```

- You can assign a `default_team` that'll be used when none is provided (see `-t` or `--all`).
- You can specify how a date should be printed with `date_format`, see [strftime](https://docs.rs/chrono/0.4.22/chrono/format/strftime/index.html) _default:_ `"%a %b %d %H:%M"`
- For your teams, `name` is whatever you want, while `location` is a [IANA location](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones)

