#![no_main]
//! Fuzzes the incremental_font_transfer patch_group.rs API

use std::{
    collections::{BTreeSet, HashMap, HashSet},
    ops::RangeInclusive,
};

use incremental_font_transfer::{
    patch_group::{PatchGroup, UriStatus},
    patchmap::SubsetDefinition,
};
use libfuzzer_sys::{arbitrary, fuzz_target};
use read_fonts::{collections::IntSet, types::Tag};
use skrifa::FontRef;
use write_fonts::FontBuilder;

#[derive(Debug, arbitrary::Arbitrary)]
struct FuzzInput {
    // Build font up from tagged data blobs to bypass some of the complexity in finding a valid font file
    // none of the IFT code should have issues with fonts malformed at the top level as parsing is left
    // up to read-fonts and skrifa.
    font_tables: HashMap<u32, Vec<u8>>,

    // Parts of the target subset definition.
    codepoints: HashSet<u32>,
    features: HashSet<u32>,
    design_space: HashMap<u32, Vec<(f64, f64)>>,

    // Patches
    patches: HashMap<String, Vec<u8>>,
    applied_patches: HashSet<String>,
}

impl FuzzInput {
    fn to_font(&self) -> Vec<u8> {
        let mut font_builder = FontBuilder::new();

        self.font_tables
            .iter()
            .map(|(tag, data)| (Tag::from_u32(*tag), data))
            .for_each(|(tag, data)| {
                font_builder.add_raw(tag, data);
            });

        font_builder.build()
    }

    fn to_subset_definition(&self) -> SubsetDefinition {
        let codepoints: IntSet<u32> = self.codepoints.iter().copied().collect();
        let feature_tags: BTreeSet<Tag> =
            self.features.iter().copied().map(Tag::from_u32).collect();

        let design_space: HashMap<Tag, Vec<RangeInclusive<f64>>> = self
            .design_space
            .iter()
            .map(|(tag, v)| {
                let v: Vec<RangeInclusive<f64>> =
                    v.iter().map(|(start, end)| *start..=*end).collect();
                (Tag::from_u32(*tag), v)
            })
            .collect();
        SubsetDefinition::new(codepoints, feature_tags, design_space)
    }
}

/// Used to ensure read only functions don't get optimized away.
fn black_box<T>(dummy: T) -> T {
    unsafe { std::ptr::read_volatile(&dummy) }
}

fuzz_target!(|input: FuzzInput| {
    let font_data = input.to_font();
    let Ok(font) = FontRef::new(&font_data) else {
        return;
    };

    let subset_definition = input.to_subset_definition();

    let Ok(group) = PatchGroup::select_next_patches(font, &subset_definition) else {
        return;
    };

    // Exercise uris() api on group
    black_box(group.has_uris());
    for uri in group.uris() {
        black_box(uri);
    }

    // Exercise patch application.
    let mut uri_map: HashMap<String, UriStatus> = input
        .patches
        .into_iter()
        .map(|(uri, data)| (uri, UriStatus::Pending(data)))
        .collect();
    for uri in input.applied_patches {
        uri_map.insert(uri.to_string(), UriStatus::Applied);
    }

    let _ = black_box(group.apply_next_patches(&mut uri_map));
});