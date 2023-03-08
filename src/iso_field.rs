// Copyright 2017 Rohit Joshi <rohit.c.joshi@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::*;
use serde::Serializer;
use strum_macros;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, strum_macros::EnumProperty)]
pub enum FieldCharType {
    #[strum(props(content_type = "number"))]
    Iso8583_n,
    #[strum(props(content_type = "string"))]
    Iso8583_ns,
    #[strum(props(content_type = "string"))]
    Iso8583_xn,
    #[strum(props(content_type = "string"))]
    Iso8583_a,
    #[strum(props(content_type = "string"))]
    Iso8583_an,
    #[strum(props(content_type = "string"))]
    Iso8583_ans,
    #[strum(props(content_type = "binary"))]
    Iso8583_ansb,
    #[strum(props(content_type = "string"))]
    Iso8583_anp,
    #[strum(props(content_type = "binary"))]
    Iso8583_b,
    #[strum(props(content_type = "binary"))]
    Iso8583_z,
    #[strum(props(content_type = "binary"))]
    Iso8583_bmps,
    #[strum(props(content_type = "string"))]
    Iso8583_mti,
    #[strum(props(content_type = "binary"))]
    Iso8583_undefined,
}

impl Default for FieldCharType {
    fn default() -> Self { FieldCharType::Iso8583_undefined }
}

impl FieldCharType {
    pub fn from_str(s: &str) -> Option<FieldCharType> {
        match s {
            "n" => Some(FieldCharType::Iso8583_n),
            "ns" => Some(FieldCharType::Iso8583_ns),
            "xs" => Some(FieldCharType::Iso8583_xn),
            "a" => Some(FieldCharType::Iso8583_a),
            "an" => Some(FieldCharType::Iso8583_an),
            "ans" => Some(FieldCharType::Iso8583_ans),
            "ansb" => Some(FieldCharType::Iso8583_ansb),
            "anp" => Some(FieldCharType::Iso8583_anp),
            "b" => Some(FieldCharType::Iso8583_b),
            "z" => Some(FieldCharType::Iso8583_z),
            "bmps" => Some(FieldCharType::Iso8583_bmps),
            "mti" => Some(FieldCharType::Iso8583_mti),
            "undefined" => Some(FieldCharType::Iso8583_undefined),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            &FieldCharType::Iso8583_n => "n",
            &FieldCharType::Iso8583_ns => "ns",
            &FieldCharType::Iso8583_xn => "xs",
            &FieldCharType::Iso8583_a => "a",
            &FieldCharType::Iso8583_an => "an",
            &FieldCharType::Iso8583_ans => "ans",
            &FieldCharType::Iso8583_ansb => "ansb",
            &FieldCharType::Iso8583_anp => "anp",
            &FieldCharType::Iso8583_b => "b",
            &FieldCharType::Iso8583_z => "z",
            &FieldCharType::Iso8583_bmps => "bmps",
            &FieldCharType::Iso8583_mti => "mti",
            &FieldCharType::Iso8583_undefined => "undefined",
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FieldSizeType {
    Fixed,
    LlVar,
    LllVar,
    LlllVar,
    BitMap,
}

impl FieldSizeType {
    pub fn from_str(s: &str) -> Option<FieldSizeType> {
        let s_lower = s.to_lowercase();
        match s_lower.as_str() {
            "fixed" => Some(FieldSizeType::Fixed),
            "llvar" => Some(FieldSizeType::LlVar),
            "lllvar" => Some(FieldSizeType::LllVar),
            "llllvar" => Some(FieldSizeType::LlllVar),
            "bitmap" => Some(FieldSizeType::BitMap),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            &FieldSizeType::Fixed => "fixed",
            &FieldSizeType::LlVar => "llvar",
            &FieldSizeType::LllVar => "lllvar",
            &FieldSizeType::LlllVar => "llllvar",
            &FieldSizeType::BitMap => "bitmap",
        }
    }
}

/// `IsoField` defination
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct IsoField {
    pub label: String,
    pub label_id: String,
    pub char_type: FieldCharType,
    pub size_type: FieldSizeType,
    pub length: usize,
}

/// `IsoField` implementation
impl IsoField {
    pub fn new(
        label: &str,
        label_id: &str,
        char_type: FieldCharType,
        length: usize,
        size_type: FieldSizeType,
    ) -> IsoField {
        IsoField {
            label: String::from(label),
            label_id: String::from(label_id),
            char_type,
            length,
            size_type,
        }
    }
}

#[derive(Debug, Clone)]
pub enum IPMValue {
    u64(u64),
    String(String),
    Binary(Vec<u8>),
}

impl IPMValue {
    pub fn get_string(&self) -> String {
        match self {
            IPMValue::u64(num) => format!("{num}"),
            IPMValue::String(s) => s.to_owned(),
            IPMValue::Binary(b) => format!("{b:?}"),
        }
    }
}

impl serde::Serialize for IPMValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            IPMValue::u64(num) => serializer.serialize_u64(*num),
            IPMValue::String(s) => serializer.serialize_str(s),
            IPMValue::Binary(b) => serializer.serialize_bytes(b),
        }
    }
}

/// Field Payload is used inside IsoMsg to represent the field label, length and location
#[derive(Debug, Default)]
pub struct FieldPayload {
    pub iso_field_label: Option<String>,
    pub iso_field_label_id: String,
    pub char_type: FieldCharType,
    pub exist: bool,
    pub index: usize,
    pub len: usize,
    pub tag_len: usize, // the length of the tag e.g LLLVar = 3
}

impl FieldPayload {
    pub fn iso_field_value<'a>(&self, buffer: &'a [u8]) -> Vec<u8> {
        buffer[self.index + self.tag_len..self.index + self.len].to_vec()
    }

    pub fn get_ipm_value(&self, buffer: &[u8]) -> eyre::Result<IPMValue> {
        let bytes = self.iso_field_value(buffer);

        if self.char_type.get_str("content_type") == Some("string") {
            let utf8_string = String::from_utf8(bytes)?;

            Ok(IPMValue::String(utf8_string))
        } else if self.char_type.get_str("content_type") == Some("number") {
            let utf8_string = String::from_utf8(bytes)?;

            let num = utf8_string.parse::<u64>()?;

            Ok(IPMValue::u64(num))
        } else {
            Ok(IPMValue::Binary(bytes))
        }
    }
}
