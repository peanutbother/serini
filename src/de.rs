use crate::{Error, error::Result};
use serde::{
    Deserialize,
    de::{self, IntoDeserializer},
};
use std::collections::HashMap;
use std::str::FromStr;

pub struct Deserializer {
    sections: HashMap<String, HashMap<String, String>>,
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s)?;
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

impl Deserializer {
    fn from_str(input: &str) -> Result<Self> {
        let mut sections = HashMap::new();
        let mut current_section = String::new();
        sections.insert(current_section.clone(), HashMap::new());

        for line in input.lines() {
            let line = line.trim();

            // Skip empty lines and comments
            if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
                continue;
            }

            // Section header
            if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len() - 1].to_string();
                sections.insert(current_section.clone(), HashMap::new());
                continue;
            }

            // Key-value pair
            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim().to_string();
                let value = Self::unescape_value(line[eq_pos + 1..].trim());

                if let Some(section) = sections.get_mut(&current_section) {
                    section.insert(key, value);
                }
            }
        }

        Ok(Deserializer { sections })
    }

    fn unescape_value(value: &str) -> String {
        value
            .replace("\\\\", "\\")
            .replace("\\n", "\n")
            .replace("\\r", "\r")
            .replace("\\t", "\t")
            .replace("\\\"", "\"")
            .replace("\\;", ";")
            .replace("\\#", "#")
    }
}

impl<'de> de::Deserializer<'de> for &mut Deserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_bool(true)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i8(0)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i16(0)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i32(0)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i64(0)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u8(0)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u16(0)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u32(0)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u64(0)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f32(0.0)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f64(0.0)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_char('a')
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_borrowed_str("")
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_string(String::new())
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_bytes(b"")
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_byte_buf(Vec::new())
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedFeature("sequences".to_string()))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedFeature("tuples".to_string()))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedFeature("tuple structs".to_string()))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_map(MapAccess::new(self))
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        // For root struct or when the struct name exists as a section
        if name.is_empty() || self.sections.contains_key(name) {
            if name.is_empty() {
                // Root struct - deserialize the whole INI file
                visitor.visit_map(RootStructAccess::new(self))
            } else {
                // Named section exists
                visitor.visit_map(StructAccess::new(self, name))
            }
        } else {
            // Check if any section exists (for renamed structs)
            // This handles cases where the struct might be renamed via serde
            if self.sections.len() > 1
                || (self.sections.len() == 1 && !self.sections.contains_key(""))
            {
                // We have sections, assume root struct
                visitor.visit_map(RootStructAccess::new(self))
            } else {
                // No sections or only root section
                visitor.visit_map(StructAccess::new(self, ""))
            }
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedFeature("enums".to_string()))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_str("")
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

struct MapAccess<'a> {
    de: &'a mut Deserializer,
    sections: Vec<String>,
    index: usize,
}

impl<'a> MapAccess<'a> {
    fn new(de: &'a mut Deserializer) -> Self {
        let sections: Vec<String> = de.sections.keys().cloned().collect();
        MapAccess {
            de,
            sections,
            index: 0,
        }
    }
}

// Root struct access - handles both root fields and sections
struct RootStructAccess<'a> {
    de: &'a mut Deserializer,
    root_fields: Vec<(String, String)>,
    sections: Vec<String>,
    index: usize,
    in_sections: bool,
}

impl<'a> RootStructAccess<'a> {
    fn new(de: &'a mut Deserializer) -> Self {
        let root_fields = if let Some(root_section) = de.sections.get("") {
            root_section
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect()
        } else {
            Vec::new()
        };

        let sections: Vec<String> = de
            .sections
            .keys()
            .filter(|k| !k.is_empty())
            .cloned()
            .collect();

        RootStructAccess {
            de,
            root_fields,
            sections,
            index: 0,
            in_sections: false,
        }
    }
}

impl<'de> de::MapAccess<'de> for RootStructAccess<'_> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if !self.in_sections && self.index < self.root_fields.len() {
            let (key, _) = &self.root_fields[self.index];
            self.index += 1;
            seed.deserialize(key.as_str().into_deserializer()).map(Some)
        } else if !self.in_sections {
            // Switch to sections
            self.in_sections = true;
            self.index = 0;
            self.next_key_seed(seed)
        } else if self.index < self.sections.len() {
            let key = &self.sections[self.index];
            self.index += 1;
            seed.deserialize(key.as_str().into_deserializer()).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        if !self.in_sections {
            let (_, value) = &self.root_fields[self.index - 1];
            seed.deserialize(ValueDeserializer::new(value))
        } else {
            let section = &self.sections[self.index - 1];
            seed.deserialize(&mut SectionDeserializer::new(self.de, section))
        }
    }
}

