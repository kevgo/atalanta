use std::env;

mod runnables;

fn main() {
    if let Some(mut command) = runnables::find(env::args()) {
        let status = command.status().unwrap();
        std::process::exit(status.code().unwrap());
    } else {
        println!("No command to execute found");
        std::process::exit(1);
    }
}
