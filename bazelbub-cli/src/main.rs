use std::process::Command;

extern crate clap;
use clap::{Arg, App, AppSettings, SubCommand};

fn main() {

    let matches =
        App::new("bazelbub")
        .version("0.0.0")
        .author("Andy Scott <andy.g.scott@gmail.com>")
        .about("local dev workflows for Basel")
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

    match matches.subcommand() {
        ("build",   Some(m)) =>
            call_bazel("build", m.values_of("args")
                       .map(|r| r.collect::<Vec<_>>())
                       .unwrap_or(vec![])),

        ("run",     Some(m)) =>
            call_bazel("run", m.values_of("args")
                       .map(|r| r.collect::<Vec<_>>())
                       .unwrap_or(vec![])),

        ("console", Some(m)) => println!("open the fancy repl"),
        _                    => unreachable!()
    }
}

fn call_bazel(cmd: &str, args: Vec<&str>) -> () {
    Command::new("bazel")
        .arg(cmd)
        .args(args)
        .spawn()
        .unwrap_or_else(|e| { panic!("failed to run bazel: {}", e) })
        .wait();
}