impl<'de> de::MapAccess<'de> for MapAccess<'_> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.index < self.sections.len() {
            let key = &self.sections[self.index];
            self.index += 1;
            seed.deserialize(key.as_str().into_deserializer()).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        let section = &self.sections[self.index - 1];
        seed.deserialize(&mut SectionDeserializer::new(self.de, section))
    }
}

struct StructAccess {
    fields: Vec<(String, String)>,
    index: usize,
}

impl StructAccess {
    fn new(de: &mut Deserializer, section: &str) -> Self {
        let fields = if let Some(section_map) = de.sections.get(section) {
            section_map
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect()
        } else {
            Vec::new()
        };

        StructAccess { fields, index: 0 }
    }
}

// Section deserializer for nested structs
struct SectionDeserializer<'a> {
    de: &'a mut Deserializer,
    section: String,
}

impl<'a> SectionDeserializer<'a> {
    fn new(de: &'a mut Deserializer, section: &str) -> Self {
        SectionDeserializer {
            de,
            section: section.to_string(),
        }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut SectionDeserializer<'a> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_struct("", &[], visitor)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_map(StructAccess::new(self.de, &self.section))
    }

    // Forward all other deserialize methods to deserialize_any
    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map enum identifier ignored_any
    }
}

impl<'de> de::MapAccess<'de> for StructAccess {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.index < self.fields.len() {
            let (key, _) = &self.fields[self.index];
            self.index += 1;
            seed.deserialize(key.as_str().into_deserializer()).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        let (_, value) = &self.fields[self.index - 1];
        seed.deserialize(ValueDeserializer::new(value))
    }
}

struct ValueDeserializer {
    value: String,
}

impl ValueDeserializer {
    fn new(value: &str) -> Self {
        ValueDeserializer {
            value: value.to_string(),
        }
    }
}

impl<'de> de::Deserializer<'de> for ValueDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value.as_str() {
            "true" => visitor.visit_bool(true),
            "false" => visitor.visit_bool(false),
            _ => Err(Error::InvalidValue {
                typ: "bool".to_string(),
                value: self.value,
            }),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i8(i8::from_str(&self.value).map_err(|_| Error::InvalidValue {
            typ: "i8".to_string(),
            value: self.value.clone(),
        })?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i16(i16::from_str(&self.value).map_err(|_| Error::InvalidValue {
            typ: "i16".to_string(),
            value: self.value.clone(),
        })?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i32(i32::from_str(&self.value).map_err(|_| Error::InvalidValue {
            typ: "i32".to_string(),
            value: self.value.clone(),
        })?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i64(i64::from_str(&self.value).map_err(|_| Error::InvalidValue {
            typ: "i64".to_string(),
            value: self.value.clone(),
        })?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u8(u8::from_str(&self.value).map_err(|_| Error::InvalidValue {
            typ: "u8".to_string(),
            value: self.value.clone(),
        })?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u16(u16::from_str(&self.value).map_err(|_| Error::InvalidValue {
            typ: "u16".to_string(),
            value: self.value.clone(),
        })?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u32(u32::from_str(&self.value).map_err(|_| Error::InvalidValue {
            typ: "u32".to_string(),
            value: self.value.clone(),
        })?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u64(u64::from_str(&self.value).map_err(|_| Error::InvalidValue {
            typ: "u64".to_string(),
            value: self.value.clone(),
        })?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f32(f32::from_str(&self.value).map_err(|_| Error::InvalidValue {
            typ: "f32".to_string(),
            value: self.value.clone(),
        })?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f64(f64::from_str(&self.value).map_err(|_| Error::InvalidValue {
            typ: "f64".to_string(),
            value: self.value.clone(),
        })?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.value.len() == 1 {
            visitor.visit_char(self.value.chars().next().unwrap())
        } else {
            Err(Error::InvalidValue {
                typ: "char".to_string(),
                value: self.value,
            })
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_string(self.value)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_string(self.value)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_bytes(self.value.as_bytes())
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_byte_buf(self.value.into_bytes())
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedFeature("sequences".to_string()))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedFeature("tuples".to_string()))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedFeature("tuple structs".to_string()))
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedFeature("maps in values".to_string()))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedFeature("structs in values".to_string()))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedFeature("enums".to_string()))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }
}
