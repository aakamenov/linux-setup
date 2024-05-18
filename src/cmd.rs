use std::{io, process::{Command, Stdio, Child}};

pub fn run(program: &'static str, args: &[&str]) {
    let result = Command::new(program)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn();

    handle(result, program, args)
}

pub fn run_privileged(program: &'static str, args: &[&str]) {
    let result = Command::new("sudo")
        .arg(program)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn();

    handle(result, program, args)
}

pub fn read(program: &'static str, args: &[&str]) -> String {
    let result = Command::new(program)
        .args(args)
        .output();

    match result {
        Ok(out) => match String::from_utf8(out.stdout) {
            Ok(result) => result,
            Err(_) => {
                eprintln!("Error parsing program ({program} {}) output.", args.join(" "));

                String::new()
            }
        }
        Err(err) => {
            print_err(err, program, args);

            String::new()
        }
    }
}

fn handle(result: io::Result<Child>, program: &'static str, args: &[&str]) {
    match result {
        Ok(child) => match child.wait_with_output() {
            Ok(out) => if !out.status.success() {
                eprintln!(
                    "Program exited with non-success status. Stderr:\n{}",
                    std::str::from_utf8(&out.stderr).unwrap_or_default()
                );
            }
            Err(err) => print_err(err, program, args)
        }
        Err(err) => print_err(err, program, args)
    }
}

#[inline]
fn print_err(err: io::Error, program: &'static str, args: &[&str]) {
    eprintln!("{program} {} error: {}", args.join(" "), err.to_string());
}
