mod color_out;

mod application;
use application::*;

use clap::*;
use std::io::{self, Write};

fn main() {
    let mut cmd = Command::new("my_super_puper_prog")
        .arg(
            Arg::new("exit")
                .long("exit")
                .help("exit")
                .action(ArgAction::SetTrue)
                .exclusive(true),
        )
        .no_binary_name(true);
    let mut app = App::new();
    let mut flag = true;

    cmd = App::add_args(cmd);

    while flag {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input_string = String::new();
        io::stdin().read_line(&mut input_string).unwrap();

        let input_command_vector = input_string.split_whitespace().collect::<Vec<_>>();
        let input_command = input_command_vector.as_slice();

        let try_get_matches = cmd.try_get_matches_from_mut(input_command);
        if let Ok(matches) = try_get_matches {
            if matches.get_flag("exit") {
                flag = false;
            } else {
                app.play(&matches);
            }
        } else {
            let err = try_get_matches.unwrap_err();
            let _ = err.print();
        }
    }
}
