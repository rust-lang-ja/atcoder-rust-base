use cargo::core::compiler::{self, CompileMode, TargetInfo};
use cargo::core::dependency;
use cargo::core::package::{Package, PackageSet};
use cargo::core::resolver::ResolveOpts;
use cargo::core::shell::{Shell, Verbosity};
use cargo::core::{PackageId, PackageIdSpec, Resolve, Workspace};
use cargo::ops::{Packages, TestOptions};
use cargo::util::command_prelude::{App, AppExt as _, AppSettings, ArgMatchesExt as _};
use cargo::{CliError, CliResult};
use failure::{format_err, Fail as _, Fallible};
use itertools::Itertools as _;
use maplit::btreeset;
use once_cell::sync::Lazy;
use serde::Deserialize;
use structopt::StructOpt;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::env;
use std::fmt::{Display, Write as _};
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::process::ExitStatus;

static MANIFEST_PATH: &str = "./Cargo.toml";
static CONFIG_PATH: &str = "./dep-tests.toml";

fn main() {
    debug_assert_eq!(
        env::current_dir()
            .ok()
            .and_then(|d| d.file_name()?.to_str().map(ToOwned::to_owned)),
        Some("atcoder-rust-base".to_owned()),
        "The cwd should be \"atcoder-rust-base\"",
    );

    let matches = Opt::clap()
        .get_matches_safe()
        .unwrap_or_else(|e| cargo::exit_with_error(e.into(), &mut Shell::new()));
    let opt = Opt::from_clap(&matches);

    let mut config = cargo::Config::default()
        .unwrap_or_else(|e| cargo::exit_with_error(e.into(), &mut Shell::new()));

    if let Err(err) = opt.run(&mut config) {
        cargo::exit_with_error(err, &mut config.shell());
    }
}

#[derive(StructOpt, Debug)]
#[structopt(about, setting(AppSettings::DeriveDisplayOrder))]
struct Opt {
    #[structopt(long, help("Activate all available features"))]
    all_features: bool,
    #[structopt(long, help("Do not activate the `default` feature"))]
    no_default_features: bool,
    #[structopt(long, help("Require Cargo.lock and cache are up to date"))]
    frozen: bool,
    #[structopt(long, help("Require Cargo.lock is up to date"))]
    locked: bool,
    #[structopt(long, help("Run without accessing the network"))]
    offline: bool,
    #[structopt(
        short,
        long,
        value_name("SPEC"),
        number_of_values(1),
        parse(try_from_str = PackageIdSpec::parse),
        help("**Dependency** to run test for")
    )]
    package: Vec<PackageIdSpec>,
    #[structopt(
        long,
        value_name("FEATURES"),
        min_values(1),
        help("Space-separated list of features to activate")
    )]
    features: Vec<String>,
    #[structopt(long, value_name("WHEN"), help("Coloring: auto, always, never"))]
    color: Option<String>,
    #[structopt(
        short,
        long,
        value_name("N"),
        help("How deep in the dependency chain to search")
    )]
    depth: Option<NonZeroUsize>,
    #[structopt(long, value_name("N"), help("Skips the first N packages"))]
    skip: Option<NonZeroUsize>,
    #[structopt(
        default_value_os({
            static DEFAULT: Lazy<PathBuf> =
                Lazy::new(|| env::temp_dir().join("atcoder-rust-base-dep-tests"));
            DEFAULT.as_ref()
        }),
        help("Directory to run tests")
    )]
    dir: PathBuf,
}

impl Opt {
    fn run(&self, config: &mut cargo::Config) -> CliResult {
        config.configure(
            0,
            None,
            &self.color,
            self.frozen,
            self.locked,
            self.offline,
            &None,
            &[],
        )?;

        let DepTestsConfig { exclude } = DepTestsConfig::load(config)?;

        let ws = Workspace::new(&config.cwd().join(MANIFEST_PATH), config)?;

        let (packages, resolve) = cargo::ops::resolve_ws_with_opts(
            &ws,
            ResolveOpts::new(
                false,
                &self.features,
                self.all_features,
                self.no_default_features,
            ),
            &Packages::Default.to_package_id_specs(&ws)?,
        )?;

        let (normal_deps, packages) = (
            find_normal_deps(&ws, &resolve, self.depth)?,
            filter_packages(&packages, &self.package, &exclude)?,
        );

        let wss = setup_workspaces(config, &self.dir, &normal_deps, &packages)?;
        for (i, (id, ws)) in wss.iter().enumerate().skip(self.skip()) {
            let mut msg = format!("Testing `{}` ({}/{})", id, i + 1, wss.len());
            if let Some(skip) = self.skip {
                write!(msg, " (skipping the first {} package(s))", skip).unwrap();
            }
            config.shell().info(msg)?;
            run_tests(&resolve, *id, ws)?;
        }

        config.shell().info("Successful!").map_err(Into::into)
    }

    fn skip(&self) -> usize {
        self.skip.map(NonZeroUsize::get).unwrap_or_default()
    }
}

