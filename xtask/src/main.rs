use anyhow::{anyhow, ensure, Context as _};
use approx::{abs_diff_eq, relative_eq};
use either::Either;
use env_logger::fmt::Color;
use fallible_iterator::FallibleIterator as _;
use indexmap::IndexMap;
use itertools::Itertools as _;
use log::{info, Level, LevelFilter};
use maplit::hashmap;
use once_cell::sync::Lazy;
use regex::Regex;
use scraper::{Html, Selector};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use structopt::StructOpt;
use url::Url;

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::ffi::{OsStr, OsString};
use std::io::{self, Read as _, Write as _};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::time::Instant;
use std::{env, f64, fs};

#[derive(StructOpt, Debug)]
#[structopt(bin_name("cargo"))]
enum Opt {
    Xtask(OptXtask),
}

#[derive(StructOpt, Debug)]
enum OptXtask {
    TestExamples(OptXtaskTestExamples),
}

#[derive(StructOpt, Debug)]
struct OptXtaskTestExamples {
    #[structopt(
        long,
        value_name("PATH"),
        default_value("./test-examples.toml"),
        help("Path to the config")
    )]
    config: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let Opt::Xtask(OptXtask::TestExamples(OptXtaskTestExamples { config })) = Opt::from_args();

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
        .filter_module("xtask", LevelFilter::Info)
        .init();

    let config = read_toml::<_, Config>(config)?;

    scrape_sample_cases(&config)?;
    cargo_build_examples_release()?;

    let tests = config
        .examples
        .iter()
        .map(|(slug, example)| {
            let bin = Path::new(".")
                .join("target")
                .join("release")
                .join("examples")
                .join(slug);

            match example {
                Example::Normal(Normal {
                    base: Base { name, url },
                    matching,
                    alt_testcases,
                }) => {
                    let testcases = if let Some(alt_testcases) = alt_testcases {
                        alt_testcases
                            .iter()
                            .enumerate()
                            .map(|(i, c)| {
                                ((i + 1).to_string().into(), (c.r#in.clone(), c.out.clone()))
                            })
                            .collect()
                    } else {
                        load_testcases(&config.testcases.expand_path(slug)?)?
                    };
                    Ok(Either::Left((name, url, bin, *matching, testcases)))
                }
                Example::Special(Special {
                    base: Base { name, url },
                    tester,
                }) => {
                    let tester = tester
                        .iter()
                        .map(|t| t.expand_as_arg(&bin))
                        .collect::<anyhow::Result<Vec<_>>>()?;
                    Ok(Either::Right((name, url, bin, tester)))
                }
            }
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    for test in tests {
        match test {
            Either::Left((name, url, bin, matching, testcases)) => {
                normal_test(&name, &url, matching, &testcases, &bin)?
            }
            Either::Right((name, url, bin, tester)) => special_test(&name, &url, &tester, &bin)?,
        }
    }
    Ok(())
}

fn scrape_sample_cases(config: &Config) -> anyhow::Result<()> {
    for (slug, example) in &config.examples {
        let dst_dir = config.testcases.expand_path(slug)?;
        if example.requires_sample_cases() && !dst_dir.exists() {
            let samples = get_html(&example.url())?.extract_samples()?;
            save_testcases(&dst_dir, &samples)?;
        }
    }
    Ok(())
}

fn get_html(url: &Url) -> anyhow::Result<Html> {
    static USER_AGENT: &str =
        "atcoder-rust-base <https://github.com/rust-lang-ja/atcoder-rust-base>";

    info!("GET: {}", url);

    let res = ureq::get(url.as_ref()).set("User-Agent", USER_AGENT).call();

    if let Some(err) = res.synthetic_error() {
        let mut err = err as &dyn std::error::Error;
        let mut displays = vec![err.to_string()];
        while let Some(source) = err.source() {
            displays.push(source.to_string());
            err = source;
        }
        let mut displays = displays.into_iter().rev();
        let cause = anyhow!("{}", displays.next().unwrap());
        return Err(displays.fold(cause, |err, display| err.context(display)));
    }

    info!("{} {}", res.status(), res.status_text());
    ensure!(res.status() == 200, "expected 200");
    let text = res.into_string()?;
    Ok(Html::parse_document(&text))
}

trait HtmlExt {
    fn extract_samples(&self) -> anyhow::Result<Vec<(String, String)>>;
}

impl HtmlExt for Html {
    fn extract_samples(&self) -> anyhow::Result<Vec<(String, String)>> {
        fn extract_samples(
            this: &Html,
            selector_for_header: &'static Selector,
            selector_for_content: &'static Selector,
            re_input: &'static Regex,
            re_output: &'static Regex,
        ) -> Option<Vec<(String, String)>> {
            macro_rules! static_selector {
                ($s:expr $(,)?) => {{
                    static SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse($s).unwrap());
                    &*SELECTOR
                }};
            }

            macro_rules! guard {
                ($p:expr $(,)?) => {
                    if !$p {
                        return None;
                    }
                };
            }

            let task_statement = this
                .select(static_selector!("#task-statement"))
                .exactly_one()
                .ok()
                .or_else(|| {
                    this.select(static_selector!(
                        r#"div[id="task-statement"] > div[id="task-statement"]"#,
                    ))
                    .exactly_one()
                    .ok()
                })?;

            let mut ins = BTreeMap::<usize, _>::new();
            let mut outs = BTreeMap::<usize, _>::new();
            let mut next = None;
            let selector = selector_for_header.or(selector_for_content);
            for elem_ref in task_statement.select(&selector) {
                if elem_ref.value().name() == "h3" {
                    let text = elem_ref.text().join("");
                    if let Some(caps) = re_input.captures(&text) {
                        next = Some((true, parse_possibly_zenkaku(&caps[1]).ok()?));
                    } else if let Some(caps) = re_output.captures(&text) {
                        next = Some((false, parse_possibly_zenkaku(&caps[1]).ok()?));
                    }
                } else if ["pre", "section"].contains(&elem_ref.value().name()) {
                    if let Some((is_input, n)) = next {
                        let text = elem_ref.text().join("");
                        if is_input {
                            ins.insert(n, text);
                        } else {
                            outs.insert(n, text);
                        }
                    }
                    next = None;
                }
            }

            let mut samples = ins
                .into_iter()
                .flat_map(|(idx, input)| outs.remove(&idx).map(|output| (input, output)))
                .collect::<Vec<_>>();

            for (input, output) in &mut samples {
                for s in &mut [input, output] {
                    if !(s.is_empty() || s.ends_with('\n')) {
                        s.push('\n');
                    }
                    guard!(is_valid_text(s));
                }
            }

            (!samples.is_empty()).then_(samples)
        }

        fn parse_possibly_zenkaku<T: FromStr>(s: &str) -> Result<T, T::Err> {
            s.parse().or_else(|err| {
                if s.chars().all(|c| '０' <= c && c <= '９') {
                    s.chars()
                        .map(|c| {
                            char::from((u32::from(c) - u32::from('０') + u32::from('0')) as u8)
                        })
                        .collect::<String>()
                        .parse()
                } else {
                    Err(err)
                }
            })
        }

        fn is_valid_text(s: &str) -> bool {
            s == "\n"
                || ![' ', '\n'].iter().any(|&c| s.starts_with(c))
                    && s.chars().all(|c| {
                        c.is_ascii() && (c.is_ascii_whitespace() == [' ', '\n'].contains(&c))
                    })
        }

        trait SelectorExt {
            fn or(&self, other: &Self) -> Self;
        }

        impl SelectorExt for Selector {
            fn or(&self, other: &Self) -> Self {
                let mut acc = self.clone();
                acc.selectors.extend(other.selectors.clone());
                acc
            }
        }

        macro_rules! lazy_regex {
            ($s:expr $(,)?) => {
                Lazy::new(|| Regex::new($s).unwrap())
            };
        }

        macro_rules! lazy_selector {
            ($s:expr $(,)?) => {
                Lazy::new(|| Selector::parse($s).unwrap())
            };
        }

        static IN_JA: Lazy<Regex> = lazy_regex!(r"\A[\s\n]*入力例\s*(\d{1,2})[.\n]*\z");
        static OUT_JA: Lazy<Regex> = lazy_regex!(r"\A[\s\n]*出力例\s*(\d{1,2})[.\n]*\z");
        static IN_EN: Lazy<Regex> = lazy_regex!(r"\ASample Input\s?([0-9]{1,2}).*\z");
        static OUT_EN: Lazy<Regex> = lazy_regex!(r"\ASample Output\s?([0-9]{1,2}).*\z");

        // Current style (Japanese)
        static P1_HEAD: Lazy<Selector> =
            lazy_selector!("span.lang > span.lang-ja > div.part > section > h3");
        static P1_CONTENT: Lazy<Selector> =
            lazy_selector!("span.lang > span.lang-ja > div.part > section > pre");
        // Current style (English)
        static P2_HEAD: Lazy<Selector> =
            lazy_selector!("span.lang > span.lang-en > div.part > section > h3");
        static P2_CONTENT: Lazy<Selector> =
            lazy_selector!("span.lang>span.lang-en>div.part>section>pre");
        // ARC019..ARC057 \ {ARC019/C, ARC046/D, ARC050, ARC052/{A, C}, ARC053, ARC055},
        // ABC007..ABC040 \ {ABC036}, ATC001, ATC002
        static P3_HEAD: Lazy<Selector> = lazy_selector!("div.part > section > h3");
        static P3_CONTENT: Lazy<Selector> = lazy_selector!("div.part > section > pre");
        // ARC002..ARC018, ARC019/C, ABC001..ABC006
        static P4_HEAD: Lazy<Selector> = lazy_selector!("div.part > h3,pre");
        static P4_CONTENT: Lazy<Selector> = lazy_selector!("div.part > section > pre");
        // ARC001, dwacon2018-final/{A, B}
        static P5_HEAD: Lazy<Selector> = lazy_selector!("h3,pre");
        static P5_CONTENT: Lazy<Selector> = lazy_selector!("section > pre");
        // ARC046/D, ARC050, ARC052/{A, C}, ARC053, ARC055, ABC036, ABC041
        static P6_HEAD: Lazy<Selector> = lazy_selector!("section > h3");
        static P6_CONTENT: Lazy<Selector> = lazy_selector!("section > pre");
        // ABC034
        static P7_HEAD: Lazy<Selector> = lazy_selector!("span.lang > span.lang-ja > section > h3");
        static P7_CONTENT: Lazy<Selector> =
            lazy_selector!("span.lang > span.lang-ja > section > pre");
        // practice contest (Japanese)
        static P8_HEAD: Lazy<Selector> = lazy_selector!("span.lang > span.lang-ja > div.part > h3");
        static P8_CONTENT: Lazy<Selector> =
            lazy_selector!("span.lang > span.lang-ja > div.part > section > pre");

        extract_samples(self, &P1_HEAD, &P1_CONTENT, &IN_JA, &OUT_JA)
            .or_else(|| extract_samples(self, &P2_HEAD, &P2_CONTENT, &IN_EN, &OUT_EN))
            .or_else(|| extract_samples(self, &P3_HEAD, &P3_CONTENT, &IN_JA, &OUT_JA))
            .or_else(|| extract_samples(self, &P4_HEAD, &P4_CONTENT, &IN_JA, &OUT_JA))
            .or_else(|| extract_samples(self, &P5_HEAD, &P5_CONTENT, &IN_JA, &OUT_JA))
            .or_else(|| extract_samples(self, &P6_HEAD, &P6_CONTENT, &IN_JA, &OUT_JA))
            .or_else(|| extract_samples(self, &P7_HEAD, &P7_CONTENT, &IN_JA, &OUT_JA))
            .or_else(|| extract_samples(self, &P8_HEAD, &P8_CONTENT, &IN_JA, &OUT_JA))
            .ok_or_else(|| anyhow!("Failed to scrape"))
    }
}

fn save_testcases(dir: &Path, cases: &[(String, String)]) -> anyhow::Result<()> {
    let contents = cases
        .iter()
        .enumerate()
        .flat_map(|(idx, (input, output))| {
            let file_name = format!("{}.txt", idx + 1);
            let input = (dir.join("in").join(&file_name), input);
            let output = (dir.join("out").join(file_name), output);
            vec![input, output]
        })
        .collect::<BTreeMap<_, _>>();

    for (path, contents) in contents {
        let parent = path.parent().expect("should not be root or empty");
        if !parent.exists() {
            create_dir_all(parent)?;
        }
        write(&path, contents)?;
        info!("Wrote {}", path.display());
    }
    Ok(())
}

fn load_testcases(dir: &Path) -> anyhow::Result<BTreeMap<OsString, (String, String)>> {
    let find_files = |dir_file_name: &str| -> _ {
        let dir = dir.join(dir_file_name);
        (|| -> _ {
            fs::read_dir(&dir)?
                .flat_map(|entry| {
                    entry
                        .map(|entry| {
                            let path = entry.path();
                            (path.extension() == Some("txt".as_ref())).then_with_(|| {
                                (path.file_stem().unwrap_or_default().to_owned(), path)
                            })
                        })
                        .transpose()
                })
                .collect::<io::Result<BTreeMap<_, _>>>()
        })()
        .with_context(|| format!("Failed to read {}", dir.display()))
    };

    let (ins, mut outs) = (find_files("in")?, find_files("out")?);
    ins.into_iter()
        .flat_map(|(stem, input)| outs.remove(&stem).map(|output| (stem, input, output)))
        .map(|(stem, input, output)| {
            let (input, output) = (read_to_string(input)?, read_to_string(output)?);
            Ok((stem, (input, output)))
        })
        .collect()
}

fn cargo_build_examples_release() -> anyhow::Result<()> {
    fn run_command<S1: AsRef<OsStr>, S2: AsRef<OsStr>, I: IntoIterator<Item = S2>>(
        program: S1,
        args: I,
    ) -> anyhow::Result<()> {
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

        let status = Command::new(program).args(&args).status()?;
        if !status.success() {
            return Err(anyhow!("{}: {}", program.to_string_lossy(), status));
        }
        Ok(())
    }

    run_command(
        env::var_os("CARGO").unwrap_or_else(|| "cargo".into()),
        &["build", "--examples", "--release"],
    )
}

fn normal_test(
    task_name: &str,
    url: &Url,
    matching: Matching,
    testcases: &BTreeMap<OsString, (String, String)>,
    binary: &Path,
) -> anyhow::Result<()> {
    info!("Testing {}", binary.display());
    info!("  Name: {:?}", task_name);
    info!("  URL: {}", url);

    for (case_name, (input, expected)) in testcases {
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
        info!("{:?}: {} in {}ms", case_name, verdict, time);
        if verdict != "AC" {
            return Err(anyhow!("Test failed"));
        }
    }
    Ok(())
}

fn special_test(task_name: &str, url: &Url, tester: &[OsString], bin: &Path) -> anyhow::Result<()> {
    info!("Testing {}", bin.display());
    info!("  Name: {:?}", task_name);
    info!("  URL: {}", url);
    info!("  Arguments: {:?}", tester);

    let start = Instant::now();
    let arg0 = tester.get(0).map(Deref::deref).unwrap_or_default();
    let status = Command::new(arg0)
        .args(&tester[1..])
        .status()
        .with_context(|| format!("Failed to execute {}", arg0.to_string_lossy()))?;
    let stop = Instant::now();
    let time = (stop - start).as_millis();
    let verdict = if status.success() { "AC" } else { "WA" };

    info!("{} in {}ms", verdict, time);
    if verdict != "AC" {
        return Err(anyhow!("Test failed"));
    }
    Ok(())
}

fn read_to_string(path: impl AsRef<Path>) -> anyhow::Result<String> {
    let path = path.as_ref();
    fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))
}

