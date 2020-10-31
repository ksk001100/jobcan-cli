mod lib;

use chrono::prelude::*;
use headless_chrome::Browser;
use lib::Jobcan;
use seahorse::{color, App, Command, Context, Flag, FlagType};
use spinners::{Spinner, Spinners};
use std::{env, process};

fn action(c: &Context) {
    let sp = Spinner::new(Spinners::Moon, color::green("Waiting..."));

    let email = match c.string_flag("email") {
        Ok(email) => email,
        Err(_) => match env::var("JOBCAN_EMAIL") {
            Ok(email) => email,
            Err(_) => {
                eprintln!(
                    "\r{}",
                    color::red("Not found enviroment variable \"JOBCAN_EMAIL\".")
                );
                process::exit(1);
            }
        },
    };
    let password = match c.string_flag("password") {
        Ok(pass) => pass,
        Err(_) => match env::var("JOBCAN_PASSWORD") {
            Ok(pass) => pass,
            Err(_) => {
                eprintln!(
                    "\r{}",
                    color::red("Not found enviroment variable \"JOBCAN_PASSWORD\".")
                );
                process::exit(1);
            }
        },
    };

    let jobcan = Jobcan::new(email, password);
    let browser = Browser::default().unwrap();
    let tab = browser.wait_for_initial_tab().unwrap();

    match jobcan.login(&tab) {
        Ok(_) => match jobcan.punch_in(&tab) {
            Ok(s) => println!("\r{}", s),
            Err(_) => {
                eprintln!("\rFailed punch in.");
                process::exit(1);
            }
        },
        Err(_) => {
            eprintln!("\rFailed login.");
            process::exit(1);
        }
    }

    sp.stop();
}

fn status_action(c: &Context) {
    let sp = Spinner::new(Spinners::Moon, color::green("Waiting..."));

    let email = match c.string_flag("email") {
        Ok(email) => email,
        Err(_) => match env::var("JOBCAN_EMAIL") {
            Ok(email) => email,
            Err(_) => {
                eprintln!(
                    "\r{}",
                    color::red("Not found enviroment variable \"JOBCAN_EMAIL\".")
                );
                process::exit(1);
            }
        },
    };
    let password = match c.string_flag("password") {
        Ok(pass) => pass,
        Err(_) => match env::var("JOBCAN_PASSWORD") {
            Ok(pass) => pass,
            Err(_) => {
                eprintln!(
                    "\r{}",
                    color::red("Not found enviroment variable \"JOBCAN_PASSWORD\".")
                );
                process::exit(1);
            }
        },
    };

    let jobcan = Jobcan::new(email, password);
    let browser = Browser::default().unwrap();
    let tab = browser.wait_for_initial_tab().unwrap();

    match jobcan.login(&tab) {
        Ok(_) => match jobcan.get_status(&tab) {
            Ok(s) => println!("\rステータス : {}", color::green(s)),
            Err(_) => {
                eprintln!("\rFailed get status.");
                process::exit(1);
            }
        },
        Err(_) => {
            eprintln!("\rFailed login.");
            process::exit(1);
        }
    }

    sp.stop();
}

fn pto_action(c: &Context) {
    let sp = Spinner::new(Spinners::Moon, color::green("Waiting..."));

    let (start_date, end_date, reason) = if c.args.len() == 2 {
        let start_date = match NaiveDate::parse_from_str(&c.args[0], "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => {
                eprintln!(
                    "\r{}",
                    color::red("start_date format error. (ex: \"2020-01-01\")")
                );
                std::process::exit(1);
            }
        };
        let end_date = match NaiveDate::parse_from_str(&c.args[1], "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => {
                eprintln!(
                    "\r{}",
                    color::red("end_date format error. (ex: \"2020-01-01\")")
                );
                std::process::exit(1);
            }
        };

        (start_date, end_date, String::new())
    } else if c.args.len() >= 3 {
        let start_date = match NaiveDate::parse_from_str(&c.args[0], "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => {
                eprintln!(
                    "\r{}",
                    color::red("start_date format error. (ex: \"2020-01-01\")")
                );
                std::process::exit(1);
            }
        };
        let end_date = match NaiveDate::parse_from_str(&c.args[1], "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => {
                eprintln!(
                    "\r{}",
                    color::red("end_date format error. (ex: \"2020-01-01\")")
                );
                std::process::exit(1);
            }
        };

        (start_date, end_date, c.args[2..].join(" "))
    } else {
        eprintln!("\r{}", color::red("Arugment error."));
        process::exit(1);
    };

    let email = match c.string_flag("email") {
        Ok(email) => email,
        Err(_) => match env::var("JOBCAN_EMAIL") {
            Ok(email) => email,
            Err(_) => {
                eprintln!(
                    "\r{}",
                    color::red("Not found enviroment variable \"JOBCAN_EMAIL\".")
                );
                process::exit(1);
            }
        },
    };
    let password = match c.string_flag("password") {
        Ok(pass) => pass,
        Err(_) => match env::var("JOBCAN_PASSWORD") {
            Ok(pass) => pass,
            Err(_) => {
                eprintln!(
                    "\r{}",
                    color::red("Not found enviroment variable \"JOBCAN_PASSWORD\".")
                );
                process::exit(1);
            }
        },
    };

    let jobcan = Jobcan::new(email, password);
    let browser = Browser::default().unwrap();
    let tab = browser.wait_for_initial_tab().unwrap();

    match jobcan.login(&tab) {
        Ok(_) => match jobcan.pto(&tab, start_date.clone(), end_date.clone(), reason) {
            Ok(_) => println!(
                "\r有給休暇申請 : {} ~ {}",
                color::green(start_date),
                color::green(end_date)
            ),
            Err(e) => {
                eprintln!("\rFailed get status.");
                println!("\n{}", e);
                process::exit(1);
            }
        },
        Err(_) => {
            eprintln!("\rFailed login.");
            process::exit(1);
        }
    }

    sp.stop();
}

fn main() {
    let args: Vec<String> = ::std::env::args().collect();

    let email_flag = Flag::new("email", FlagType::String)
        .description("email flag")
        .alias("e");
    let pass_flag = Flag::new("email", FlagType::String)
        .description("password flag")
        .alias("p");

    let status_command = Command::new("status")
        .description("Show current attendance status command")
        .usage("jobcan status")
        .flag(email_flag.clone())
        .flag(pass_flag.clone())
        .action(status_action)
        .alias("s");

    let pto_command = Command::new("pto")
        .description("Paid holidays command")
        .usage("jobcan pto [start_date] [end_date] [reason]")
        .flag(email_flag.clone())
        .flag(pass_flag.clone())
        .action(pto_action)
        .alias("p");

    let app = App::new(color::blue(env!("CARGO_PKG_NAME")))
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .usage("jobcan")
        .version(env!("CARGO_PKG_VERSION"))
        .action(action)
        .flag(email_flag)
        .flag(pass_flag)
        .command(status_command)
        .command(pto_command);

    app.run(args);
}
