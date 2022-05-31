use crate::contest::Contest;
use anyhow::{bail, Context};
use chrono::Local;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use std::{io::BufRead, path::PathBuf, process::Command, str::FromStr, thread};

pub fn start() -> anyhow::Result<()> {
    let config = crate::config::read_config()?;

    let upcoming_contests = crate::contest::get_upcoming_contests()?;
    let next_contest = upcoming_contests
        .first()
        .with_context(|| "No upcoming contests are found.")?;

    crate::contest::print_contests(&[next_contest.clone()])?;

    let Contest {
        name,
        id,
        start_time,
        ..
    } = next_contest;

    let path = PathBuf::from_str(&config.cargo_compete_home_dir)?
        .join("contests")
        .join(&id);

    let now = Local::now();
    if now < *start_time {
        let wait_second = (*start_time - now).num_seconds() as u64;
        let bar = ProgressBar::new(wait_second);
        bar.set_message(format!("waiting for the {} open", name));
        bar.set_draw_rate(2);
        bar.set_style(
            ProgressStyle::default_bar().template("[{eta_precise}] {bar:50.green} {msg}"),
        );

        while !bar.is_finished() {
            let elapsed = (Local::now() - now).num_seconds() as u64;
            bar.set_position(elapsed);
            thread::sleep(std::time::Duration::from_secs(1));
            if elapsed > wait_second {
                bar.finish_with_message("Contest Starts");
            }
        }
    }

    if !path.as_path().exists() {
        let out = Command::new("cargo-compete").args(["new", id]).output()?;
        if out.status.success() {
            eprintln!("Created a package for {}", id);
        } else {
            bail!("failed to execute 'cargo compete new {}'", id);
        }
    }

    if config.open_tasks_print {
        let _ = open::that(format!("https://atcoder.jp/contests/{}/tasks_print", id))
            .with_context(|| "failed to open tasks page in the browser")?;
    }

    let bin_paths = Command::new("exa")
        .arg(path.join("src/bin/").as_os_str())
        .arg("-1")
        .args(["--colour", "never"])
        .output()
        .unwrap()
        .stdout
        .lines()
        .flatten()
        .map(|p| path.join("src/bin").join(p))
        .collect_vec();

    let _ = Command::new("code").arg(path).args(bin_paths).output()?;

    eprintln!("Good Luck🚀");

    Ok(())
}

#[test]
fn preogress_bar() {
    let now = Local::now();
    let wait_second = 3;
    let bar = ProgressBar::new(wait_second);
    bar.set_draw_rate(2);
    bar.set_message("waiting for the contest open");
    bar.set_style(ProgressStyle::default_bar().template("[{eta_precise}] {bar:50.green} {msg}"));
    while !bar.is_finished() {
        let elapsed = (Local::now() - now).num_seconds() as u64;
        bar.set_position(elapsed);
        if elapsed > wait_second {
            bar.finish_with_message("Contest Start");
        }
    }
}

#[test]
fn open_tasks_print() {
    open::that("https://atcoder.jp/contests/abc252/tasks_print").unwrap();
}

#[test]
fn open_code() {
    let path = PathBuf::from("/Users/roku/room/comp/atcoder/contests/abc252");
    let bin_paths = Command::new("exa")
        .arg(path.join("src/bin/").as_os_str())
        .arg("-1")
        .args(["--colour", "never"])
        .output()
        .unwrap()
        .stdout
        .lines()
        .flatten()
        .map(|p| path.join("src/bin").join(p))
        .collect_vec();

    let _ = Command::new("code").arg(path).args(bin_paths).output();
}