fn read_toml<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> anyhow::Result<T> {
    let path = path.as_ref();
    fs::read_to_string(path)
        .map_err(anyhow::Error::from)
        .and_then(|s| toml::from_str(&s).map_err(Into::into))
        .with_context(|| format!("Failed to read {}", path.display()))
}

fn write(path: impl AsRef<Path>, contents: impl AsRef<str>) -> anyhow::Result<()> {
    let (path, contents) = (path.as_ref(), contents.as_ref());
    fs::write(path, contents).with_context(|| format!("Failed to write {}", path.display()))
}

fn create_dir_all(path: impl AsRef<Path>) -> anyhow::Result<()> {
    let path = path.as_ref();
    fs::create_dir_all(path).with_context(|| format!("Failed to create {}", path.display()))
}

trait BoolExt {
    /// <https://github.com/rust-lang/rust/issues/64260>
    fn then_<T>(self, t: T) -> Option<T>;

    /// <https://github.com/rust-lang/rust/issues/64260>
    fn then_with_<T, F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> T;
}

impl BoolExt for bool {
    fn then_<T>(self, t: T) -> Option<T> {
        if self {
            Some(t)
        } else {
            None
        }
    }

    fn then_with_<T, F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> T,
    {
        if self {
            Some(f())
        } else {
            None
        }
    }
}

