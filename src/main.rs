// Standard library
use std::ffi::OsString;
use std::fs::{
    File,
    read_to_string,
};
use std::io::Write;
use std::process::ExitCode;

// Third-party libraries
use clap::{
    Arg,
    Command,
};

// Local libraries
use colgroup_poc::transpile;


#[derive(Debug)]
struct TranspileArgs {
    src: String,
    dest: Option<String>,
    target: String,
}


fn parse_args<I, S>(args: I) -> Result<TranspileArgs, clap::Error>
where
    I: IntoIterator<Item = S>,
    S: Into<OsString> + Clone,
{
    let matches = Command::new("colgroup_poc")
        .about("POC transpiler for ColGroup extension to GFM Table")
        .args([
            Arg::new("src")
                .value_name("source")
                .help("Markdown file containing the table to parse")
                .required(true),
            Arg::new("dest")
                .short('o')
                .long("output")
                .value_name("destination")
                .help("Declare where to output the transpilation result"),
            Arg::new("target")
                .long("target")
                .value_name("html|pdf|...")
                .help("Target language for the table translation (default: html)")
                .default_value("html"),
        ])
        .try_get_matches_from(
            std::iter::once(OsString::from("colgroup_poc"))
                .chain(args.into_iter().map(|arg| arg.into())),
        )?;

    return Ok(TranspileArgs {
        src: matches
            .get_one::<String>("src")
            .expect("required argument")
            .clone(),
        dest: matches.get_one::<String>("dest").cloned(),
        target: matches
            .get_one::<String>("target")
            .expect("default value is set")
            .clone(),
    });
}


fn main() -> ExitCode {
    let transpile_cmd = match parse_args(std::env::args_os().skip(1)) {
        Ok(val) => val,
        Err(err) => err.exit(),
    };

    println!("{transpile_cmd:#?}");

    let src = transpile_cmd.src;
    let dest = transpile_cmd.dest.unwrap_or(format!("output.{}", transpile_cmd.target));
    let target = transpile_cmd.target;

    let input = match read_to_string(&src) {
        Ok(val) => val,
        Err(err) => {
            eprintln!(
                "Couldn't read source file '{src}': {err}",
            );
            return ExitCode::FAILURE;
        },
    };

    let output= match transpile(&input, target.as_str()) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Couldn't transpile '{src}': {err}");
            return ExitCode::FAILURE;
        },
    };

    let mut destfile = match File::create(&dest) {
        Ok(val) => val,
        Err(err) => {
            eprintln!(
                "Couldn't create destination file '{dest}': {err}"
            );
            return ExitCode::FAILURE;
        },
    };

    if let Err(err) = destfile.write_all(output.as_bytes()) {
        eprintln!(
            "Couldn't write to destination file '{dest}': {err}"
        );
        return ExitCode::FAILURE;
    }

    return ExitCode::SUCCESS;
}
