// Copyright 2017 Rohit Joshi <rohit.c.joshi@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FieldCharType {
    Iso8583_n,
    Iso8583_ns,
    Iso8583_xn,
    ISO8583_a,
    Iso8583_an,
    Iso8583_ans,
    Iso8583_ansb,
    Iso8583_anp,
    Iso8583_b,
    ISO8583_z,
    Iso8583_bmps,
    Iso8583_mti,
}

impl FieldCharType {
    pub fn from_str(s: &str) -> Option<FieldCharType> {
        match s {
            "n" => Some(FieldCharType::Iso8583_n),
            "ns" => Some(FieldCharType::Iso8583_ns),
            "xs" => Some(FieldCharType::Iso8583_xn),
            "a" => Some(FieldCharType::ISO8583_a),
            "an" => Some(FieldCharType::Iso8583_an),
            "ans" => Some(FieldCharType::Iso8583_ans),
            "ansb" => Some(FieldCharType::Iso8583_ansb),
            "anp" => Some(FieldCharType::Iso8583_anp),
            "b" => Some(FieldCharType::Iso8583_b),
            "z" => Some(FieldCharType::ISO8583_z),
            "bmps" => Some(FieldCharType::Iso8583_bmps),
            "mti" => Some(FieldCharType::Iso8583_mti),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            &FieldCharType::Iso8583_n => "n",
            &FieldCharType::Iso8583_ns => "ns",
            &FieldCharType::Iso8583_xn => "xs",
            &FieldCharType::ISO8583_a => "a",
            &FieldCharType::Iso8583_an => "an",
            &FieldCharType::Iso8583_ans => "ans",
            &FieldCharType::Iso8583_ansb => "ansb",
            &FieldCharType::Iso8583_anp => "anp",
            &FieldCharType::Iso8583_b => "b",
            &FieldCharType::ISO8583_z => "z",
            &FieldCharType::Iso8583_bmps => "bmps",
            &FieldCharType::Iso8583_mti => "mti",
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
    pub char_type: FieldCharType,
    pub size_type: FieldSizeType,
    pub length: usize,
}

/// `IsoField` implementation
impl IsoField {
    pub fn new(
        label: &str,
        char_type: FieldCharType,
        length: usize,
        size_type: FieldSizeType,
    ) -> IsoField {
        IsoField {
            label: String::from(label),
            char_type: char_type,
            length: length,
            size_type: size_type,
        }
    }
}

/// Field Payload
#[derive(Debug, Default)]
pub struct FieldPayload {
    pub iso_field_label: Option<String>,
    pub exist: bool,
    pub index: usize,
    pub len: usize,
}

impl FieldPayload {
    pub fn iso_field_value<'a>(&self, buffer: &'a [u8]) -> Vec<u8> {
        buffer[self.index..self.index + self.len].to_vec()
    }
}
