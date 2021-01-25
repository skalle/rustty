use std::io::{stdout, stdin, Write};
use std::fs::File;
use nix::pty::openpty;
use std::os::unix::io::FromRawFd;


use crossterm::{
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
    Result,
    tty::IsTty,
};
use std::process::{Child, Command, Stdio};
use std::time::Duration;

pub struct Pty {
    process: Child,
    fd: i32,
}

fn create_pty(process: &str) -> Pty {
    let ends = openpty(None,None).expect("openpty failed");
    let master = ends.master;
    let slave = ends.slave;

    let mut builder = Command::new(process);
    builder.stdin(unsafe { Stdio::from_raw_fd(slave)});
    builder.stdout(unsafe { Stdio::from_raw_fd(slave)});
    builder.stderr(unsafe { Stdio::from_raw_fd(slave)});

    match builder.spawn() {
        Ok(process) => {
            let pty = Pty{
                process,
                fd: master,
            };
            pty
        }
        Err(e) => {
            panic!("Failed to create pty: {}",e);
        }
    }
}

fn main() -> Result<()>{

    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(SetBackgroundColor(Color::Red))?
        .execute(Print("Styled Text here."))?
        .execute(ResetColor)?;

    if stdin().is_tty() {
        println!("Is a TTY");
    } else {
        println!("Is not TTY");
    };

    let shell = "/bin/bash";
    let pty = create_pty(shell);
    // println!("{:?}", pty);

    let  mut output = unsafe {File::from_raw_fd(pty.fd)};
    write!(output,"touch /tmp/itworks\n")?;
    output.flush()?;

    std::thread::sleep(Duration::new(1,0));

    println!("{}", pty.process.id());
    Ok(())
}
