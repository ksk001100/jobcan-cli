use headless_chrome::Browser;
use seahorse::{color, App, Context, Flag, FlagType};
use spinners::{Spinner, Spinners};
use std::env;
use std::process;
use std::thread;
use std::time::Duration;

fn jobcan_punch_in(email: String, password: String) -> Result<String, failure::Error> {
    let browser = Browser::default()?;
    let tab = browser.wait_for_initial_tab()?;
    let url = "https://id.jobcan.jp/users/sign_in?app_key=atd";

    tab.navigate_to(url)?;

    tab.wait_for_element("input#user_email")?.click()?;
    tab.type_str(&email)?;

    tab.wait_for_element("input#user_password")?.click()?;
    tab.type_str(&password)?;

    tab.wait_for_element("input.form__login")?.click()?;

    tab.wait_for_url("https://ssl.jobcan.jp/employee")?;

    let before_status = tab
        .wait_for_element("div#working_status")?
        .get_description()?
        .find(|n| n.node_name == "#text")
        .unwrap()
        .node_value
        .to_owned();

    tab.wait_for_element("p#adit-button-push")?.click()?;
    thread::sleep(Duration::from_secs(2));

    let after_status = tab
        .wait_for_element("div#working_status")?
        .get_description()?
        .find(|n| n.node_name == "#text")
        .unwrap()
        .node_value
        .to_owned();

    Ok(format!(
        "{} -> {}",
        color::yellow(before_status),
        color::green(after_status)
    ))
}

fn action(c: &Context) {
    let sp = Spinner::new(Spinners::Moon, color::green("Waiting..."));

    let email = match c.string_flag("email") {
        Some(email) => email,
        None => match env::var("JOBCAN_EMAIL") {
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
        Some(pass) => pass,
        None => match env::var("JOBCAN_PASSWORD") {
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
    match jobcan_punch_in(email, password) {
        Ok(s) => println!("\r{}", s),
        Err(_) => {
            eprintln!("\rFailed punch in.");
            process::exit(1);
        }
    }
    sp.stop();
}

fn main() {
    let args: Vec<String> = ::std::env::args().collect();

    let email_flag = Flag::new("email", "jobcan --email(-e) [email]", FlagType::String).alias("e");
    let pass_flag =
        Flag::new("email", "jobcan --password(-p) [password]", FlagType::String).alias("p");

    let app = App::new()
        .name(color::blue(env!("CARGO_PKG_NAME")))
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .usage("jobcan")
        .version(env!("CARGO_PKG_VERSION"))
        .action(action)
        .flag(email_flag)
        .flag(pass_flag);

    app.run(args);
}
