#[macro_use]
extern crate clap;

extern crate spacedout;

fn main() {
    let matches = clap::App::new("spacedout")
        .about("Spaced Out: Find where your disk space is used")
        .version(crate_version!())
        .arg(clap::Arg::with_name("PATH")
             .help("Path to scan")
             .required(true)
             .index(1))
        .get_matches();

    spacedout::test(matches.value_of("PATH").unwrap());
}
