use std::io::{self, Write};
use std::process::Command;

use clap::Parser;

enum Compiler {
    gfortran,
    ifort,
    flang,
    lfortran
}

impl From<&str> for Compiler {
    fn from(s: &str) -> Compiler {
        match s {
            "gfortran" => Compiler::gfortran,
            "ifort" => Compiler::ifort,
            "flang" => Compiler::flang,
            "lfortran" => Compiler::lfortran,
        }
    }
}

impl From<Compiler> for String {
    fn from(c: Compiler) -> String {
        match c {
            Compiler::gfortran => "gfortran",
            Compiler::ifort => "ifort",
            Compiler::flang => "flang",
            Compiler::lfortran => "lfortran",
            _ => "ERROR" // but how do I make this throw an error? need to learn more rust
        }
    }
}

/// Invoke a certain Fortran compiler to compile and run a fortran program
#[derive(Parser, Debug)]
struct Cli {
    /// The compiler to use
    compiler: Compiler,
    /// The path to the file to read
    // path: std::path::PathBuf,
    filename: String,
    extension: String,
}

fn main() {
    let args = Cli::parse();

    let compiler = args.compiler;
    let filename = args.filename;
    let extension = args.extension;

    println!(
        "compiler: {:?}, filename: {:?}, extension: {:?}",
        compiler, filename, extension
    );

    let output = Command::new(compiler)
        .arg(format!("{}.{}", filename, extension))
        .args([
            "-o",
            &filename,
        ])
        .output()
        .expect("failed to compile fortran source program");

    let output = Command::new(format!("./{}", filename))
        .output()
        .expect("failed to run fortran-compiled binary");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());
}

/*
fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    let args = Cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path),
    };

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}
*/
