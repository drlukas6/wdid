use structopt::StructOpt;
use std::process::Command;
use regex::Regex;
use termion::color;

#[derive(Debug, StructOpt)]
#[structopt(name = "Time frame", about = "Number of days from when you want to get your log from")]
struct Opt {

    /// Set number of days you want to get your work log
    #[structopt(short, long, default_value = "31")]
    timeframe: u16
}

const STATUS_OK: i32 = 0;

fn main() {

    let timeframe = Opt::from_args().timeframe;

    // Getting the git username from config
    let git_user_name_bytes = Command::new("git")
        .arg("config")
        .arg("user.name")
        .output()
        .expect("Could not get git user name")
        .stdout;

    let git_user_name = String::from_utf8(git_user_name_bytes)
        .expect("Error getting a git user name");

    // Getting the log matching te current user in the given timeframe
    let log_output = Command::new("git")
        .arg("log")
        .arg(format!("--author={}", git_user_name))
        .arg(format!("--since=\"{} days ago\"", timeframe))
        .arg("--format=date:%aD,message:%B")
        .output()
        .expect("Could not execute git log command");

    if log_output.status.code() != Option::Some(STATUS_OK) {
        eprintln!("error getting git log")
    }

    let re = Regex::new(r#"^date:([\w\D]+),message:([\w\D]+)$"#).unwrap();

    let text_output = String::from_utf8(log_output.stdout).unwrap();

    let lines = text_output.lines().filter(|line| {
        !line.is_empty() && line.contains("date") && re.is_match(line)
    });

    println!("{}Work log in the last {} days:", color::Fg(color::Cyan),timeframe);

    for text in lines {

        println!("{}--------------------------------------------------------", color::Fg(color::LightBlack));
        for cap in re.captures_iter(text) {
            println!("{}{}\t\t{}{}", color::Fg(color::Yellow), &cap[1], color::Fg(color::Green), &cap[2]);
        }
    }

    println!("{}--------------------------------------------------------", color::Fg(color::LightBlack));
}
