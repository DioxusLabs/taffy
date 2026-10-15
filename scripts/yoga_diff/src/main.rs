//! Differential harness: lays out the same trees with C++ Yoga (through the `yoga` crate) and Taffy,
//! and compares layouts and measure/baseline call logs bit-for-bit.
//!
//! ```text
//! yoga_diff gen <out_dir> <count> [--rtl] [--quirks] [--no-flex] [--seed-offset N]
//! yoga_diff run <case files/dirs...> [--errata E] [--scale S] [--fit-content] [--web-flex-basis] [--matrix] [--show N] [--tol F]
//! yoga_diff record <case files/dirs...> --out <dir> [--matrix | config flags]
//! yoga_diff check <recorded files/dirs...> [--show N]
//! ```

mod case;
mod config;
mod gen;
mod output;
mod taffy_side;
mod text;
mod yoga_side;

use case::{Case, Step};
use config::Config;
use output::{differing_nodes, same_calls, Run};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use taffy_side::{TaffyMode, TaffySide};
use yoga_side::YogaTree;

#[derive(Serialize, Deserialize)]
struct Expected {
    config: case::ConfigSpec,
    runs: Vec<Run>,
}

#[derive(Serialize, Deserialize)]
struct Recorded {
    case: Case,
    /// Keyed by `Config::key()`.
    expected: BTreeMap<String, Expected>,
}

/// Runs the case (initial layout plus every step) through Yoga.
fn run_yoga(case: &Case, config: Config) -> Vec<Run> {
    let mut case = case.clone();
    let mut yoga = YogaTree::new(&case, config);
    let mut inputs = case.layout_inputs.clone();
    let mut runs = vec![yoga.calculate_layout(&inputs)];
    let steps = std::mem::take(&mut case.steps);
    for step in &steps {
        apply_to_case(&mut case, step, &mut inputs);
        yoga.apply_step(step, &case.tree);
        runs.push(yoga.calculate_layout(&inputs));
    }
    runs
}

fn run_taffy(case: &Case, config: Config, mode: TaffyMode) -> Vec<Run> {
    let mut case = case.clone();
    let mut taffy = TaffySide::new(&case, config, mode);
    let mut inputs = case.layout_inputs.clone();
    let mut runs = vec![taffy.calculate_layout(&case.tree, &inputs)];
    let steps = std::mem::take(&mut case.steps);
    for step in &steps {
        apply_to_case(&mut case, step, &mut inputs);
        taffy.apply_step(step, &case.tree);
        runs.push(taffy.calculate_layout(&case.tree, &inputs));
    }
    runs
}

fn apply_to_case(case: &mut Case, step: &Step, inputs: &mut case::LayoutInputs) {
    match step {
        Step::SetStyle { path, style } => {
            let node = case.tree.get_mut(path);
            for (k, v) in style {
                node.style.insert(k.clone(), v.clone());
            }
        }
        Step::MarkDirty { .. } => {}
        Step::LayoutInputs(new_inputs) => *inputs = new_inputs.clone(),
    }
}

#[derive(Default)]
struct Stats {
    trees: usize,
    trees_differing: usize,
    nodes: usize,
    nodes_differing: usize,
    /// Differing nodes whose parent matches: where a difference originates.
    root_cause_nodes: usize,
    runs_with_call_diffs: usize,
    first: Vec<(usize, String, usize)>,
    /// 0 = bit-for-bit.
    tolerance: f32,
}

impl Stats {
    fn compare(&mut self, name: &str, case: &Case, expected: &[Run], actual: &[Run]) {
        let parents: Vec<Option<usize>> = case.tree.preorder().into_iter().map(|(_, p)| p).collect();
        let mut tree_bad = false;
        for (run, (e, a)) in expected.iter().zip(actual).enumerate() {
            self.nodes += e.layout.len();
            let bad = differing_nodes(e, a, self.tolerance);
            if !same_calls(e, a) {
                self.runs_with_call_diffs += 1;
            }
            if bad.is_empty() {
                continue;
            }
            tree_bad = true;
            self.nodes_differing += bad.len();
            let root_causes: Vec<usize> = bad
                .iter()
                .copied()
                .filter(|&i| parents.get(i).copied().flatten().is_none_or(|p| !bad.contains(&p)))
                .collect();
            self.root_cause_nodes += root_causes.len();
            if run == 0 || self.first.iter().all(|(_, n, _)| n != name) {
                self.first.push((e.layout.len(), name.to_string(), root_causes.first().copied().unwrap_or(bad[0])));
            }
        }
        self.trees += 1;
        self.trees_differing += tree_bad as usize;
    }

    fn report(&self, show: usize) {
        let pct = |a: usize, b: usize| if b == 0 { 0.0 } else { 100.0 * a as f64 / b as f64 };
        println!(
            "trees differing {}/{} ({:.1}%), nodes differing {}/{} ({:.2}%), root-cause nodes {}, runs with call-log differences {}",
            self.trees_differing,
            self.trees,
            pct(self.trees_differing, self.trees),
            self.nodes_differing,
            self.nodes,
            pct(self.nodes_differing, self.nodes),
            self.root_cause_nodes,
            self.runs_with_call_diffs
        );
        let mut first = self.first.clone();
        first.sort();
        for (nodes, name, node) in first.iter().take(show) {
            println!("  {name} nodes={nodes} node={node}");
        }
    }
}

