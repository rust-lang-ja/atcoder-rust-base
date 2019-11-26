use cargo::core::compiler::{self, CompileMode, TargetInfo};
use cargo::core::dependency::{self, Dependency};
use cargo::core::package::{Package, PackageSet};
use cargo::core::resolver::ResolveOpts;
use cargo::core::shell::{Shell, Verbosity};
use cargo::core::{PackageIdSpec, Resolve, Workspace};
use cargo::ops::{CompileFilter, CompileOptions, FilterRule, LibRule, Packages, TestOptions};
use cargo::util::command_prelude::{App, AppExt as _, AppSettings, ArgMatchesExt as _};
use cargo::{CliError, CliResult};
use either::Either;
use failure::{format_err, Fail as _, Fallible};
use itertools::Itertools as _;
use maplit::btreeset;
use once_cell::sync::Lazy;
use serde::Deserialize;
use structopt::StructOpt;

use std::borrow::Borrow;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::env;
use std::fmt::Display;
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
        help("Package to run test for")
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

        let DepTestsConfig { exclude, filter } = DepTestsConfig::load(config)?;

        let ws = Workspace::new(&config.cwd().join(MANIFEST_PATH), config)?;

        let (packages, resolve) = cargo::ops::resolve_ws_with_opts(
            &ws,
            ResolveOpts::new(
                true,
                &self.features,
                self.all_features,
                self.no_default_features,
            ),
            &Packages::Default.to_package_id_specs(&ws)?,
        )?;

        let dev_pkgs = filter_packages(&ws, &packages, &resolve, None, |_| true)?;
        let (dev_deps_free_pkgs, target_pkgs) =
            filter_packages(&ws, &packages, &resolve, self.depth, |d| {
                d.kind() == dependency::Kind::Normal
            })?
            .iter()
            .copied()
            .filter(|pkg| {
                let (id, include) = (pkg.package_id(), &self.package);
                include.iter().any(|s| s.matches(id))
                    || include.is_empty() && !exclude.iter().any(|s| s.matches(id))
            })
            .partition(|pkg| pkg.dependencies().iter().all(|d| d.is_transitive()));

        let new_ws = setup_workspace(config, &self.dir, &dev_pkgs, &target_pkgs, &resolve)?;

        run_tests(&ws, dev_deps_free_pkgs, &filter)?;
        run_tests(&new_ws, new_ws.members(), &filter)?;
        config.shell().info("Successful!").map_err(Into::into)
    }
}

