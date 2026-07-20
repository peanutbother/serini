use crate::{Error, error::Result};
use serde_core::{
    Deserialize,
    de::{self, IntoDeserializer},
};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Section {
    name: String,
    fields: HashMap<String, String>,
}

pub struct Deserializer {
    sections: Vec<Section>,
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
        let mut sections = vec![Section {
            name: String::new(),
            fields: HashMap::new(),
        }];
        let mut current_section = 0usize;

        for line in input.lines() {
            let line = line.trim();

            // Skip empty lines and comments
            if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
                continue;
            }

            // Section header
            if line.starts_with('[') && line.ends_with(']') {
                sections.push(Section {
                    name: line[1..line.len() - 1].to_string(),
                    fields: HashMap::new(),
                });
                current_section = sections.len() - 1;
                continue;
            }

            // Key-value pair
            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim().to_string();
                let value = Self::unescape_value(line[eq_pos + 1..].trim())?;

                sections[current_section].fields.insert(key, value);
            }
        }

        Ok(Deserializer { sections })
    }

    fn section_base_name(name: &str) -> &str {
        if let Some((base, suffix)) = name.rsplit_once('.')
            && !suffix.is_empty()
            && suffix.chars().all(|c| c.is_ascii_digit())
        {
            return base;
        }

        name
    }

    fn pascal_case(name: &str) -> String {
        name.split('_')
            .filter(|part| !part.is_empty())
            .map(|part| {
                let mut chars = part.chars();
                match chars.next() {
                    Some(first) => {
                        let mut result = first.to_uppercase().to_string();
                        result.push_str(chars.as_str());
                        result
                    }
                    None => String::new(),
                }
            })
            .collect()
    }

    fn singular_candidates(name: &str) -> Vec<String> {
        let mut candidates = vec![name.to_string()];

        let singular = if let Some(base) = name.strip_suffix("ies") {
            format!("{base}y")
        } else if let Some(base) = name.strip_suffix("ses") {
            base.to_string()
        } else if let Some(base) = name.strip_suffix('s') {
            base.to_string()
        } else {
            String::new()
        };

        if !singular.is_empty() && singular != name {
            candidates.push(singular.clone());
            let pascal = Self::pascal_case(&singular);
            if pascal != singular {
                candidates.push(pascal);
            }
        }

        let pascal = Self::pascal_case(name);
        if pascal != name {
            candidates.push(pascal);
        }

        candidates.sort();
        candidates.dedup();
        candidates
    }

    fn section_indices_matching(&self, name: &str) -> Vec<usize> {
        let candidates = Self::singular_candidates(name);
        self.sections
            .iter()
            .enumerate()
            .filter_map(|(index, section)| {
                let base = Self::section_base_name(&section.name);
                candidates
                    .iter()
                    .any(|candidate| candidate == base)
                    .then_some(index)
            })
            .collect()
    }

    fn section_index(&self, name: &str) -> Option<usize> {
        self.sections
            .iter()
            .enumerate()
            .find_map(|(index, section)| {
                (Self::section_base_name(&section.name) == name).then_some(index)
            })
    }

    fn section(&self, index: usize) -> &Section {
        &self.sections[index]
    }

    fn unescape_value(value: &str) -> Result<String> {
        let mut unescaped = String::with_capacity(value.len());
        let mut iter = value.chars();
        while let Some(c) = iter.next() {
            match c {
                '\\' => match iter
                    .next()
                    .ok_or(Error::Unescape("Backslash without escape".to_string()))?
                {
                    '0' => unescaped.push('\0'),
                    'a' => unescaped.push('\x07'),
                    'b' => unescaped.push('\x08'),
                    't' => unescaped.push('\t'),
                    'r' => unescaped.push('\r'),
                    'n' => unescaped.push('\n'),
                    'x' => {
                        let s: String = iter.by_ref().take(4).collect();
                        let codepoint = u32::from_str_radix(&s, 16)
                            .map_err(|e| Error::Unescape(e.to_string()))?;
                        let char = char::from_u32(codepoint).ok_or_else(|| {
                            Error::Unescape(format!("{s:?} is not a valid unicode codepoint"))
                        })?;
                        unescaped.push(char);
                    }
                    escapee => unescaped.push(escapee),
                },
                c => unescaped.push(c),
            }
        }
        Ok(unescaped)
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

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(SequenceDeserializer::new(
            self,
            (1..self.sections.len()).collect(),
        ))
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
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if name.is_empty() {
            visitor.visit_map(RootStructAccess::new(self, fields))
        } else if let Some(index) = self.section_index(name) {
            visitor.visit_map(StructAccess::new(self, index))
        } else {
            visitor.visit_map(RootStructAccess::new(self, fields))
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
    sections: Vec<usize>,
    index: usize,
}

impl<'a> MapAccess<'a> {
    fn new(de: &'a mut Deserializer) -> Self {
        let sections: Vec<usize> = (0..de.sections.len()).collect();
        MapAccess {
            de,
            sections,
            index: 0,
        }
    }
}

// Enum to track field source
enum FieldSource {
    Root(String),
    Section(usize),
    Sequence(Vec<usize>),
}

// Root struct access - handles both root fields and sections
struct RootStructAccess<'a> {
    de: &'a mut Deserializer,
    fields: Vec<(String, FieldSource)>,
    index: usize,
}

