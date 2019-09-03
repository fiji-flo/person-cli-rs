use person_cli;

use std::env;

use person_cli::app;

fn main() -> Result<(), String> {
    app::run(env::args_os())
}
