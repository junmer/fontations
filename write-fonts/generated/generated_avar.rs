// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// The [avar (Axis Variations)](https://docs.microsoft.com/en-us/typography/opentype/spec/avar) table
#[derive(Clone, Debug, Default)]
pub struct Avar {
    /// Major version number of the axis variations table — set to 1.
    /// Minor version number of the axis variations table — set to 0.
    pub version: MajorMinor,
    /// The segment maps array — one segment map for each axis, in the order of axes specified in the 'fvar' table.
    pub axis_segment_maps: Vec<SegmentMaps>,
}

impl Avar {
    /// Construct a new `Avar`
    pub fn new(version: MajorMinor, axis_segment_maps: Vec<SegmentMaps>) -> Self {
        Self {
            version,
            axis_segment_maps,
        }
    }
}

impl FontWrite for Avar {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        self.version.write_into(writer);
        (0 as u16).write_into(writer);
        (array_len(&self.axis_segment_maps).unwrap() as u16).write_into(writer);
        self.axis_segment_maps.write_into(writer);
    }
}

impl Validate for Avar {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("Avar", |ctx| {
            ctx.in_field("axis_segment_maps", |ctx| {
                self.axis_segment_maps.validate_impl(ctx);
            });
        })
    }
}

impl TopLevelTable for Avar {
    const TAG: Tag = Tag::new(b"avar");
}

impl<'a> FromObjRef<read_fonts::tables::avar::Avar<'a>> for Avar {
    fn from_obj_ref(obj: &read_fonts::tables::avar::Avar<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        Avar {
            version: obj.version(),
            axis_segment_maps: obj
                .axis_segment_maps()
                .iter()
                .filter_map(|x| x.map(|x| FromObjRef::from_obj_ref(&x, offset_data)).ok())
                .collect(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::avar::Avar<'a>> for Avar {}

impl<'a> FontRead<'a> for Avar {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::avar::Avar as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// [SegmentMaps](https://learn.microsoft.com/en-us/typography/opentype/spec/avar#table-formats) record
#[derive(Clone, Debug, Default)]
pub struct SegmentMaps {
    /// The array of axis value map records for this axis.
    pub axis_value_maps: Vec<AxisValueMap>,
}

impl SegmentMaps {
    /// Construct a new `SegmentMaps`
    pub fn new(axis_value_maps: Vec<AxisValueMap>) -> Self {
        Self {
            axis_value_maps: axis_value_maps.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for SegmentMaps {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (array_len(&self.axis_value_maps).unwrap() as u16).write_into(writer);
        self.axis_value_maps.write_into(writer);
    }
}

impl Validate for SegmentMaps {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("SegmentMaps", |ctx| {
            ctx.in_field("axis_value_maps", |ctx| {
                if self.axis_value_maps.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.axis_value_maps.validate_impl(ctx);
            });
        })
    }
}

impl FromObjRef<read_fonts::tables::avar::SegmentMaps<'_>> for SegmentMaps {
    fn from_obj_ref(obj: &read_fonts::tables::avar::SegmentMaps, offset_data: FontData) -> Self {
        SegmentMaps {
            axis_value_maps: obj.axis_value_maps().to_owned_obj(offset_data),
        }
    }
}

/// [AxisValueMap](https://learn.microsoft.com/en-us/typography/opentype/spec/avar#table-formats) record
#[derive(Clone, Debug, Default)]
pub struct AxisValueMap {
    /// A normalized coordinate value obtained using default normalization.
    pub from_coordinate: F2Dot14,
    /// The modified, normalized coordinate value.
    pub to_coordinate: F2Dot14,
}

impl AxisValueMap {
    /// Construct a new `AxisValueMap`
    pub fn new(from_coordinate: F2Dot14, to_coordinate: F2Dot14) -> Self {
        Self {
            from_coordinate,
            to_coordinate,
        }
    }
}

impl FontWrite for AxisValueMap {
    fn write_into(&self, writer: &mut TableWriter) {
        self.from_coordinate.write_into(writer);
        self.to_coordinate.write_into(writer);
    }
}

impl Validate for AxisValueMap {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl FromObjRef<read_fonts::tables::avar::AxisValueMap> for AxisValueMap {
    fn from_obj_ref(obj: &read_fonts::tables::avar::AxisValueMap, _: FontData) -> Self {
        AxisValueMap {
            from_coordinate: obj.from_coordinate(),
            to_coordinate: obj.to_coordinate(),
        }
    }
}