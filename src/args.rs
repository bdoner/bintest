use clap;

pub struct Arguments {
    pub quiet: bool,
    pub test_dir: String,
    pub source: String,
    pub temp: String,
    pub expected: String,
    pub command: String,
    pub verbose: bool,
    pub ignore: bool,
    pub clean_failed: bool,
}

pub fn get_args() -> Arguments {
    let args = get_matches();
    let args = args.subcommand_matches("bintest").unwrap();

    let quiet = args.occurrences_of("quiet") > 0;
    let test_dir = args.value_of("directory").unwrap();
    let source = args.value_of("source").unwrap();
    let temp = args.value_of("temp").unwrap();
    let expected = args.value_of("expected").unwrap();
    let command = args.value_of("command").unwrap();
    let verbose = args.occurrences_of("verbose") > 0;
    let ignore = args.occurrences_of("ignore") == 0;
    let clean_failed = args.occurrences_of("clean_failed") > 0;

    Arguments {
        quiet: quiet,
        test_dir: test_dir.into(),
        source: source.into(),
        temp: temp.into(),
        expected: expected.into(),
        command: command.into(),
        verbose: verbose,
        ignore: ignore,
        clean_failed: clean_failed,
    }
}

fn get_matches() -> clap::ArgMatches<'static> {
    clap::App::new("Bintest")
        .subcommand(clap::SubCommand::with_name("bintest")
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
            .arg(clap::Arg::with_name("quiet")
                    .short("q")
                    .long("quiet")
                    .help("Output one character per test."))
            .arg(clap::Arg::with_name("verbose")
                    .short("v")
                    .long("verbose")
                    .help("Don't capture output."))
            .arg(clap::Arg::with_name("ignore")
                    .short("i")
                    .long("ignore")
                    .help("Don't ignore tests starting with ignore"))
            .arg(clap::Arg::with_name("clean_failed")
                    .short("f")
                    .long("clean-failed")
                    .help("By default, bintest will leave the temporary folder for failed/errored \
                        tests. This will make it clean up the temporary folder."))
        )
        .get_matches()
}
