// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

#[derive(Clone, Debug, Default)]
pub struct BasicTable {
    pub simple_records: Vec<SimpleRecord>,
    pub array_records: Vec<ContainsArrays>,
}

impl BasicTable {
    /// Construct a new `BasicTable`
    pub fn new(simple_records: Vec<SimpleRecord>, array_records: Vec<ContainsArrays>) -> Self {
        Self {
            simple_records: simple_records.into_iter().map(Into::into).collect(),
            array_records,
        }
    }
}

impl FontWrite for BasicTable {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (array_len(&self.simple_records).unwrap() as u16).write_into(writer);
        self.simple_records.write_into(writer);
        (self.compute_arrays_inner_count() as u16).write_into(writer);
        (array_len(&self.array_records).unwrap() as u32).write_into(writer);
        self.array_records.write_into(writer);
    }
}

impl Validate for BasicTable {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("BasicTable", |ctx| {
            ctx.in_field("simple_records", |ctx| {
                if self.simple_records.len() > (u16::MAX as usize) {
                    ctx.report("array excedes max length");
                }
                self.simple_records.validate_impl(ctx);
            });
            ctx.in_field("array_records", |ctx| {
                if self.array_records.len() > (u32::MAX as usize) {
                    ctx.report("array excedes max length");
                }
                self.array_records.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::codegen_test::records::BasicTable<'a>> for BasicTable {
    fn from_obj_ref(obj: &read_fonts::codegen_test::records::BasicTable<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        BasicTable {
            simple_records: obj.simple_records().to_owned_obj(offset_data),
            array_records: obj
                .array_records()
                .iter()
                .filter_map(|x| x.map(|x| FromObjRef::from_obj_ref(&x, offset_data)).ok())
                .collect(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::codegen_test::records::BasicTable<'a>> for BasicTable {}

impl<'a> FontRead<'a> for BasicTable {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::codegen_test::records::BasicTable as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

#[derive(Clone, Debug, Default)]
pub struct SimpleRecord {
    pub val1: u16,
    pub va2: u32,
}

impl SimpleRecord {
    /// Construct a new `SimpleRecord`
    pub fn new(val1: u16, va2: u32) -> Self {
        Self { val1, va2 }
    }
}

impl FontWrite for SimpleRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.val1.write_into(writer);
        self.va2.write_into(writer);
    }
}

impl Validate for SimpleRecord {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl FromObjRef<read_fonts::codegen_test::records::SimpleRecord> for SimpleRecord {
    fn from_obj_ref(obj: &read_fonts::codegen_test::records::SimpleRecord, _: FontData) -> Self {
        SimpleRecord {
            val1: obj.val1(),
            va2: obj.va2(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ContainsArrays {
    pub scalars: Vec<u16>,
    pub records: Vec<SimpleRecord>,
}

impl ContainsArrays {
    /// Construct a new `ContainsArrays`
    pub fn new(scalars: Vec<u16>, records: Vec<SimpleRecord>) -> Self {
        Self {
            scalars: scalars.into_iter().map(Into::into).collect(),
            records: records.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for ContainsArrays {
    fn write_into(&self, writer: &mut TableWriter) {
        self.scalars.write_into(writer);
        self.records.write_into(writer);
    }
}

impl Validate for ContainsArrays {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("ContainsArrays", |ctx| {
            ctx.in_field("scalars", |ctx| {
                if self.scalars.len() > (u16::MAX as usize) {
                    ctx.report("array excedes max length");
                }
            });
            ctx.in_field("records", |ctx| {
                if self.records.len() > (u16::MAX as usize) {
                    ctx.report("array excedes max length");
                }
                self.records.validate_impl(ctx);
            });
        })
    }
}

impl FromObjRef<read_fonts::codegen_test::records::ContainsArrays<'_>> for ContainsArrays {
    fn from_obj_ref(
        obj: &read_fonts::codegen_test::records::ContainsArrays,
        offset_data: FontData,
    ) -> Self {
        ContainsArrays {
            scalars: obj.scalars().to_owned_obj(offset_data),
            records: obj.records().to_owned_obj(offset_data),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ContainsOffests {
    pub array: OffsetMarker<Vec<SimpleRecord>>,
    pub other: OffsetMarker<BasicTable, WIDTH_32>,
}

impl ContainsOffests {
    /// Construct a new `ContainsOffests`
    pub fn new(array: Vec<SimpleRecord>, other: BasicTable) -> Self {
        Self {
            array: array.into(),
            other: other.into(),
        }
    }
}

impl FontWrite for ContainsOffests {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (array_len(&self.array).unwrap() as u16).write_into(writer);
        self.array.write_into(writer);
        self.other.write_into(writer);
    }
}

impl Validate for ContainsOffests {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("ContainsOffests", |ctx| {
            ctx.in_field("array", |ctx| {
                self.array.validate_impl(ctx);
            });
            ctx.in_field("other", |ctx| {
                self.other.validate_impl(ctx);
            });
        })
    }
}

impl FromObjRef<read_fonts::codegen_test::records::ContainsOffests> for ContainsOffests {
    fn from_obj_ref(
        obj: &read_fonts::codegen_test::records::ContainsOffests,
        offset_data: FontData,
    ) -> Self {
        ContainsOffests {
            array: obj.array(offset_data).to_owned_obj(offset_data),
            other: obj.other(offset_data).to_owned_table(),
        }
    }
}