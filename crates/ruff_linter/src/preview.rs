//! Helpers to test if a specific preview style is enabled or not.
//!
//! The motivation for these functions isn't to avoid code duplication but to ease promoting preview behavior
//! to stable. The challenge with directly checking the `preview` attribute of [`LinterSettings`] is that it is unclear
//! which specific feature this preview check is for. Having named functions simplifies the promotion:
//! Simply delete the function and let Rust tell you which checks you have to remove.

use ruff_macros::CacheKey;
use serde::{Deserialize, Serialize};

use crate::settings::LinterSettings;
use crate::settings::types::LintPreviewConfig;

/// Identifies a specific linter preview feature that can be individually enabled or disabled.
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
pub enum LintPreviewFeature {
    AllowNestedRoots,
    B006CheckGuaranteedMutableExpr,
    B006UnsafeFixPreserveAssignmentExpr,
    BadVersionInfoInNonStub,
    BaseloaderSafeInYamlLoad,
    Ble001ExcInfoSuppression,
    CheckComprehensionsInTupleCall,
    ComprehensionWithMinMaxSum,
    CustomExceptionChecking,
    DunderInitFixUnusedImport,
    EnumerateForLoopIntIndex,
    ExpandedImportConventions,
    ExpandedPathlibSubclassCheck,
    ExtendedI18nFunctionMatching,
    ExtendedSnmpApiPathDetection,
    FileLevelInvalidRuleCode,
    FixBuiltinOpen,
    FixFStringLogging,
    FixManualDictComprehension,
    FixManualListComprehension,
    FixOsChmod,
    FixOsGetcwd,
    FixOsMakedirs,
    FixOsMkdir,
    FixOsPathAbspath,
    FixOsPathBasename,
    FixOsPathDirname,
    FixOsPathExists,
    FixOsPathExpanduser,
    FixOsPathGetatime,
    FixOsPathGetctime,
    FixOsPathGetmtime,
    FixOsPathGetsize,
    FixOsPathIsabs,
    FixOsPathIsdir,
    FixOsPathIsfile,
    FixOsPathIslink,
    FixOsPathSamefile,
    FixOsReadlink,
    FixOsRemove,
    FixOsRename,
    FixOsReplace,
    FixOsRmdir,
    FixOsSymlink,
    FixOsUnlink,
    FutureRequiredPreviewGenerics,
    IncludeMarkdownFiles,
    IncludePywFiles,
    IncorrectDictIteratorComprehension,
    MutableDefaultInDataclassField,
    PluralNgettextCheck,
    Py315Support,
    RefinedSubmoduleImportMatch,
    ResolveStringAnnotationPyi041,
    S310ResolveStringLiteralBindings,
    StandaloneMockNonExistent,
    SuspiciousFunctionReference,
    TypeVarDefault,
    TypingExtensionsStrAlias,
    UndefinedExportInDunderInit,
    UnicodeToUnicodeConfusables,
    Up024PreciseHighlighting,
}

// Rule-specific behavior

// https://github.com/astral-sh/ruff/pull/21382
pub(crate) fn is_custom_exception_checking_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::CustomExceptionChecking)
}

// https://github.com/astral-sh/ruff/pull/15541
pub(crate) fn is_suspicious_function_reference_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::SuspiciousFunctionReference)
}

// https://github.com/astral-sh/ruff/pull/10759
pub(crate) fn is_comprehension_with_min_max_sum_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::ComprehensionWithMinMaxSum)
}

// https://github.com/astral-sh/ruff/pull/12657
pub(crate) fn is_check_comprehensions_in_tuple_call_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::CheckComprehensionsInTupleCall)
}

// https://github.com/astral-sh/ruff/issues/15347
pub(crate) fn is_bad_version_info_in_non_stub_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::BadVersionInfoInNonStub)
}

/// <https://github.com/astral-sh/ruff/pull/19303>
pub(crate) fn is_fix_f_string_logging_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixFStringLogging)
}

// https://github.com/astral-sh/ruff/pull/16719
pub(crate) fn is_fix_manual_dict_comprehension_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixManualDictComprehension)
}

// https://github.com/astral-sh/ruff/pull/13919
pub(crate) fn is_fix_manual_list_comprehension_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixManualListComprehension)
}

