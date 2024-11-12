mod color_out;

mod application;
use application::*;

use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    let mut app = App::new();

    println!("---------------------");
    println!("help     help program");
    println!("---------------------");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input_string = String::new();
        io::stdin().read_line(&mut input_string).unwrap();

        let input_command_vector = input_string.split_whitespace().collect::<Vec<_>>();
        let input_command = input_command_vector.as_slice();

        // input_command_vector
        //     .iter()
        //     .for_each(|s| println!(">>>>{s}"));

        if input_command.cmp(["exit"].as_slice()) == Ordering::Equal {
            break;
        } else if input_command.cmp(["help"].as_slice()) == Ordering::Equal {
            println!("---------------------");
            println!("help     help program");
            println!("exit     exit program");
            app.play(&["-h"]);
            println!("---------------------");
        } else {
            app.play(input_command);
        }
    }
}