#[derive(Debug, Deserialize)]
struct Config {
    testcases: Template,
    examples: IndexMap<String, Example>,
}

#[derive(Debug)]
struct Template(Vec<TemplateToken>);

impl Template {
    fn expand(&self, vars: &HashMap<&str, &OsStr>) -> anyhow::Result<OsString> {
        let args = self.0.iter().map(|token| match token {
            TemplateToken::Brace(name) => vars.get(&**name).copied().ok_or_else(|| {
                anyhow!(
                    "Undefined variable {:?} (expected {:?})",
                    name,
                    vars.keys().collect::<BTreeSet<_>>(),
                )
            }),
            TemplateToken::Plain(plain) => Ok(plain.as_ref()),
        });
        fallible_iterator::convert(args).fold(OsString::new(), |mut acc, arg| {
            acc.push(arg);
            Ok(acc)
        })
    }

    fn expand_path(&self, slug: &str) -> anyhow::Result<PathBuf> {
        let vars = hashmap!("problem" => slug.as_ref());
        self.expand(&vars).map(Into::into)
    }

    fn expand_as_arg(&self, bin: &Path) -> anyhow::Result<OsString> {
        let vars = hashmap!("bin" => bin.as_ref());
        self.expand(&vars)
    }
}

impl<'de> Deserialize<'de> for Template {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use nom::branch::alt;
        use nom::bytes::complete::take_while1;
        use nom::character::complete::{alphanumeric1, char, space0};
        use nom::multi::many0;
        use nom::IResult;

