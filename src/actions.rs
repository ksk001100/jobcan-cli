use crate::lib::Jobcan;
use chrono::prelude::*;
use headless_chrome::Browser;
use seahorse::{color, Context};
use spinners::{Spinner, Spinners};
use std::{env, process};

pub fn punch_in_action(c: &Context) {
    let sp = Spinner::new(Spinners::Moon, color::green("Waiting..."));
    let jobcan = Jobcan::new(email(c), password(c));
    let browser = Browser::default().unwrap();
    let tab = browser.new_tab().unwrap();

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

pub fn status_action(c: &Context) {
    let sp = Spinner::new(Spinners::Moon, color::green("Waiting..."));
    let jobcan = Jobcan::new(email(c), password(c));
    let browser = Browser::default().unwrap();
    let tab = browser.new_tab().unwrap();

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

pub fn pto_action(c: &Context) {
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

    let jobcan = Jobcan::new(email(c), password(c));
    let browser = Browser::default().unwrap();
    let tab = browser.new_tab().unwrap();

    match jobcan.login(&tab) {
        Ok(_) => match jobcan.pto(&tab, start_date, end_date, reason) {
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

fn email(c: &Context) -> String {
    match c.string_flag("email") {
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
    }
}

fn password(c: &Context) -> String {
    match c.string_flag("password") {
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
    }
}
