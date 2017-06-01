use clap;

pub fn get_args() -> clap::ArgMatches<'static> {
    clap::App::new("Bintest")
        .version(crate_version!())
        .author(crate_authors!())
        .about("A utility to test binaries.")
        .arg(clap::Arg::with_name("directory")
                 .short("d")
                 .long("directory")
                 .value_name("DIRECTORY")
                 .takes_value(true)
                 .default_value("bintest")
                 .help("Directory containing the tests."))
        .arg(clap::Arg::with_name("source")
                 .short("s")
                 .long("source")
                 .value_name("SOURCE")
                 .takes_value(true)
                 .default_value("source")
                 .help("Test's subdirectory containing source folder."))
        .arg(clap::Arg::with_name("temp")
                 .short("t")
                 .long("temp")
                 .value_name("TEMP")
                 .takes_value(true)
                 .default_value("temp")
                 .help("Test's subdirectory containing temporary test folder."))
        .arg(clap::Arg::with_name("expected")
                 .short("e")
                 .long("expected")
                 .value_name("EXPECTED")
                 .takes_value(true)
                 .default_value("expected")
                 .help("Test's subdirectory containing expected folder."))
        .arg(clap::Arg::with_name("command")
                 .short("c")
                 .long("command")
                 .value_name("COMMAND")
                 .takes_value(true)
                 .default_value("command.sh")
                 .help("File inside test containing command to be ran.\
                   If file doesn't exist, will run 'cargo run -q'"))
        .get_matches()
}
