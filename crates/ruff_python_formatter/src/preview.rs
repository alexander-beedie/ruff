//! Helpers to test if a specific preview style is enabled or not.
//!
//! The motivation for these functions isn't to avoid code duplication but to ease promoting preview styles
//! to stable. The challenge with directly using [`is_preview`](PyFormatContext::is_preview) is that it is unclear
//! for which specific feature this preview check is for. Having named functions simplifies the promotion:
//! Simply delete the function and let Rust tell you which checks you have to remove.

use ruff_macros::CacheKey;
use serde::{Deserialize, Serialize};

use crate::PyFormatContext;

/// Identifies a specific formatter preview feature that can be individually enabled or disabled.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    CacheKey,
    strum_macros::Display,
    strum_macros::EnumString,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum FormatterPreviewFeature {
    HugParensWithBracesAndSquareBrackets,
    FluentLayoutSplitFirstCall,
}

/// Returns `true` if the [`hug_parens_with_braces_and_square_brackets`](https://github.com/astral-sh/ruff/issues/8279) preview style is enabled.
pub(crate) fn is_hug_parens_with_braces_and_square_brackets_enabled(
    context: &PyFormatContext,
) -> bool {
    context
        .is_preview_feature_enabled(FormatterPreviewFeature::HugParensWithBracesAndSquareBrackets)
}

/// Returns `true` if the
/// [`fluent_layout_split_first_call`](https://github.com/astral-sh/ruff/pull/21369) preview
/// style is enabled.
pub(crate) fn is_fluent_layout_split_first_call_enabled(context: &PyFormatContext) -> bool {
    context.is_preview_feature_enabled(FormatterPreviewFeature::FluentLayoutSplitFirstCall)
}