        fn tokens(input: &str) -> IResult<&str, Vec<TemplateToken>> {
            many0(alt((brace, plain)))(input)
        }

        fn brace(input: &str) -> IResult<&str, TemplateToken> {
            let (input, _) = char('{')(input)?;
            let (input, _) = space0(input)?;
            let (input, name) = alphanumeric1(input)?;
            let (input, _) = space0(input)?;
            let (input, _) = char('}')(input)?;
            Ok((input, TemplateToken::Brace(name.to_owned())))
        }

        fn plain(input: &str) -> IResult<&str, TemplateToken> {
            let (input, plain) = take_while1(|c| !['{', '}'].contains(&c))(input)?;
            Ok((input, TemplateToken::Plain(plain.to_owned())))
        }

        let input = String::deserialize(deserializer)?;
        let (_, tokens) = tokens(&input).map_err(|err| match err {
            nom::Err::Incomplete(_) => unreachable!(),
            nom::Err::Error((s, k)) | nom::Err::Failure((s, k)) => serde::de::Error::custom(
                format!("{:?} at {}: {:?}", input, input.len() - s.len(), k),
            ),
        })?;
        Ok(Self(tokens))
    }
}

#[derive(Debug)]
enum TemplateToken {
    Brace(String),
    Plain(String),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum Example {
    Normal(Normal),
    Special(Special),
}

impl Example {
    fn url(&self) -> &Url {
        match self {
            Self::Normal(this) => &this.base.url,
            Self::Special(this) => &this.base.url,
        }
    }