fn filter_packages<'a>(
    ws: &'a Workspace,
    packages: &'a PackageSet,
    resolve: &Resolve,
    depth: Option<NonZeroUsize>,
    extra_pred: fn(&Dependency) -> bool,
) -> Fallible<BTreeSet<&'a Package>> {
    let rustc = ws.config().load_global_rustc(Some(&ws))?;
    let host_triple = &rustc.host;
    let target_info = TargetInfo::new(
        ws.config(),
        &Some(host_triple.clone()),
        &rustc,
        compiler::Kind::Host,
    )?;

    let mut outcome = btreeset!(ws.current()?);
    let mut cur = outcome.clone();
    let mut depth = depth.map(NonZeroUsize::get);
    while !cur.is_empty() && depth.map_or(true, |d| d > 0) {
        let mut next = btreeset!();
        for from in cur {
            for (to, deps) in resolve.deps(from.package_id()) {
                let to = packages.get_one(to)?;
                for dep in deps {
                    if dep
                        .platform()
                        .as_ref()
                        .map_or(true, |p| p.matches(host_triple, target_info.cfg()))
                        && extra_pred(dep)
                        && outcome.insert(to)
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
        outcome.remove(member);
    }
    Ok(outcome)
}

fn setup_workspace<'cfg>(
    config: &'cfg cargo::Config,
    root: &Path,
    pkgs_to_build: &BTreeSet<&Package>,
    pkgs_to_test: &BTreeSet<&Package>,
    resolve: &Resolve,
) -> Fallible<Workspace<'cfg>> {
    let deps = pkgs_to_build
        .iter()
        .map(|dep| {
            let path_or_version = if pkgs_to_test.contains(dep) {
                let src = dep.root();
                let dst = root.join(src.file_name().unwrap_or_default());
                let dst = cargo::util::paths::normalize_path(&if dst.is_relative() {
                    config.cwd().join(dst)
                } else {
                    dst
                });

                config
                    .shell()
                    .info(format!("Copying {} to {}", src.display(), dst.display()))?;

                fs_extra::dir::copy(
                    src,
                    &dst,
                    &fs_extra::dir::CopyOptions {
                        skip_exist: true,
                        copy_inside: true,
                        ..fs_extra::dir::CopyOptions::new()
                    },
                )?;

                Either::Left(format!(
                    "./{}",
                    dst.file_name()
                        .unwrap_or_default()
                        .to_str()
                        .expect("the directory names should be <name>-<version>")
                ))
            } else {
                Either::Right(dep.package_id().version())
            };
            let features = resolve.features_sorted(dep.package_id());
            Ok((dep.package_id(), (path_or_version, features)))
        })
        .collect::<Fallible<BTreeMap<_, _>>>()?;

    for (path_or_version, _) in deps.values() {
        if let Either::Left(path) = path_or_version {
            let manifest_path = root.join(path).join("Cargo.toml");
            let manifest_path = cargo::util::paths::normalize_path(&manifest_path);
            let mut cargo_toml =
                cargo::util::paths::read(&manifest_path)?.parse::<toml_edit::Document>()?;
            cargo_toml.as_table_mut().remove("profile");
            cargo::util::paths::write(&manifest_path, cargo_toml.to_string().as_ref())?;
            config
                .shell()
                .info(format!("Modified {}", manifest_path.display()))?;
        }
    }

    let src = cargo::util::paths::normalize_path(&config.cwd().join("Cargo.lock"));
    let dst = root.join("Cargo.lock");

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

    let mut cargo_toml = r#"[package]
name = "atcoder-rust-base-dep-tests"
version = "0.0.0"
edition = "2018"
publish = false

[workspace]
members = []

[patch.crates-io]
"#
    .parse::<toml_edit::Document>()
    .unwrap();

    cargo_toml["workspace"]["members"] = {
        let mut workspace = toml_edit::Array::default();
        for (path_or_version, _) in deps.values() {
            if let Either::Left(path) = path_or_version {
                workspace.push(&**path);
            }
        }
        toml_edit::value(workspace)
    };

    for (id, (path_or_version, _)) in &deps {
        if let Either::Left(path) = path_or_version {
            cargo_toml["patch"]["crates-io"][&*id.name()]["path"] = toml_edit::value(&**path);
        }
    }

    cargo_toml["dependencies"] = toml_edit::table();
    for (i, (id, (path_or_version, features))) in deps.iter().enumerate() {
        let dummy_extern_crate_name = format!("_{}", i);
        let mut val = toml_edit::InlineTable::default();
        val.get_or_insert("package", &*id.name());
        match path_or_version {
            Either::Left(path) => val.get_or_insert("path", &**path),
            Either::Right(version) => val.get_or_insert("version", format!("={}", version)),
        };
        val.get_or_insert("default-features", false);
        let mut val_features = toml_edit::Array::default();
        for &feature in features {
            val_features.push(feature);
        }
        val.get_or_insert("features", val_features);
        cargo_toml["dependencies"][&dummy_extern_crate_name] = toml_edit::value(val);
    }

    let manifest_path = root.join("Cargo.toml");
    cargo::util::paths::write(&manifest_path, cargo_toml.to_string().as_ref())?;
    config
        .shell()
        .info(format!("Wrote {}", manifest_path.display()))?;

    let src_dir = root.join("src");
    cargo::util::paths::create_dir_all(&src_dir)?;
    let src_path = src_dir.join("lib.rs");
    cargo::util::paths::write(&src_path, b"")?;
    config
        .shell()
        .info(format!("Wrote {}", src_path.display()))?;

    Workspace::new(&manifest_path, config)
}

