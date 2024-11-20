mod put;
use put::*;

mod caller;
use caller::*;

use crate::color_out::*;

use clap::*;

pub struct App {
    put: Put,
    caller: Caller,
}

impl App {
    pub fn new() -> App {
        App {
            put: Put::new(),
            caller: Caller::new(),
        }
    }
    
    pub fn add_args(cmd: Command) -> Command {
        let mut new_cmd = cmd.clone()
            .arg(
                Arg::new("list")
                    .short('l')
                    .long("list")
                    .help("list all functions")
                    .action(ArgAction::SetTrue)
                    .exclusive(true)
            );
            
        new_cmd = Put::add_args(new_cmd);
        new_cmd = Caller::add_args(new_cmd);
        
        new_cmd
    }
    
    pub fn play(&mut self, input_command: &ArgMatches) {
        if input_command.get_flag("list") {
            list();
        } else {
            self.put.update(&input_command);
            self.caller.update(&input_command);
            
            let data_in = self.put.data_in();
            let data_out;
            
            if data_in.len() > 0 {
                data_out = self.caller.play(data_in);
            } else {
                data_out = error("no in data found".to_string());
            }
            
            
            self.put.data_out(data_out);
        }
    }
}
