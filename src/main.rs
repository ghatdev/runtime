use std::process::{Command};
use std::time::{Instant};

fn main() {
    let args:Vec<String>= std::env::args().collect();

    let start = Instant::now();

    let command = &args[1];
    let args_slice = &args.as_slice()[2..];

    let output = Command::new(command)
        .args(args_slice)
        .output()
        .expect("failed");

    let _result = String::from_utf8_lossy(&output.stdout);
    
    let end = Instant::now();
    let duration = end.duration_since(start);

    println!("[OUTPUT]:\n{0}\n", _result);

    print!("Duration: ");

    if duration.as_secs() > 60 {
        print!("{0}m ", duration.as_secs()/60);
    }

    if duration.subsec_millis()>1000 {
        print!("{0}s ", duration.as_secs()%60);   
    }

    println!("{0}ms.", duration.subsec_millis()%1000);
}
