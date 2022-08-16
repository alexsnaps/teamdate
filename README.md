# Teamdate

Helps you keep track of time for team members across different timezones.

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

Where `name` is whatever you want, while `location` is a [IANA location](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones)
