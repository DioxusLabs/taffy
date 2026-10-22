//! Yoga configuration for a run, and the matrix of configurations the harness exercises.

use crate::case::ConfigSpec;
use yoga::{Errata, ExperimentalFeature};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Config {
    pub errata: Errata,
    pub fix_flex_basis_fit_content: bool,
    pub web_flex_basis: bool,
    pub point_scale_factor: f32,
    pub use_web_defaults: bool,
}

/// Yoga's default `YGConfigNew()` errata.
pub fn default_errata() -> Errata {
    Errata::MinSizeUndefinedInsteadOfAuto | Errata::FlexFirstPassUsesRunningTotals
}

const ERRATA_NAMES: [(&str, Errata); 5] = [
    ("stretch-flex-basis", Errata::StretchFlexBasis),
    ("absolute-position-without-insets-excludes-padding", Errata::AbsolutePositionWithoutInsetsExcludesPadding),
    ("absolute-percent-against-inner-size", Errata::AbsolutePercentAgainstInnerSize),
    ("min-size-undefined-instead-of-auto", Errata::MinSizeUndefinedInsteadOfAuto),
    ("flex-first-pass-uses-running-totals", Errata::FlexFirstPassUsesRunningTotals),
];

pub fn parse_errata(s: &str) -> Result<Errata, String> {
    Ok(match s {
        "all" => Errata::All,
        "none" => Errata::None,
        "classic" => Errata::Classic,
        "default" => default_errata(),
        _ => {
            let mut e = Errata::None;
            for part in s.split('|') {
                let (_, bit) =
                    ERRATA_NAMES.iter().find(|(n, _)| *n == part).ok_or_else(|| format!("unknown errata `{part}`"))?;
                e |= *bit;
            }
            e
        }
    })
}

pub fn errata_name(e: Errata) -> String {
    if e == Errata::All {
        return "all".into();
    }
    if e == Errata::Classic {
        return "classic".into();
    }
    if e == Errata::None {
        return "none".into();
    }
    ERRATA_NAMES.iter().filter(|(_, bit)| e & *bit == *bit).map(|(n, _)| *n).collect::<Vec<_>>().join("|")
}

impl Default for Config {
    fn default() -> Self {
        Config {
            errata: default_errata(),
            fix_flex_basis_fit_content: false,
            web_flex_basis: false,
            point_scale_factor: 1.0,
            use_web_defaults: false,
        }
    }
}

impl Config {
    pub fn from_spec(spec: &ConfigSpec) -> Result<Config, String> {
        let mut c = Config::default();
        if let Some(e) = &spec.errata {
            c.errata = parse_errata(e)?;
        }
        for f in &spec.experimental_features {
            match f.as_str() {
                "fix-flex-basis-fit-content" => c.fix_flex_basis_fit_content = true,
                "web-flex-basis" => c.web_flex_basis = true,
                _ => return Err(format!("unknown experimental feature `{f}`")),
            }
        }
        if let Some(s) = spec.point_scale_factor {
            c.point_scale_factor = s;
        }
        c.use_web_defaults = spec.use_web_defaults;
        Ok(c)
    }

    pub fn to_spec(self) -> ConfigSpec {
        let mut features = Vec::new();
        if self.fix_flex_basis_fit_content {
            features.push("fix-flex-basis-fit-content".to_string());
        }
        if self.web_flex_basis {
            features.push("web-flex-basis".to_string());
        }
        ConfigSpec {
            errata: Some(errata_name(self.errata)),
            experimental_features: features,
            point_scale_factor: Some(self.point_scale_factor),
            use_web_defaults: self.use_web_defaults,
        }
    }

    /// Short stable name used as the key of recorded expectations.
    pub fn key(&self) -> String {
        let mut k = format!("errata={};scale={}", errata_name(self.errata), self.point_scale_factor);
        if self.fix_flex_basis_fit_content {
            k.push_str(";fix-flex-basis-fit-content");
        }
        if self.web_flex_basis {
            k.push_str(";web-flex-basis");
        }
        if self.use_web_defaults {
            k.push_str(";web-defaults");
        }
        k
    }

    pub fn apply(&self, config: &mut yoga::Config) {
        config.set_errata(self.errata);
        config.set_experimental_feature_enabled(
            ExperimentalFeature::FixFlexBasisFitContent,
            self.fix_flex_basis_fit_content,
        );
        config.set_experimental_feature_enabled(ExperimentalFeature::WebFlexBasis, self.web_flex_basis);
        config.set_point_scale_factor(self.point_scale_factor);
        config.set_use_web_defaults(self.use_web_defaults);
    }

    /// The configurations the port is verified against, RN's three first.
    pub fn matrix() -> Vec<Config> {
        let mut out = Vec::new();
        let base = Config { point_scale_factor: 0.0, ..Config::default() };
        let rn = [Errata::All, Errata::MinSizeUndefinedInsteadOfAuto, Errata::None];
        for e in rn {
            out.push(Config { errata: e, ..base });
            out.push(Config { errata: e, fix_flex_basis_fit_content: true, ..base });
        }
        out.push(Config { errata: Errata::Classic, ..base });
        out.push(Config { errata: default_errata(), ..base });
        for (_, bit) in ERRATA_NAMES {
            out.push(Config { errata: bit, ..base });
        }
        out.push(Config { errata: Errata::None, web_flex_basis: true, ..base });
        for scale in [1.0, 2.0, 3.0, 2.625] {
            out.push(Config { errata: Errata::All, point_scale_factor: scale, ..base });
        }
        out
    }
}