fn collect_files(args: &[String]) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for a in args {
        let p = Path::new(a);
        if p.is_dir() {
            let mut entries: Vec<PathBuf> = std::fs::read_dir(p)
                .unwrap()
                .map(|e| e.unwrap().path())
                .filter(|p| p.extension().is_some_and(|e| e == "json"))
                .collect();
            entries.sort();
            files.extend(entries);
        } else {
            files.push(p.to_path_buf());
        }
    }
    files
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> T {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

struct Flags {
    positional: Vec<String>,
    show: usize,
    matrix: bool,
    config: Option<Config>,
    out: Option<PathBuf>,
    profile: gen::Profile,
    seed_offset: u64,
    tolerance: f32,
}

fn parse_flags(args: &[String]) -> Flags {
    let mut f = Flags {
        positional: Vec::new(),
        show: 10,
        matrix: false,
        config: None,
        out: None,
        profile: gen::Profile::default(),
        seed_offset: 0,
        tolerance: 0.0,
    };
    let mut config: Option<Config> = None;
    fn cfg(config: &mut Option<Config>) -> &mut Config {
        config.get_or_insert(Config { point_scale_factor: 0.0, errata: yoga::Errata::All, ..Config::default() })
    }
    let mut i = 0;
    while i < args.len() {
        let next = |i: &mut usize| -> String {
            *i += 1;
            args.get(*i).unwrap_or_else(|| panic!("`{}` needs a value", args[*i - 1])).clone()
        };
        match args[i].as_str() {
            "--show" => f.show = next(&mut i).parse().unwrap(),
            "--matrix" => f.matrix = true,
            "--out" => f.out = Some(next(&mut i).into()),
            "--errata" => cfg(&mut config).errata = config::parse_errata(&next(&mut i)).unwrap(),
            "--scale" => cfg(&mut config).point_scale_factor = next(&mut i).parse().unwrap(),
            "--fit-content" => cfg(&mut config).fix_flex_basis_fit_content = true,
            "--web-flex-basis" => cfg(&mut config).web_flex_basis = true,
            "--web-defaults" => cfg(&mut config).use_web_defaults = true,
            "--rtl" => f.profile.rtl = true,
            "--quirks" => f.profile.quirks = true,
            "--no-flex" => f.profile.no_flex = true,
            "--seed-offset" => f.seed_offset = next(&mut i).parse().unwrap(),
            "--tol" => f.tolerance = next(&mut i).parse().unwrap(),
            a if a.starts_with("--") => panic!("unknown flag `{a}`"),
            a => f.positional.push(a.to_string()),
        }
        i += 1;
    }
    f.config = config;
    f
}

fn configs(flags: &Flags, case: &Case) -> Vec<Config> {
    if flags.matrix {
        Config::matrix()
    } else if let Some(c) = flags.config {
        vec![c]
    } else if !case.config.is_default() {
        vec![Config::from_spec(&case.config).unwrap()]
    } else {
        vec![Config { point_scale_factor: 0.0, errata: yoga::Errata::All, ..Config::default() }]
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some((command, rest)) = args.split_first() else {
        eprintln!("usage: yoga_diff <gen|run|record|check> ...");
        std::process::exit(2);
    };
    let flags = parse_flags(rest);
    match command.as_str() {
        "gen" => {
            let out = Path::new(&flags.positional[0]);
            let count: u64 = flags.positional[1].parse().unwrap();
            std::fs::create_dir_all(out).unwrap();
            for i in 0..count {
                let seed = flags.seed_offset + i;
                let case = gen::case(seed, flags.profile);
                std::fs::write(out.join(format!("r{seed:05}.json")), serde_json::to_string(&case).unwrap()).unwrap();
            }
        }
        "run" => {
            let files = collect_files(&flags.positional);
            let mut by_config: BTreeMap<String, Stats> = BTreeMap::new();
            for file in &files {
                let case: Case = read_json(file);
                let name = file.file_stem().unwrap().to_string_lossy().to_string();
                for config in configs(&flags, &case) {
                    let yoga = run_yoga(&case, config);
                    let taffy = run_taffy(&case, config, TaffyMode::Flex);
                    by_config
                        .entry(config.key())
                        .or_insert_with(|| Stats { tolerance: flags.tolerance, ..Default::default() })
                        .compare(&name, &case, &yoga, &taffy);
                }
            }
            for (key, stats) in &by_config {
                print!("{key}: ");
                stats.report(flags.show);
            }
        }
        "record" => {
            let out = flags.out.clone().expect("record needs --out <dir>");
            std::fs::create_dir_all(&out).unwrap();
            for file in collect_files(&flags.positional) {
                let case: Case = read_json(&file);
                let mut expected = BTreeMap::new();
                for config in configs(&flags, &case) {
                    expected.insert(config.key(), Expected { config: config.to_spec(), runs: run_yoga(&case, config) });
                }
                let recorded = Recorded { case, expected };
                std::fs::write(out.join(file.file_name().unwrap()), serde_json::to_string_pretty(&recorded).unwrap())
                    .unwrap();
            }
        }
        "check" => {
            let mut stats = Stats { tolerance: flags.tolerance, ..Default::default() };
            for file in collect_files(&flags.positional) {
                let recorded: Recorded = read_json(&file);
                let name = file.file_stem().unwrap().to_string_lossy().to_string();
                for (key, expected) in &recorded.expected {
                    let config = Config::from_spec(&expected.config).unwrap();
                    let taffy = run_taffy(&recorded.case, config, TaffyMode::Flex);
                    stats.compare(&format!("{name}[{key}]"), &recorded.case, &expected.runs, &taffy);
                }
            }
            stats.report(flags.show);
            if stats.trees_differing > 0 {
                std::process::exit(1);
            }
        }
        other => panic!("unknown command `{other}`"),
    }
}
