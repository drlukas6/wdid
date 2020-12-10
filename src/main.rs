use structopt::StructOpt;
use std::process::Command;
use regex::Regex;

#[derive(Debug, StructOpt)]
#[structopt(name = "Time frame", about = "Number of days from when you want to get your log from")]
struct Opt {

    /// Set number of days you want to get your work log
    #[structopt(short, long, default_value = "31")]
    timeframe: u16
}

const TEST_FOLDER: &str = "/Users/lukassestic/Developer/Photomath";
const STATUS_OK: i32 = 0;

fn main() {

    let opt = Opt::from_args();

    let git_user_name_bytes = Command::new("git")
        .current_dir(TEST_FOLDER)
        .arg("config")
        .arg("user.name")
        .output()
        .expect("Could not get git user name")
        .stdout;

    let git_user_name = String::from_utf8(git_user_name_bytes)
        .expect("Error getting a git user name");

    let mut command = Command::new("git");

    command.current_dir(TEST_FOLDER);

    command.arg("log");
    command.arg(format!("--author={}", git_user_name));
    command.arg(format!("--since=\"{} days ago\"", opt.timeframe));
    command.arg("--format=date:%aD,message:%B");

    let output = command.output().expect("Could not execute git log command");

    let stdout = output.stdout;

    if output.status.code() != Option::Some(STATUS_OK) {
        eprintln!("error getting git log")
    }

    let re = Regex::new(r#"^date:([\w\D]+),message:([\w\D]+)$"#).unwrap();

    let text_output = String::from_utf8(stdout).unwrap();

    let lines = text_output.lines().filter(|line| {
        !line.is_empty() && line.contains("date") && re.is_match(line)
    });

    println!("Work log in the last {} days:", opt.timeframe);

    for text in lines {

        println!("--------------------------------------------------------");
        for cap in re.captures_iter(text) {
            println!("Date: {}\t\t{}", &cap[1], &cap[2]);
        }
    }
}
