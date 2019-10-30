extern crate daemonize;

use std::fs::File;

use daemonize::Daemonize;

fn main() {
    let stdout = File::create("/tmp/daemon.out").unwrap();
    let stderr = File::create("/tmp/daemon.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/test.pid")
        .chown_pid_file(true)
        .working_directory("/tmp")
        .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr); // Redirect stderr to `/tmp/daemon.err`.
                         //.exit_action(|| println!("Executed before master process exits"))
                         //.privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => {
            println!("Success, daemonized");

            println!("another thing in here!");
        }
        Err(e) => eprintln!("Error, {}", e),
    };

    println!("and one more thing");
}
