//! The `oath` command.
//!
//! There are no verbs yet. `oath run`, `oath swear` and `oath hash` arrive with
//! the evaluator and the store; until then the binary only identifies itself.

use std::process::ExitCode;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const USAGE: &str = "usage: oath <command>

  -V, --version   print the version
  -h, --help      print this message

No commands yet. See ROADMAP.md for what lands next.

exit codes: 0 success, 1 runtime error, 2 usage or source error";

/// What the arguments asked for, decided before anything is printed so the
/// decision can be tested without a subprocess.
#[derive(Debug, PartialEq, Eq)]
enum Response {
    /// Write to stdout, exit 0.
    Say(String),
    /// Write to stderr, exit with this code.
    Complain(String, u8),
}

fn dispatch(args: &[String]) -> Response {
    match args {
        [] => Response::Complain(USAGE.to_string(), 2),
        [flag] if flag == "--version" || flag == "-V" => Response::Say(format!("oath {VERSION}")),
        [flag] if flag == "--help" || flag == "-h" => Response::Say(USAGE.to_string()),
        [unknown, ..] => {
            Response::Complain(format!("oath: no such command: {unknown}\n\n{USAGE}"), 2)
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match dispatch(&args) {
        Response::Say(text) => {
            println!("{text}");
            ExitCode::SUCCESS
        }
        Response::Complain(text, code) => {
            eprintln!("{text}");
            ExitCode::from(code)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dispatch_of(args: &[&str]) -> Response {
        let owned: Vec<String> = args.iter().map(|a| (*a).to_string()).collect();
        dispatch(&owned)
    }

    #[test]
    fn version_flag_prints_the_crate_version() {
        assert_eq!(
            dispatch_of(&["--version"]),
            Response::Say(format!("oath {VERSION}"))
        );
        assert_eq!(dispatch_of(&["-V"]), dispatch_of(&["--version"]));
    }

    #[test]
    fn help_flag_prints_usage_and_succeeds() {
        assert_eq!(dispatch_of(&["--help"]), Response::Say(USAGE.to_string()));
        assert_eq!(dispatch_of(&["-h"]), dispatch_of(&["--help"]));
    }

    #[test]
    fn no_arguments_is_a_usage_error() {
        assert_eq!(dispatch_of(&[]), Response::Complain(USAGE.to_string(), 2));
    }

    #[test]
    fn an_unknown_command_is_named_back_to_the_reader() {
        let Response::Complain(message, code) = dispatch_of(&["swear", "x.oath"]) else {
            panic!("an unknown command must be a usage error");
        };
        assert!(
            message.contains("swear"),
            "message should name the command: {message}"
        );
        assert_eq!(code, 2);
    }
}
