/// all commands that could be run
pub enum Command {
    List,
    Unknown,
}

pub fn parse_cli_args<AS: AsRef<str>>(mut args: impl Iterator<Item = AS>) -> Command {
    match args.next() {
        Some(arg) => match arg.as_ref() {
            "-l" => Command::List,
            _ => Command::Unknown,
        },
        None => Command::List,
    }
}

pub fn execute(command: Command) -> u8 {
    0
    // if let Some(mut command) = runnables::find(env::args()) {
    //     let status = command.status().unwrap();
    //     std::process::exit(status.code().unwrap());
    // } else {
    //     println!("No command to execute found");
    //     std::process::exit(1);
    // }
}

fn main() {
    let cli_args = parse_cli_args(std::env::args());
    execute(cli_args);
}