// https://github.com/astral-sh/ruff/pull/18763
pub(crate) fn is_fix_os_path_getsize_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixOsPathGetsize)
}
// https://github.com/astral-sh/ruff/pull/18922
pub(crate) fn is_fix_os_path_getmtime_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixOsPathGetmtime)
}

// https://github.com/astral-sh/ruff/pull/18922
pub(crate) fn is_fix_os_path_getatime_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixOsPathGetatime)
}

// https://github.com/astral-sh/ruff/pull/18922
pub(crate) fn is_fix_os_path_getctime_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixOsPathGetctime)
}

// https://github.com/astral-sh/ruff/pull/19213
pub(crate) fn is_fix_os_path_abspath_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixOsPathAbspath)
}

// https://github.com/astral-sh/ruff/pull/19213
pub(crate) fn is_fix_os_rmdir_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixOsRmdir)
}

// https://github.com/astral-sh/ruff/pull/19213
pub(crate) fn is_fix_os_unlink_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixOsUnlink)
}

// https://github.com/astral-sh/ruff/pull/19213
pub(crate) fn is_fix_os_remove_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixOsRemove)
}

// https://github.com/astral-sh/ruff/pull/19213
pub(crate) fn is_fix_os_path_exists_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixOsPathExists)
}

// https://github.com/astral-sh/ruff/pull/19213
pub(crate) fn is_fix_os_path_expanduser_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixOsPathExpanduser)
}

// https://github.com/astral-sh/ruff/pull/19213
pub(crate) fn is_fix_os_path_isdir_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixOsPathIsdir)
}

// https://github.com/astral-sh/ruff/pull/19213
pub(crate) fn is_fix_os_path_isfile_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixOsPathIsfile)
}

// https://github.com/astral-sh/ruff/pull/19213
pub(crate) fn is_fix_os_path_islink_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixOsPathIslink)
}

// https://github.com/astral-sh/ruff/pull/19213
pub(crate) fn is_fix_os_path_isabs_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixOsPathIsabs)
}

// https://github.com/astral-sh/ruff/pull/19213
pub(crate) fn is_fix_os_readlink_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixOsReadlink)
}

// https://github.com/astral-sh/ruff/pull/19213
pub(crate) fn is_fix_os_path_basename_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixOsPathBasename)
}

// https://github.com/astral-sh/ruff/pull/19213
pub(crate) fn is_fix_os_path_dirname_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixOsPathDirname)
}

// https://github.com/astral-sh/ruff/pull/19404
pub(crate) fn is_fix_os_chmod_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixOsChmod)
}

// https://github.com/astral-sh/ruff/pull/19404
pub(crate) fn is_fix_os_rename_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixOsRename)
}

// https://github.com/astral-sh/ruff/pull/19404
pub(crate) fn is_fix_os_replace_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixOsReplace)
}

// https://github.com/astral-sh/ruff/pull/19404
pub(crate) fn is_fix_os_path_samefile_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixOsPathSamefile)
}

// https://github.com/astral-sh/ruff/pull/19245
pub(crate) fn is_fix_os_getcwd_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixOsGetcwd)
}

// https://github.com/astral-sh/ruff/pull/19514
pub(crate) fn is_fix_os_mkdir_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixOsMkdir)
}

// https://github.com/astral-sh/ruff/pull/19514
pub(crate) fn is_fix_os_makedirs_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixOsMakedirs)
}

// https://github.com/astral-sh/ruff/pull/20009
pub(crate) fn is_fix_os_symlink_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixOsSymlink)
}

// https://github.com/astral-sh/ruff/pull/11436
// https://github.com/astral-sh/ruff/pull/11168
pub(crate) fn is_dunder_init_fix_unused_import_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::DunderInitFixUnusedImport)
}

// https://github.com/astral-sh/ruff/pull/8473
pub(crate) fn is_unicode_to_unicode_confusables_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::UnicodeToUnicodeConfusables)
}

// https://github.com/astral-sh/ruff/pull/11370
pub(crate) fn is_undefined_export_in_dunder_init_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::UndefinedExportInDunderInit)
}

