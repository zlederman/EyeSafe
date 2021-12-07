extern crate job_scheduler;
extern crate daemonize;
use std::process::Command;
use job_scheduler::{JobScheduler, Job};
use std::time::Duration;
use std::fs::File;
use daemonize::Daemonize;

struct OSAParams {
    notif_statement : String,
    title : String,
    sound : String
}

impl OSAParams {
    fn build_notif(self) -> String {
        return format!("display notification \"{}\" with title \"{}\" sound name \"{}\"",
        self.notif_statement,
        self.title,
        self.sound
        )
    } 
}

fn main() {
    let stdout = File::create("/tmp/daemon.out").unwrap();
    let stderr = File::create("/tmp/daemon.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/test.pid") // Every method except `new` and `start`
        .chown_pid_file(true)      // is optional, see `Daemonize` documentation
        .working_directory("/tmp") // for default behaviour.
        .user("nobody")
        .group("daemon") // Group name
        .group(2)        // or group id.
        .umask(0o777)    // Set umask, `0o027` by default.
        .stdout(stdout)  // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr)  // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => println!("Success, daemonized"),
        Err(e) => eprintln!("Error, {}", e),
    }

    let cmd = OSAParams {
        notif_statement : String::from("Its Time To Look Up For 20 Seconds!"),
        title: String::from("👁  👃 👁"),
        sound: String::from("Funk")
    }.build_notif();

    let mut sched = JobScheduler::new();
    sched.add(
        Job::new("0 */20 * * * *".parse().unwrap(), || {
            Command::new("osascript")
            .arg("-e")
            .arg(cmd.clone())
            .output().expect("notification failed");
        })
    );
    loop {
        sched.tick();
        std::thread::sleep(Duration::from_millis(500));
    }

  

    
}

