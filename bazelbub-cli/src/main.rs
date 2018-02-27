extern crate clap;
use clap::{Arg, App, AppSettings, SubCommand};

use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;

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

        ("console", Some(m)) =>
            command_console(m.value_of("target").unwrap()),
        _                    => unreachable!()
    }
}

fn command_console(target: &str) -> () {
    let dir = env::temp_dir();

    let file_workspace = File::create(dir.join("WORKSPACE")).unwrap();
    let file_build = File::create(dir.join("BUILD")).unwrap();
    let mut file_skylark = File::create(dir.join("bazelbub_aspect.bzl")).unwrap();
    let skylark = include_str!("bazelbub_aspect.bzl");
    file_skylark.write_all(skylark.as_bytes()).unwrap();

    let flag_aspects = "--aspects=@bazelbub//:bazelbub_aspect.bzl%bazelbub_aspect";
    let flag_override_repository =
        &format!("--override_repository=bazelbub={}", dir.display());
    let flag_output_groups = "--output_groups=bazelbub";

    let mut vec: Vec<&str> = Vec::new();
    vec.push(flag_aspects);
    vec.push(flag_override_repository);
    vec.push(flag_output_groups);
    vec.push(target);

    call_bazel("build", vec);

    drop(file_workspace);
    drop(file_build);
    drop(file_skylark);
}

fn call_bazel(cmd: &str, args: Vec<&str>) -> () {

    let mut command = Command::new("bazel");
    command.arg(cmd);
    command.args(args);

    println!("running: {:?}", command);

    command
        .spawn()
        .unwrap_or_else(|e| { panic!("failed to run bazel: {}", e) })
        .wait()
        .unwrap();
}
