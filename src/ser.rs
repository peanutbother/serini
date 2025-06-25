use crate::{Error, error::Result};
use serde::{Serialize, ser};

pub struct Serializer {
    output: String,
    current_section: Option<String>,
    section_names: Vec<String>,
}

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: String::new(),
        current_section: None,
        section_names: Vec::new(),
    };

    // First pass: collect all section names
    let mut section_collector = SectionCollector {
        sections: Vec::new(),
    };
    value.serialize(&mut section_collector)?;
    serializer.section_names = section_collector.sections;

    // Second pass: actual serialization
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

// Helper to collect section names
struct SectionCollector {
    sections: Vec<String>,
}

impl ser::Serializer for &mut SectionCollector {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    // All other methods just return Ok(())
    fn serialize_bool(self, _v: bool) -> Result<()> {
        Ok(())
    }
    fn serialize_i8(self, _v: i8) -> Result<()> {
        Ok(())
    }
    fn serialize_i16(self, _v: i16) -> Result<()> {
        Ok(())
    }
    fn serialize_i32(self, _v: i32) -> Result<()> {
        Ok(())
    }
    fn serialize_i64(self, _v: i64) -> Result<()> {
        Ok(())
    }
    fn serialize_u8(self, _v: u8) -> Result<()> {
        Ok(())
    }
    fn serialize_u16(self, _v: u16) -> Result<()> {
        Ok(())
    }
    fn serialize_u32(self, _v: u32) -> Result<()> {
        Ok(())
    }
    fn serialize_u64(self, _v: u64) -> Result<()> {
        Ok(())
    }
    fn serialize_f32(self, _v: f32) -> Result<()> {
        Ok(())
    }
    fn serialize_f64(self, _v: f64) -> Result<()> {
        Ok(())
    }
    fn serialize_char(self, _v: char) -> Result<()> {
        Ok(())
    }
    fn serialize_str(self, _v: &str) -> Result<()> {
        Ok(())
    }
    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        Ok(())
    }
    fn serialize_none(self) -> Result<()> {
        Ok(())
    }
    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }
    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Ok(())
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        Ok(())
    }
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(self)
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(self)
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(self)
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(self)
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(self)
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(self)
    }
}

impl ser::SerializeStruct for &mut SectionCollector {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // Check if this field is a struct that will become a section
        let mut detector = StructDetector::new();
        let _ = value.serialize(&mut detector);

        if detector.is_struct {
            self.sections.push(key.to_string());
        }

        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Implement dummy traits for SectionCollector
impl ser::SerializeSeq for &mut SectionCollector {
    type Ok = ();
    type Error = Error;
    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeTuple for &mut SectionCollector {
    type Ok = ();
    type Error = Error;
    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeTupleStruct for &mut SectionCollector {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeTupleVariant for &mut SectionCollector {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeMap for &mut SectionCollector {
    type Ok = ();
    type Error = Error;
    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }
    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeStructVariant for &mut SectionCollector {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }
    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Helper struct to detect if a value serializes as a struct
struct StructDetector {
    is_struct: bool,
}

impl StructDetector {
    fn new() -> Self {
        StructDetector { is_struct: false }
    }
}

impl ser::Serializer for &mut StructDetector {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        self.is_struct = true;
        Ok(self)
    }

    // All other methods just return Ok(())
    fn serialize_bool(self, _v: bool) -> Result<()> {
        Ok(())
    }
    fn serialize_i8(self, _v: i8) -> Result<()> {
        Ok(())
    }
    fn serialize_i16(self, _v: i16) -> Result<()> {
        Ok(())
    }
    fn serialize_i32(self, _v: i32) -> Result<()> {
        Ok(())
    }
    fn serialize_i64(self, _v: i64) -> Result<()> {
        Ok(())
    }
    fn serialize_u8(self, _v: u8) -> Result<()> {
        Ok(())
    }
    fn serialize_u16(self, _v: u16) -> Result<()> {
        Ok(())
    }
    fn serialize_u32(self, _v: u32) -> Result<()> {
        Ok(())
    }
    fn serialize_u64(self, _v: u64) -> Result<()> {
        Ok(())
    }
    fn serialize_f32(self, _v: f32) -> Result<()> {
        Ok(())
    }
    fn serialize_f64(self, _v: f64) -> Result<()> {
        Ok(())
    }
    fn serialize_char(self, _v: char) -> Result<()> {
        Ok(())
    }
    fn serialize_str(self, _v: &str) -> Result<()> {
        Ok(())
    }
    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        Ok(())
    }
    fn serialize_none(self) -> Result<()> {
        Ok(())
    }
    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }
    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Ok(())
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        Ok(())
    }
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(self)
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(self)
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(self)
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(self)
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(self)
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(self)
    }
}

