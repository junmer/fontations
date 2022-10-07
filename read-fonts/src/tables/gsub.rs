//! the [GSUB] table
//!
//! [GSUB]: https://docs.microsoft.com/en-us/typography/opentype/spec/gsub

use font_types::Tag;

pub use super::layout::{
    ChainedSequenceContext, ClassDef, CoverageTable, Device, FeatureList, FeatureVariations,
    Lookup, LookupList, ScriptList, SequenceContext,
};

#[cfg(test)]
#[path = "../tests/test_gsub.rs"]
mod tests;

/// 'GSUB'
pub const TAG: Tag = Tag::new(b"GSUB");

include!("../../generated/generated_gsub.rs");

/// A typed GSUB [LookupList] table
pub type SubstitutionLookupList<'a> = LookupList<'a, SubstitutionLookup<'a>>;

/// A GSUB [SequenceContext](super::layout::SequenceContext)
pub type SubstitutionSequenceContext<'a> = super::layout::SequenceContext<'a>;

/// A GSUB [ChainedSequenceContext](super::layout::ChainedSequenceContext)
pub type SubstitutionChainContext<'a> = super::layout::ChainedSequenceContext<'a>;