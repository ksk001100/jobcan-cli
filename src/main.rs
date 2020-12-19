mod actions;
mod lib;

use seahorse::{color, App, Command, Flag, FlagType};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let app = App::new(color::yellow(env!("CARGO_PKG_NAME")))
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .usage("jobcan")
        .version(env!("CARGO_PKG_VERSION"))
        .action(actions::punch_in_action)
        .flag(email_flag())
        .flag(pass_flag())
        .command(status_command())
        .command(pto_command());

    app.run(args);
}

fn status_command() -> Command {
    Command::new("status")
        .description("Show current attendance status command")
        .usage("jobcan status")
        .flag(email_flag())
        .flag(pass_flag())
        .action(actions::status_action)
        .alias("s")
}

fn pto_command() -> Command {
    Command::new("pto")
        .description("Paid holidays command")
        .usage("jobcan pto [start_date] [end_date] [reason]")
        .flag(email_flag())
        .flag(pass_flag())
        .action(actions::pto_action)
        .alias("p")
}

fn email_flag() -> Flag {
    Flag::new("email", FlagType::String)
        .description("email flag")
        .alias("e")
}

fn pass_flag() -> Flag {
    Flag::new("password", FlagType::String)
        .description("password flag")
        .alias("p")
}
