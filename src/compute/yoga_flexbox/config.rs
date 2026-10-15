//! Port of `yoga/config/Config.h` and `yoga/config/Config.cpp`.

use super::enums::{YogaErrata, YogaExperimentalFeature};

/// The set of enabled [`YogaExperimentalFeature`]s.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct YogaExperimentalFeatures(u8);

impl YogaExperimentalFeatures {
    /// No experimental features enabled.
    pub const NONE: YogaExperimentalFeatures = YogaExperimentalFeatures(0);

    /// The bit representing `feature`.
    const fn bit(feature: YogaExperimentalFeature) -> u8 {
        1 << (feature as u8)
    }

    /// Whether `feature` is enabled.
    pub const fn contains(self, feature: YogaExperimentalFeature) -> bool {
        self.0 & Self::bit(feature) != 0
    }

    /// Enables or disables `feature`.
    pub fn set(&mut self, feature: YogaExperimentalFeature, enabled: bool) {
        if enabled {
            self.0 |= Self::bit(feature);
        } else {
            self.0 &= !Self::bit(feature);
        }
    }

    /// Returns the set with `feature` enabled.
    pub fn with(mut self, feature: YogaExperimentalFeature) -> Self {
        self.set(feature, true);
        self
    }
}

/// Settings that apply to a whole Yoga tree (Yoga's `YGConfig`), passed to the Yoga layout functions.
///
/// Yoga invalidates every cached layout when any of these change; a host must do the same by marking
/// the tree dirty (see [`update_invalidates_layout`](Self::update_invalidates_layout)).
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct YogaConfig {
    /// Use CSS flexbox defaults (`flex-direction: row`, `align-content: stretch`, `flex-shrink: 1`,
    /// `position: static`) instead of Yoga's.
    pub use_web_defaults: bool,
    /// Opt-in behaviour changes.
    pub experimental_features: YogaExperimentalFeatures,
    /// Deliberate deviations from the CSS specification.
    pub errata: YogaErrata,
    /// Pixel density the layout is rounded to; `0.0` disables rounding.
    pub point_scale_factor: f32,
}

impl Default for YogaConfig {
    /// Yoga's `YGConfigNew()` defaults.
    fn default() -> Self {
        YogaConfig {
            use_web_defaults: false,
            experimental_features: YogaExperimentalFeatures::NONE,
            errata: YogaErrata::MIN_SIZE_UNDEFINED_INSTEAD_OF_AUTO | YogaErrata::FLEX_FIRST_PASS_USES_RUNNING_TOTALS,
            point_scale_factor: 1.0,
        }
    }
}

impl YogaConfig {
    /// True if any flag of `errata` is enabled.
    pub const fn has_errata(&self, errata: YogaErrata) -> bool {
        self.errata.contains(errata)
    }

    /// Whether `feature` is enabled.
    pub const fn is_experimental_feature_enabled(&self, feature: YogaExperimentalFeature) -> bool {
        self.experimental_features.contains(feature)
    }

    /// Whether layouts computed with `self` are invalid under `new` (Yoga's `configUpdateInvalidatesLayout`).
    pub fn update_invalidates_layout(&self, new: &YogaConfig) -> bool {
        self.errata != new.errata
            || self.experimental_features != new.experimental_features
            || self.point_scale_factor != new.point_scale_factor
            || self.use_web_defaults != new.use_web_defaults
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_yoga() {
        let config = YogaConfig::default();
        assert!(config.has_errata(YogaErrata::MIN_SIZE_UNDEFINED_INSTEAD_OF_AUTO));
        assert!(config.has_errata(YogaErrata::FLEX_FIRST_PASS_USES_RUNNING_TOTALS));
        assert!(!config.has_errata(YogaErrata::STRETCH_FLEX_BASIS));
        assert_eq!(config.point_scale_factor, 1.0);
        assert!(!config.use_web_defaults);
        assert!(!config.is_experimental_feature_enabled(YogaExperimentalFeature::FixFlexBasisFitContent));
    }

    #[test]
    fn experimental_features() {
        let mut features = YogaExperimentalFeatures::NONE.with(YogaExperimentalFeature::FixFlexBasisFitContent);
        assert!(features.contains(YogaExperimentalFeature::FixFlexBasisFitContent));
        assert!(!features.contains(YogaExperimentalFeature::WebFlexBasis));
        features.set(YogaExperimentalFeature::WebFlexBasis, true);
        features.set(YogaExperimentalFeature::FixFlexBasisFitContent, false);
        assert_eq!(features, YogaExperimentalFeatures::NONE.with(YogaExperimentalFeature::WebFlexBasis));
    }

    #[test]
    fn update_invalidates_layout() {
        let config = YogaConfig::default();
        assert!(!config.update_invalidates_layout(&config));
        assert!(config.update_invalidates_layout(&YogaConfig { point_scale_factor: 2.0, ..config }));
        assert!(config.update_invalidates_layout(&YogaConfig { errata: YogaErrata::ALL, ..config }));
        assert!(config.update_invalidates_layout(&YogaConfig { use_web_defaults: true, ..config }));
        assert!(config.update_invalidates_layout(&YogaConfig {
            experimental_features: YogaExperimentalFeatures::NONE.with(YogaExperimentalFeature::WebFlexBasis),
            ..config
        }));
    }
}