impl ser::SerializeSeq for &mut StructDetector {
    type Ok = ();
    type Error = Error;
    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeTuple for &mut StructDetector {
    type Ok = ();
    type Error = Error;
    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeTupleStruct for &mut StructDetector {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeTupleVariant for &mut StructDetector {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeMap for &mut StructDetector {
    type Ok = ();
    type Error = Error;
    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }
    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeStruct for &mut StructDetector {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeStructVariant for &mut StructDetector {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl Serializer {
    fn escape_value(value: &str) -> String {
        value
            .replace('\\', "\\\\")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t")
            .replace('"', "\\\"")
            .replace(';', "\\;")
            .replace('#', "\\#")
    }

    fn write_key_value(&mut self, key: &str, value: &str) {
        self.output.push_str(key);
        self.output.push_str(" = ");
        self.output.push_str(&Self::escape_value(value));
        self.output.push('\n');
    }

    fn write_commented_key(&mut self, key: &str) {
        self.output.push_str("; ");
        self.output.push_str(key);
        self.output.push_str(" = \n");
    }
}

impl ser::Serializer for &mut Serializer {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.output.push_str(if v { "true" } else { "false" });
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.output.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.output.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.output.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.output.push(v);
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.output.push_str(v);
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.serialize_str(&String::from_utf8_lossy(v))
    }

    fn serialize_none(self) -> Result<()> {
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::UnsupportedFeature("enum variants".to_string()))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::UnsupportedFeature("sequences".to_string()))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::UnsupportedFeature("tuples".to_string()))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::UnsupportedFeature("tuple structs".to_string()))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::UnsupportedFeature("tuple variants".to_string()))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        // Don't create section headers here - they're created in serialize_field
        if self.current_section.is_none() {
            self.current_section = Some("".to_string());
        }
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::UnsupportedFeature("struct variants".to_string()))
    }
}

impl ser::SerializeSeq for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::UnsupportedFeature("sequences".to_string()))
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeTuple for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::UnsupportedFeature("tuples".to_string()))
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeTupleStruct for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::UnsupportedFeature("tuple structs".to_string()))
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeTupleVariant for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::UnsupportedFeature("tuple variants".to_string()))
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeMap for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeStruct for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // First, detect if the value is a struct
        let mut detector = StructDetector::new();
        let _ = value.serialize(&mut detector);

        if detector.is_struct {
            // This is a nested struct - serialize it as a section
            if !self.output.is_empty() && !self.output.ends_with('\n') {
                self.output.push('\n');
            }
            self.output.push('[');
            self.output.push_str(key);
            self.output.push_str("]\n");

            // Serialize the struct's fields
            let mut nested_serializer = Serializer {
                output: String::new(),
                current_section: Some(key.to_string()),
                section_names: self.section_names.clone(),
            };
            value.serialize(&mut nested_serializer)?;

            // Add the fields (the nested serializer won't have section headers)
            self.output.push_str(&nested_serializer.output);
        } else {
            // Regular value or Option
            let mut temp_serializer = Serializer {
                output: String::new(),
                current_section: self.current_section.clone(),
                section_names: self.section_names.clone(),
            };

            match value.serialize(&mut temp_serializer) {
                Ok(_) => {
                    if temp_serializer.output.is_empty() {
                        // This was None
                        // Skip commented lines for fields that are section names
                        if !self.section_names.contains(&key.to_string()) {
                            self.write_commented_key(key);
                        }
                    } else {
                        // This was Some(value) or a regular value
                        self.write_key_value(key, &temp_serializer.output);
                    }
                }
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeStructVariant for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::UnsupportedFeature("struct variants".to_string()))
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}
