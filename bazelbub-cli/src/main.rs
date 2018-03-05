extern crate clap;
use clap::{Arg, App, AppSettings, SubCommand};

use std::env;
use std::fs::File;
use std::io::Read;
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

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[derive(Deserialize, Debug)]
struct AspectInfo {
    runtime_classpath: Vec<String>,
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

    let mut vec1: Vec<&str> = Vec::new();
    vec1.push(flag_aspects);
    vec1.push(flag_override_repository);
    vec1.push(flag_output_groups);
    vec1.push(target);

    call_bazel("build", vec1);

    let mut vec2: Vec<&str> = Vec::new();
    vec2.push(flag_aspects);
    vec2.push(flag_override_repository);
    vec2.push(flag_output_groups);
    vec2.push("@scala//:scala-compiler");

    call_bazel("build", vec2);

    drop(file_workspace);
    drop(file_build);
    drop(file_skylark);

    let output1 =
        Command::new("bazel").args(&["info", "bazel-bin"])
        .output().unwrap();
    let sout1 = String::from_utf8(output1.stdout).unwrap();
    let bazel_bin = sout1.trim();

    let output2 =
        Command::new("bazel").args(&["query", "--output=label", target])
        .output().unwrap();
    let sout2 = String::from_utf8(output2.stdout).unwrap();
    let bits = sout2.trim().split(':').collect::<Vec<&str>>();
    let (package, name) = (bits[0], bits[1]);

    let output3 =
        Command::new("bazel").args(&["info", "execution_root"])
        .output().unwrap();
    let sout3 = String::from_utf8(output3.stdout).unwrap();
    let execution_root = sout3.trim();

    let output_path1 = &format!("{}{}/bazelbub.{}.json", bazel_bin, package, name);
    let output_path2 = &format!("{}/external/scala/bazelbub.scala-compiler.json", bazel_bin);

    let mut data1 = String::new();
    let mut file1 = File::open(output_path1).unwrap();
    file1.read_to_string(&mut data1).unwrap();

    let mut data2 = String::new();
    let mut file2 = File::open(output_path2).unwrap();
    file2.read_to_string(&mut data2).unwrap();

    let aspect_info1: AspectInfo = serde_json::from_str(&data1).ok().unwrap();
    let aspect_info2: AspectInfo = serde_json::from_str(&data2).ok().unwrap();

    let full_classpath =
        aspect_info1.runtime_classpath.iter().chain(
            aspect_info2.runtime_classpath.iter())
        .map(|p| format!("{}/{}", execution_root, p))
        .collect::<Vec<String>>();

    Command::new("java")
        .args(&["-cp", &full_classpath.join(":")])
        .arg("scala.tools.nsc.MainGenericRunner")
        .arg("-usejavacp")
        .spawn()
        .unwrap_or_else(|e| { panic!("failed to run java: {}", e) })
        .wait()
        .unwrap();
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