fn find_normal_deps(
    ws: &Workspace,
    resolve: &Resolve,
    depth: Option<NonZeroUsize>,
) -> Fallible<BTreeSet<PackageId>> {
    let rustc = ws.config().load_global_rustc(Some(&ws))?;
    let host_triple = &rustc.host;
    let target_info = TargetInfo::new(
        ws.config(),
        &Some(host_triple.clone()),
        &rustc,
        compiler::Kind::Host,
    )?;

    let member = ws.current()?.package_id();
    let mut normal_deps = btreeset!(member);
    let mut cur = btreeset!(member);
    let mut depth = depth.map(NonZeroUsize::get);
    while !cur.is_empty() && depth.map_or(true, |d| d > 0) {
        let mut next = btreeset!();
        for from in cur {
            for (to, deps) in resolve.deps(from) {
                for dep in deps {
                    if dep.kind() == dependency::Kind::Normal // `dep` may be a build-dependency.
                        && dep
                            .platform()
                            .as_ref()
                            .map_or(true, |p| p.matches(host_triple, target_info.cfg()))
                        && normal_deps.insert(to)
                    {
                        next.insert(to);
                    }
                }
            }
        }
        cur = next;
        depth = depth.map(|d| d - 1);
    }
    for member in ws.members() {
        normal_deps.remove(&member.package_id());
    }
    Ok(normal_deps)
}

fn filter_packages<'a>(
    packages: &'a PackageSet,
    include: &[PackageIdSpec],
    exclude: &HashSet<PackageIdSpec>,
) -> Fallible<HashMap<PackageId, &'a Package>> {
    let packages = packages.get_many(packages.package_ids())?;
    Ok(packages
        .into_iter()
        .map(|p| (p.package_id(), p))
        .filter(|&(id, _)| {
            (include.is_empty() || include.iter().any(|s| s.matches(id)))
                && !exclude.iter().any(|s| s.matches(id))
        })
        .collect::<HashMap<_, _>>())
}

fn setup_workspaces<'cfg>(
    config: &'cfg cargo::Config,
    root: &Path,
    normal_deps: &BTreeSet<PackageId>,
    packages: &HashMap<PackageId, &Package>,
) -> Fallible<BTreeMap<PackageId, Workspace<'cfg>>> {
    let wss = normal_deps
        .iter()
        .flat_map(|d| packages.get(d))
        .map(|dep| {
            let src = dep.root();
            let dst = root.join(src.file_name().unwrap_or_default());
            let dst = cargo::util::paths::normalize_path(&if dst.is_relative() {
                config.cwd().join(dst)
            } else {
                dst
            });

            config
                .shell()
                .info(&format!("Copying {} to {}", src.display(), dst.display()))?;

            fs_extra::dir::copy(
                src,
                &dst,
                &fs_extra::dir::CopyOptions {
                    skip_exist: true,
                    copy_inside: true,
                    ..fs_extra::dir::CopyOptions::new()
                },
            )?;

            let ws = Workspace::new(&dst.join("Cargo.toml"), config)?;
            Ok((dep.package_id(), ws))
        })
        .collect::<Fallible<BTreeMap<_, _>>>()?;

    for ws in wss.values() {
        let src = cargo::util::paths::normalize_path(&config.cwd().join("Cargo.lock"));
        let dst = ws.root().join("Cargo.lock");

        config
            .shell()
            .info(&format!("Copying {} to {}", src.display(), dst.display()))?;

        fs_extra::file::copy(
            src,
            dst,
            &fs_extra::file::CopyOptions {
                overwrite: true,
                ..fs_extra::file::CopyOptions::new()
            },
        )?;
    }
    Ok(wss)
}

fn run_tests(resolve: &Resolve, id: PackageId, ws: &Workspace) -> CliResult {
    // `ws.current()?.package_id().source_id()` differs to `id.source_id()`.

    let compile_opts = {
        let features = resolve.features(id);
        let mut args = vec!["".to_owned(), "--no-default-features".to_owned()];
        if !features.is_empty() {
            args.push("--features".to_owned());
            args.push(features.iter().join(" "));
        }
        App::new("")
            .arg_features()
            .get_matches_from_safe(args)?
            .compile_options(ws.config(), CompileMode::Test, Some(ws))?
    };

    let test_opts = TestOptions {
        compile_opts,
        no_run: false,
        no_fail_fast: false,
    };

    match cargo::ops::run_tests(&ws, &test_opts, &[])? {
        None => Ok(()),
        Some(err) => Err(match err.exit.as_ref().and_then(ExitStatus::code) {
            Some(code) => {
                let hint = format_err!("{}", err.hint(&ws, &test_opts.compile_opts));
                CliError::new(err.context(hint).into(), code)
            }
            None => CliError::new(err.into(), 101),
        }),
    }
}

trait ShellExt {
    fn info(&mut self, message: impl Display) -> Fallible<()>;
}

impl ShellExt for Shell {
    fn info(&mut self, message: impl Display) -> Fallible<()> {
        if self.verbosity() == Verbosity::Quiet {
            return Ok(());
        }
        let message = format!(
            "{} {}\n",
            if self.supports_color() {
                "\x1B[1m\x1B[36minfo:\x1B[0m"
            } else {
                "info:"
            },
            message,
        );
        self.print_ansi(message.as_ref())
    }
}

#[derive(Deserialize, Debug)]
struct DepTestsConfig {
    exclude: HashSet<PackageIdSpec>,
}

impl DepTestsConfig {
    fn load(config: &cargo::Config) -> Fallible<Self> {
        let path = cargo::util::paths::normalize_path(&config.cwd().join(CONFIG_PATH));
        let toml = cargo::util::paths::read(&path)?;
        let this = toml::from_str(&toml)?;
        config.shell().info(format!("Loaded {}", path.display()))?;
        Ok(this)
    }
}