// https://github.com/astral-sh/ruff/pull/14236
pub(crate) fn is_allow_nested_roots_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::AllowNestedRoots)
}

// https://github.com/astral-sh/ruff/pull/20659
pub(crate) fn is_future_required_preview_generics_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FutureRequiredPreviewGenerics)
}

// https://github.com/astral-sh/ruff/pull/20169
pub(crate) fn is_fix_builtin_open_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FixBuiltinOpen)
}

// https://github.com/astral-sh/ruff/pull/20200
pub(crate) fn is_refined_submodule_import_match_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::RefinedSubmoduleImportMatch)
}

// https://github.com/astral-sh/ruff/pull/20660
pub(crate) fn is_type_var_default_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::TypeVarDefault)
}

// github.com/astral-sh/ruff/issues/20004
pub(crate) fn is_b006_check_guaranteed_mutable_expr_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::B006CheckGuaranteedMutableExpr)
}

// github.com/astral-sh/ruff/issues/20004
pub(crate) fn is_b006_unsafe_fix_preserve_assignment_expr_enabled(
    settings: &LinterSettings,
) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::B006UnsafeFixPreserveAssignmentExpr)
}

pub(crate) fn is_typing_extensions_str_alias_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::TypingExtensionsStrAlias)
}

// https://github.com/astral-sh/ruff/pull/19045
pub(crate) fn is_extended_i18n_function_matching_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::ExtendedI18nFunctionMatching)
}

// https://github.com/astral-sh/ruff/pull/21374
pub(crate) fn is_extended_snmp_api_path_detection_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::ExtendedSnmpApiPathDetection)
}

// https://github.com/astral-sh/ruff/pull/21395
pub(crate) fn is_enumerate_for_loop_int_index_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::EnumerateForLoopIntIndex)
}

// https://github.com/astral-sh/ruff/pull/21469
pub(crate) fn is_s310_resolve_string_literal_bindings_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::S310ResolveStringLiteralBindings)
}

// https://github.com/astral-sh/ruff/pull/22057
pub(crate) fn is_ble001_exc_info_suppression_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::Ble001ExcInfoSuppression)
}

// https://github.com/astral-sh/ruff/pull/22419
pub(crate) fn is_py315_support_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::Py315Support)
}

// https://github.com/astral-sh/ruff/pull/23046
pub(crate) fn is_mutable_default_in_dataclass_field_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::MutableDefaultInDataclassField)
}

// https://github.com/astral-sh/ruff/pull/22830
pub(crate) fn is_standalone_mock_non_existent_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::StandaloneMockNonExistent)
}

// https://github.com/astral-sh/ruff/pull/23013
pub(crate) fn is_up024_precise_highlighting_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::Up024PreciseHighlighting)
}

// https://github.com/astral-sh/ruff/pull/21078
pub(crate) fn is_plural_ngettext_check_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::PluralNgettextCheck)
}

// https://github.com/astral-sh/ruff/pull/19023
pub(crate) fn is_resolve_string_annotation_pyi041_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::ResolveStringAnnotationPyi041)
}

// https://github.com/astral-sh/ruff/pull/23510
pub(crate) fn is_baseloader_safe_in_yaml_load_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::BaseloaderSafeInYamlLoad)
}

// https://github.com/astral-sh/ruff/pull/21373
// Takes `&LintPreviewConfig` instead of `&LinterSettings` because call sites
// in flake8_import_conventions settings need to check this before `LinterSettings` is built.
pub(crate) fn is_expanded_import_conventions_enabled(preview: &LintPreviewConfig) -> bool {
    preview.is_feature_enabled(LintPreviewFeature::ExpandedImportConventions)
}

// https://github.com/astral-sh/ruff/pull/19440
pub(crate) fn is_expanded_pathlib_subclass_check_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::ExpandedPathlibSubclassCheck)
}

// https://github.com/astral-sh/ruff/pull/23535
pub(crate) fn is_file_level_invalid_rule_code_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::FileLevelInvalidRuleCode)
}

// https://github.com/astral-sh/ruff/pull/23473
pub(crate) fn is_incorrect_dict_iterator_comprehension_enabled(settings: &LinterSettings) -> bool {
    settings
        .preview
        .is_feature_enabled(LintPreviewFeature::IncorrectDictIteratorComprehension)
}