    fn requires_sample_cases(&self) -> bool {
        match self {
            Self::Normal(this) => this.alt_testcases.is_none(),
            Self::Special(_) => false,
        }
    }
}

#[derive(Debug, Deserialize)]
struct Normal {
    #[serde(flatten)]
    base: Base,
    matching: Matching,
    alt_testcases: Option<Vec<AltTestCase>>,
}

#[derive(Debug, Deserialize)]
struct Special {
    #[serde(flatten)]
    base: Base,
    tester: Vec<Template>,
}

#[derive(Debug, Deserialize)]
struct Base {
    name: String,
    url: Url,
}

#[derive(Debug, Clone, Copy, Deserialize)]
enum Matching {
    Exact,
    Words,
    FloatOr {
        #[serde(default = "nan")]
        abs: f64,
        #[serde(default = "nan")]
        rel: f64,
    },
}

const fn nan() -> f64 {
    f64::NAN
}

impl Matching {
    fn accepts(self, expected: &str, actual: &str) -> bool {
        match self {
            Matching::Exact => expected == actual,
            Matching::Words => {
                itertools::equal(expected.split_whitespace(), actual.split_whitespace())
            }
            Matching::FloatOr { abs, rel } => itertools::diff_with(
                expected.split_whitespace(),
                actual.split_whitespace(),
                |expected, actual| {
                    if let (Ok(expected), Ok(actual)) =
                        (expected.parse::<f64>(), actual.parse::<f64>())
                    {
                        abs_diff_eq!(expected, actual, epsilon = abs)
                            || relative_eq!(expected, actual, max_relative = rel)
                    } else {
                        expected == actual
                    }
                },
            )
            .is_none(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct AltTestCase {
    r#in: String,
    out: String,
}
