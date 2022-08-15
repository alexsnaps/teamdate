fn main() {
  let matches = clap::App::new("teamdate")
    .about("Tracking team mates across timezones")
    .author("Alex Snaps <alex@wcgw.dev>")
    .version(full_version().as_str())
    .get_matches();
}


fn full_version() -> String {
  format!(
    "v{} ({})",
    env!("CARGO_PKG_VERSION"),
    env!("TDATE_GIT_HASH"),
  )
}