impl<'a> RootStructAccess<'a> {
    fn new(de: &'a mut Deserializer, expected_fields: &'static [&'static str]) -> Self {
        let mut fields = Vec::new();
        let root_section = de.section(0);

        for &field_name in expected_fields {
            let matching_sections = de.section_indices_matching(field_name);
            if matching_sections.len() > 1 {
                fields.push((
                    field_name.to_string(),
                    FieldSource::Sequence(matching_sections),
                ));
            } else if matching_sections.len() == 1 {
                let index = matching_sections[0];
                let section = de.section(index);
                if section.fields.len() == 1 && section.fields.contains_key("value") {
                    fields.push((field_name.to_string(), FieldSource::Sequence(vec![index])));
                } else {
                    let section_name = section.name.as_str();
                    let singular = Deserializer::singular_candidates(field_name);
                    if singular.iter().any(|candidate| {
                        candidate != field_name
                            && Deserializer::section_base_name(section_name) == candidate
                    }) {
                        fields.push((field_name.to_string(), FieldSource::Sequence(vec![index])));
                    } else {
                        fields.push((field_name.to_string(), FieldSource::Section(index)));
                    }
                }
            } else if let Some(value) = root_section.fields.get(field_name) {
                fields.push((field_name.to_string(), FieldSource::Root(value.clone())));
            }
        }

        RootStructAccess {
            de,
            fields,
            index: 0,
        }
    }
}

impl<'de> de::MapAccess<'de> for RootStructAccess<'_> {
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
        let (_, source) = &self.fields[self.index - 1];
        match source {
            FieldSource::Root(value) => seed.deserialize(ValueDeserializer::new(value)),
            FieldSource::Section(index) => {
                seed.deserialize(&mut SectionDeserializer::new(self.de, *index))
            }
            FieldSource::Sequence(indices) => seed.deserialize(
                &mut SequenceValueDeserializer::new(self.de, indices.clone()),
            ),
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
            let key = self.de.section(self.sections[self.index]).name.clone();
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
        let section = self.sections[self.index - 1];
        seed.deserialize(&mut SectionDeserializer::new(self.de, section))
    }
}

struct StructAccess {
    fields: Vec<(String, String)>,
    index: usize,
}

impl StructAccess {
    fn new(de: &mut Deserializer, section_index: usize) -> Self {
        let fields = de
            .section(section_index)
            .fields
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        StructAccess { fields, index: 0 }
    }
}

struct SequenceDeserializer<'a> {
    de: &'a mut Deserializer,
    sections: Vec<usize>,
    index: usize,
}

impl<'a> SequenceDeserializer<'a> {
    fn new(de: &'a mut Deserializer, sections: Vec<usize>) -> Self {
        SequenceDeserializer {
            de,
            sections,
            index: 0,
        }
    }
}

impl<'de, 'a> de::SeqAccess<'de> for SequenceDeserializer<'a> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.index < self.sections.len() {
            let section = self.sections[self.index];
            self.index += 1;
            seed.deserialize(&mut SectionDeserializer::new(self.de, section))
                .map(Some)
        } else {
            Ok(None)
        }
    }
}

struct SequenceValueDeserializer<'a> {
    inner: SequenceDeserializer<'a>,
}

impl<'a> SequenceValueDeserializer<'a> {
    fn new(de: &'a mut Deserializer, sections: Vec<usize>) -> Self {
        SequenceValueDeserializer {
            inner: SequenceDeserializer::new(de, sections),
        }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut SequenceValueDeserializer<'a> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(&mut self.inner)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    serde_core::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string
        bytes byte_buf unit unit_struct newtype_struct tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

// Section deserializer for nested structs and sequence elements
struct SectionDeserializer<'a> {
    de: &'a mut Deserializer,
    section: usize,
}

impl<'a> SectionDeserializer<'a> {
    fn new(de: &'a mut Deserializer, section: usize) -> Self {
        SectionDeserializer { de, section }
    }

