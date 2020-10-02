mod lib;

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
        (c.args[0].clone(), c.args[1].clone(), String::new())
    } else if c.args.len() >= 3 {
        (c.args[0].clone(), c.args[1].clone(), c.args[2..].join(" "))
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
        .usage("jobcan --email(-e) [email]")
        .alias("e");
    let pass_flag = Flag::new("email", FlagType::String)
        .usage("jobcan --password(-p) [password]")
        .alias("p");

    let status_command = Command::new("status")
        .usage("jobcan status")
        .flag(
            email_flag
                .clone()
                .usage("jobcan status(s) --email(-e) [email]"),
        )
        .flag(
            pass_flag
                .clone()
                .usage("jobcan status(s) --password(-p) [password]"),
        )
        .action(status_action)
        .alias("s");

    let pto_command = Command::new("pto")
        .usage("jobcan pto [start_date] [end_date] [reason]")
        .flag(
            email_flag
                .clone()
                .usage("jobcan pto(p) [start_date] [end_date] [reason] --email(-e) [email]"),
        )
        .flag(
            pass_flag
                .clone()
                .usage("jobcan pto(p) [start_date] [end_date] [reason] --password(-p) [password]"),
        )
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
