extern crate clap;
use clap::{Arg, App, AppSettings, SubCommand};

fn main() {

    let matches =
        App::new("bazelbub")
        .version("0.0.0")
        .author("Andy Scott <andy.g.scott@gmail.com>")
        .about("local dev workflows for Bazel")
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .global_settings(&[AppSettings::ColoredHelp])
        .subcommand(SubCommand::with_name("console")
                    .about("opens an interactive repl for a target")
                    .arg(Arg::with_name("target")
                         .help("specifies the bazel target")
                         .required(true)
                         .index(1)))
        .subcommand(SubCommand::with_name("build")
                    .about("calls bazel build")
                    .setting(AppSettings::TrailingVarArg)
                    .arg(Arg::with_name("args")
                         .multiple(true)
                         .allow_hyphen_values(true)))
        .subcommand(SubCommand::with_name("run")
                    .about("calls bazel run")
                    .setting(AppSettings::TrailingVarArg)
                    .arg(Arg::with_name("args")
                         .multiple(true)
                         .allow_hyphen_values(true)))
        .get_matches();

    match matches.subcommand_name() {
        Some("build")   => println!("invoke bazel build!"),
        Some("run")     => println!("invoke bazel run!"),
        Some("console") => println!("open the fancy repl"),
        _               => println!("oh no")
    }
}
