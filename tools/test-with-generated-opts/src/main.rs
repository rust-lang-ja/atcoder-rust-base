use anyhow::{anyhow, Context as _};
use env_logger::fmt::Color;
use indexmap::IndexMap;
use itertools::Itertools as _;
use log::{info, Level, LevelFilter};
use serde::Deserialize;
use structopt::StructOpt;
use tempdir::TempDir;

use std::collections::{BTreeMap, HashMap};
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::{self, Read as _, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::Instant;

#[derive(StructOpt, Debug)]
struct Opt {}

fn main() -> anyhow::Result<()> {
    Opt::from_args();

    env_logger::builder()
        .format(|buf, record| {
            let mut style = buf.style();
            let mut write_with_style = |color, bold, intense, value| -> _ {
                let value = style
                    .set_color(color)
                    .set_bold(bold)
                    .set_intense(intense)
                    .value(value);
                write!(buf, "{}", value)
            };
            write_with_style(Color::Black, false, true, "[")?;
            match record.level() {
                Level::Info => write_with_style(Color::Cyan, true, false, "INFO"),
                Level::Warn => write_with_style(Color::Yellow, true, false, "WARN"),
                Level::Error => write_with_style(Color::Red, true, false, "ERROR"),
                _ => unreachable!(),
            }?;
            write_with_style(Color::Black, false, true, "]")?;
            writeln!(buf, " {}", record.args())
        })
        .filter_module("test_with_generated_opts", LevelFilter::Info)
        .init();

    let Tests { tests } = File::open("./examples/tests.ron")
        .map_err(anyhow::Error::from)
        .and_then(|h| ron::de::from_reader(h).map_err(Into::into))
        .with_context(|| "Failed to read ./examples/tests.ron")?;

    let tempdir = TempDir::new("atcoder-rust-base-test-with-generated-opts")?;

    let tests = tests
        .into_iter()
        .map(|(slug, Test { name, matching })| {
            let src = Path::new("./examples").join(&slug).with_extension("rs");
            let testsets = Path::new("./examples/testsets").join(&slug);
            let binary = compile(&src, tempdir.path(), &slug)?;
            Ok((name, matching, testsets, binary))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    for (name, matching, testsets, binary) in tests {
        test(&name, matching, &testsets, &binary)?;
    }
    Ok(())
}

fn compile(src: &Path, tempdir: &Path, dir_name: &str) -> anyhow::Result<PathBuf> {
    fn run_command<S1: AsRef<OsStr>, S2: AsRef<OsStr>, I: IntoIterator<Item = S2>>(
        program: S1,
        args: I,
    ) -> anyhow::Result<Vec<u8>> {
        let program = program.as_ref();
        let args = args.into_iter().collect::<Vec<_>>();

        info!(
            "Running `{}{}`",
            shell_escape::escape(program.to_string_lossy()),
            args.iter()
                .map(AsRef::as_ref)
                .map(OsStr::to_string_lossy)
                .map(shell_escape::escape)
                .format_with("", |s, f| f(&format_args!(" {}", s))),
        );

        let Output { status, stdout, .. } = Command::new(program)
            .args(&args)
            .stdin(Stdio::null())
            .stderr(Stdio::inherit())
            .output()?;

        if !status.success() {
            return Err(anyhow!("{}: {}", program.to_string_lossy(), status));
        }
        Ok(stdout)
    }

    let generated_opts = {
        let program = which::which("rustc-dep-option-generator")?;
        let stdout = run_command(&program, &["--format", "json"])?;
        serde_json::from_slice::<Vec<String>>(&stdout)
            .with_context(|| format!("{}: invalid output", program.to_string_lossy()))?
    };

    let out = tempdir
        .join(dir_name)
        .with_extension(if cfg!(windows) { "exe" } else { "" });

    let program = env::var_os("RUSTC")
        .map(Ok)
        .unwrap_or_else(|| which::which("rustc").map(Into::into))?;

    let args = {
        let mut args = vec![
            OsString::from("--edition"),
            OsString::from("2018"),
            OsString::from("-C"),
            OsString::from("opt-level=3"),
            OsString::from("-o"),
            OsString::from(&out),
        ];
        for opt in generated_opts {
            args.push(opt.into());
        }
        args.push(src.to_owned().into());
        args
    };

    run_command(program, args)?;
    Ok(out)
}

fn test(task_name: &str, matching: Matching, testsets: &Path, binary: &Path) -> anyhow::Result<()> {
    let testsets = {
        let find_files = |dir: &str| -> _ {
            fs::read_dir(testsets.join(dir))?
                .map(|entry| {
                    let path = entry?.path();
                    let name = path
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned();
                    Ok((name, path))
                })
                .collect::<io::Result<HashMap<_, _>>>()
        };

        let (ins, outs) = (find_files("in")?, find_files("out")?);

        ins.into_iter()
            .flat_map(|(stem, path_in)| {
                outs.get(&stem)
                    .map(|path_out| (stem, (path_in, path_out.clone())))
            })
            .collect::<BTreeMap<_, _>>()
    };

    info!("Testing {} for {:?}", binary.display(), task_name);

    for (test_name, (path_in, path_out)) in testsets {
        fn read_to_string(path: &Path) -> anyhow::Result<String> {
            fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))
        }

        let input = read_to_string(&path_in)?;
        let expected = read_to_string(&path_out)?;
        let start = Instant::now();

        let mut child = Command::new(binary)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .with_context(|| format!("Failed to execute {}", binary.display()))?;

        child.stdin.as_mut().unwrap().write_all(input.as_ref())?;
        child.stdin.take();
        let actual = {
            let mut actual = "".to_owned();
            child
                .stdout
                .as_mut()
                .unwrap()
                .read_to_string(&mut actual)
                .with_context(|| format!("{} outputted invalid UTF-8", binary.display()))?;
            actual
        };
        let status = child.wait()?;
        let stop = Instant::now();

        let time = (stop - start).as_millis();
        let verdict = if status.success() && matching.accepts(&expected, &actual) {
            "AC"
        } else if status.success() {
            "WA"
        } else {
            "RE"
        };
        info!("{}: {} in {}ms", test_name, verdict, time);
        if verdict != "AC" {
            return Err(anyhow!("Test failed"));
        }
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
struct Tests {
    tests: IndexMap<String, Test>,
}

#[derive(Debug, Deserialize)]
struct Test {
    name: String,
    matching: Matching,
}

#[derive(Debug, Clone, Copy, Deserialize)]
enum Matching {
    ExactWhole,
    ExactWords,
}

impl Matching {
    fn accepts(self, expected: &str, actual: &str) -> bool {
        match self {
            Matching::ExactWhole => expected == actual,
            Matching::ExactWords => {
                itertools::equal(expected.split_whitespace(), actual.split_whitespace())
            }
        }
    }
}