    fn primitive_value(&self) -> Option<&str> {
        let section = self.de.section(self.section);
        section.fields.get("value").map(|value| value.as_str())
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut SectionDeserializer<'a> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if let Some(value) = self.primitive_value() {
            visitor.visit_string(value.to_string())
        } else {
            self.deserialize_struct("", &[], visitor)
        }
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
        visitor.visit_map(StructAccess::new(self.de, self.section))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        // Section exists, so it's Some
        visitor.visit_some(self)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if let Some(value) = self.primitive_value() {
            ValueDeserializer::new(value).deserialize_bool(visitor)
        } else {
            Err(Error::UnsupportedFeature(
                "booleans in sections".to_string(),
            ))
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if let Some(value) = self.primitive_value() {
            ValueDeserializer::new(value).deserialize_i8(visitor)
        } else {
            Err(Error::UnsupportedFeature(
                "integers in sections".to_string(),
            ))
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if let Some(value) = self.primitive_value() {
            ValueDeserializer::new(value).deserialize_i16(visitor)
        } else {
            Err(Error::UnsupportedFeature(
                "integers in sections".to_string(),
            ))
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if let Some(value) = self.primitive_value() {
            ValueDeserializer::new(value).deserialize_i32(visitor)
        } else {
            Err(Error::UnsupportedFeature(
                "integers in sections".to_string(),
            ))
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if let Some(value) = self.primitive_value() {
            ValueDeserializer::new(value).deserialize_i64(visitor)
        } else {
            Err(Error::UnsupportedFeature(
                "integers in sections".to_string(),
            ))
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if let Some(value) = self.primitive_value() {
            ValueDeserializer::new(value).deserialize_u8(visitor)
        } else {
            Err(Error::UnsupportedFeature(
                "integers in sections".to_string(),
            ))
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if let Some(value) = self.primitive_value() {
            ValueDeserializer::new(value).deserialize_u16(visitor)
        } else {
            Err(Error::UnsupportedFeature(
                "integers in sections".to_string(),
            ))
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if let Some(value) = self.primitive_value() {
            ValueDeserializer::new(value).deserialize_u32(visitor)
        } else {
            Err(Error::UnsupportedFeature(
                "integers in sections".to_string(),
            ))
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if let Some(value) = self.primitive_value() {
            ValueDeserializer::new(value).deserialize_u64(visitor)
        } else {
            Err(Error::UnsupportedFeature(
                "integers in sections".to_string(),
            ))
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if let Some(value) = self.primitive_value() {
            ValueDeserializer::new(value).deserialize_f32(visitor)
        } else {
            Err(Error::UnsupportedFeature("floats in sections".to_string()))
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if let Some(value) = self.primitive_value() {
            ValueDeserializer::new(value).deserialize_f64(visitor)
        } else {
            Err(Error::UnsupportedFeature("floats in sections".to_string()))
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if let Some(value) = self.primitive_value() {
            ValueDeserializer::new(value).deserialize_char(visitor)
        } else {
            Err(Error::UnsupportedFeature("chars in sections".to_string()))
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if let Some(value) = self.primitive_value() {
            visitor.visit_string(value.to_string())
        } else {
            Err(Error::UnsupportedFeature("strings in sections".to_string()))
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if let Some(value) = self.primitive_value() {
            visitor.visit_string(value.to_string())
        } else {
            Err(Error::UnsupportedFeature("strings in sections".to_string()))
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if let Some(value) = self.primitive_value() {
            visitor.visit_bytes(value.as_bytes())
        } else {
            Err(Error::UnsupportedFeature("bytes in sections".to_string()))
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if let Some(value) = self.primitive_value() {
            visitor.visit_byte_buf(value.as_bytes().to_vec())
        } else {
            Err(Error::UnsupportedFeature("bytes in sections".to_string()))
        }
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

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let _ = visitor;
        Err(Error::UnsupportedFeature("nested sequences".to_string()))
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
        if self.primitive_value().is_some() {
            Err(Error::UnsupportedFeature(
                "maps in primitive sections".to_string(),
            ))
        } else {
            visitor.visit_map(StructAccess::new(self.de, self.section))
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
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unescape_value() {
        for (escaped, should_unescaped) in [
            (r#"\\\\"#, r#"\\"#),
            (r#"\\n"#, r#"\n"#),
            (r#"\\r"#, r#"\r"#),
            (r#"\\t"#, r#"\t"#),
            (r#"\\\""#, r#"\""#),
            (r#"\\\;"#, r#"\;"#),
            (r#"\\\#"#, r#"\#"#),
        ] {
            assert_eq!(
                should_unescaped.to_string(),
                Deserializer::unescape_value(escaped).unwrap()
            );
        }
    }
}