fn run_tests<I: IntoIterator<Item = P>, P: Borrow<Package>>(
    ws: &Workspace,
    pkgs: I,
    filter: &HashMap<PackageIdSpec, DepTestsConfigFilter>,
) -> CliResult {
    fn run_tests(
        ws: &Workspace,
        pkg: &Package,
        modify_compile_opts: impl FnOnce(&mut CompileOptions),
    ) -> CliResult {
        let spec = PackageIdSpec::from_package_id(pkg.package_id()).to_string();
        let mut compile_opts = App::new("")
            .arg_package("")
            .get_matches_from_safe(&["", "-p", &spec])?
            .compile_options(ws.config(), CompileMode::Test, Some(ws))?;
        modify_compile_opts(&mut compile_opts);

        let test_opts = TestOptions {
            compile_opts,
            no_run: false,
            no_fail_fast: false,
        };

        if let Some(err) = cargo::ops::run_tests(&ws, &test_opts, &[])? {
            return Err(match err.exit.as_ref().and_then(ExitStatus::code) {
                Some(code) => {
                    let hint = format_err!("{}", err.hint(&ws, &test_opts.compile_opts));
                    CliError::new(err.context(hint).into(), code)
                }
                None => CliError::new(err.into(), 101),
            });
        }
        Ok(())
    }

    fn default_compile_filter() -> CompileFilter {
        CompileFilter::new(
            LibRule::True,
            FilterRule::none(),
            FilterRule::none(),
            FilterRule::none(),
            FilterRule::none(),
        )
    }

    for pkg in pkgs {
        let pkg = pkg.borrow();
        let filter = match filter
            .iter()
            .filter(|(k, _)| k.matches(pkg.package_id()))
            .map(|(_, v)| v.clone())
            .exactly_one()
        {
            Ok(filter) => Some(filter),
            Err(err) => match err.count() {
                0 => None,
                n => return Err(format_err!("`{}` matches {} specs", pkg, n).into()),
            },
        };

        run_tests(ws, pkg, |mut compile_opts| {
            compile_opts.filter = filter
                .as_ref()
                .map(DepTestsConfigFilter::compile_filter)
                .unwrap_or_else(default_compile_filter);
        })?;

        if filter.map_or(true, |DepTestsConfigFilter { doc, .. }| doc) {
            run_tests(ws, pkg, |mut compile_opts| {
                compile_opts.build_config.mode = CompileMode::Doctest;
                compile_opts.filter = default_compile_filter();
            })?;
        }
    }
    Ok(())
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
    filter: HashMap<PackageIdSpec, DepTestsConfigFilter>,
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

#[derive(Deserialize, Default, Debug, Clone, PartialEq)]
struct DepTestsConfigFilter {
    #[serde(default)]
    doc: bool,
    #[serde(default)]
    lib: bool,
    #[serde(default)]
    bin: BTreeSet<String>,
    #[serde(default)]
    example: BTreeSet<String>,
    #[serde(default)]
    test: BTreeSet<String>,
    #[serde(default)]
    bench: BTreeSet<String>,
}

impl DepTestsConfigFilter {
    fn compile_filter(&self) -> CompileFilter {
        CompileFilter::new(
            if self.lib {
                LibRule::True
            } else {
                LibRule::False
            },
            FilterRule::new(self.bin.iter().cloned().collect(), false),
            FilterRule::new(self.test.iter().cloned().collect(), false),
            FilterRule::new(self.example.iter().cloned().collect(), false),
            FilterRule::new(self.bench.iter().cloned().collect(), false),
        )
    }
}
