use clap::*;

mod calculate;
use calculate::caller;

pub mod put {
    use clap::*;
    use std::fs;
    use std::io;
    use std::path;

    pub fn add_args(cmd: Command) -> Command {
        cmd.clone()
            .args(&[
                arg!(-i --input <FILE> "input file name, else console input"),
                arg!(-o --output <FILE> "output file name, else console output"),
            ])
            .group(
                ArgGroup::new("put")
                    .args(["input", "output"])
                    .requires("functions"),
            )
    }
    pub fn input(matches: &ArgMatches) -> Vec<i64> {
        if let Some(string_file_path) = matches.get_one::<String>("input") {
            let path = path::Path::new(string_file_path);
            let display = path.display();

            let s = fs::read_to_string(path).expect(&format!("error of read \"{}\" file ", display));

            let data = s
                .split_whitespace()
                .map(|x| {
                    x.parse::<i64>()
                        .expect(&format!("error of parse \"{}\" to int ", x))
                })
                .collect::<Vec<i64>>();

            data
        } else {
            println!("Please input the data:");
            
            let mut input = String::new();
            
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read file");


            let data = input
                .split_whitespace()
                .map(|x| {
                    x.parse::<i64>()
                        .expect(&format!("error of parse \"{}\" to int ", x))
                })
                .collect::<Vec<i64>>();

            data
        }
    }
    pub fn output(matches: &ArgMatches, out: String) {
        if let Some(string_file_path) = matches.get_one::<String>("output") {
            fs::write(string_file_path, out)
                .expect(&format!("Failed write to {}", string_file_path));
        } else {
            println!("{}", out);
        }
    }
}

fn main() {
    let mut cmd = Command::new("").arg(
        Arg::new("list")
            .short('l')
            .long("list")
            .help("list all functions")
            .action(ArgAction::SetTrue)
            .exclusive(true),
    );
    cmd = put::add_args(cmd);
    cmd = caller::add_args(cmd);
    // let mut functions = Command::new("functions").about("call functions").arg(
    //     Arg::new("list")
    //         .short('l')
    //         .long("list")
    //         .help("list all functions")
    //         .action(ArgAction::SetTrue)
    //         .exclusive(true),
    // );
    // functions = put::add_args(functions);
    // functions = caller::add_args(functions);
    // cmd = cmd.clone().subcommand(functions);

    let matches = cmd.get_matches();

    // dbg!(&matches);

    if matches.get_flag("list") {
        let s = [
            "All functions",
            "  1 -- arithmetic mean",
            "  2 -- geometric mean",
            "  3 -- generalized mean",
            "  4 -- arithmetic geometric mean",
            "  5 -- modified arithmetic geometric mean",
            "  6 -- quasi arithmetic mean",
            "  7 -- truncated mean",
            "  8 -- winsorizing mean",
            "  9 -- median mean",
            "  a -- moda mean",
            "  b -- average linear deviation",
            "  c -- average quadratic deviation",
            "  d -- linear coefficient variation",
            "  e -- quadratic coefficient variation",
            "  f -- variance",
        ];
        s.iter().for_each(|s| println!("{}", s));
    } else if matches.contains_id("functions") {
        let data = put::input(&matches);
        let out = caller::call(&data, &matches);

        put::output(&matches, out);
    }
}
