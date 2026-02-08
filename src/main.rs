use affirmations::{LONGDAY, NEUTRAL, NVIM, SENIOR, SWAYWM, VETERAN};
use colorize::AnsiColor;
use rand::random_range;
use sysinfo::System;
mod affirmations;

struct Report {
    pub is_using_vim: bool,
    pub is_using_sway: bool,
    pub is_working_long_time: bool,
}

impl Report {
    fn new() -> Report {
        let mut is_using_vim = false;
        let mut is_using_sway = false;

        let mut sys = System::new_all();
        sys.refresh_all();
        for (_, process) in sys.processes() {
            if process.name().to_string_lossy().contains("vim") {
                is_using_vim = true;
            }
            if process.name().to_string_lossy().contains("sway") {
                is_using_sway = true;
            }
        }

        let is_working_long_time = System::uptime() > (3600 * 5);

        Report {
            is_using_vim,
            is_using_sway,
            is_working_long_time,
        }
    }
}

fn get_affermation(report: Report) -> String {
    match random_range(0..1000) {
        1 => "got a shiny affimation.".magenta(),
        x if x < 500 && report.is_working_long_time => {
            LONGDAY[rand::random_range(0..LONGDAY.len())].red()
        }
        x if x < 200 && report.is_using_vim => NVIM[rand::random_range(0..NVIM.len())].blue(),
        x if x > 200 && x < 400 && report.is_using_sway => {
            SWAYWM[rand::random_range(0..SWAYWM.len())].blue()
        }
        x if x > 400 && x < 600 => SENIOR[rand::random_range(0..SENIOR.len())].cyan(),
        x if x > 600 && x < 700 => VETERAN[rand::random_range(0..VETERAN.len())].yellow(),
        _ => NEUTRAL[rand::random_range(0..NEUTRAL.len())].to_string(),
    }
}

fn main() {
    // Be mindful
    if random_range(0..5) > 0 {
        return;
    }
    let name = whoami::account().unwrap_or_else(|_| "The developer".to_string());
    let report = Report::new();
    println!("✱ {} {}", name.green(), get_affermation(report));
}
