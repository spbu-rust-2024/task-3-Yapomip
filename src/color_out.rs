
use colored::Colorize;

pub fn message(warning_string: String) -> String
{
    format!("{}: {}",
        "message".truecolor(90, 26, 212),
        warning_string
    )
}

pub fn warning(warning_string: String) -> String
{
    format!("{}: {}",
        "exit".truecolor(90, 212, 12),
        warning_string
    )
}


pub fn error(error_string: String) -> String
{
    format!("{}: {}",
        "exit".truecolor(254, 47, 12),
        error_string
    )
}
