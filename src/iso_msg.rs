// Copyright 2017 Rohit Joshi <rohit.c.joshi@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::iso_field::FieldCharType;
use crate::iso_field::FieldPayload;
use crate::iso_field::FieldSizeType;
use crate::iso_field::IsoField;
use crate::iso_specs::IsoSpecs;
use bit_array::BitArray;
use std::borrow::Cow;
use std::fmt;
use std::ops::Deref;
use std::str;
use typenum::U128;

/// `IsoMsg`
pub struct IsoMsg<'a, 'b> {
    payload: Cow<'a, [u8]>,
    iso_spec: &'b IsoSpecs,
    fields: Vec<FieldPayload>,
}

impl fmt::Debug for IsoMsg<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result: String = self
            .present_fields()
            .iter()
            .fold("".to_string(), |acc, &x| {
                format!(
                    "{} \n {:?} \n values: {:?} \n",
                    acc,
                    x.iso_field_label.clone().expect("cannot open field label"),
                    String::from_utf8_lossy(&x.iso_field_value(self.payload.deref()))
                )
            });
        write!(f, "{}", result)
    }
}

impl<'a, 'b> IsoMsg<'a, 'b> {
    pub fn new(iso_spec: &'b IsoSpecs, payload: &'a [u8]) -> IsoMsg<'a, 'b> {
        let fields = IsoMsg::from_byte_array(iso_spec, payload);

        IsoMsg {
            iso_spec: iso_spec,
            payload: Cow::Borrowed(payload),
            fields: fields,
        }
    }

    pub fn length(&self) -> usize {
        self.present_fields().iter().map(|&x| x.len).sum()
    }

    pub fn remove_field(&mut self, index: usize) -> Result<(), &str> {
        assert!(index < self.fields.len());
        assert!(index < self.iso_spec.specs.len());
        self.fields[index].exist = false;
        Ok(())
    }

    pub fn set_field(&mut self, index: usize, buffer: &[u8]) -> Result<(), &str> {
        trace!(
            "set_field: index:{}, buffer:{}",
            index,
            str::from_utf8(&buffer).unwrap()
        );
        assert!(index < self.fields.len());
        assert!(index < self.iso_spec.specs.len());
        assert!(buffer.len() <= self.iso_spec.specs[index].length);

        let len_prefix = self.get_field_length_prefix(index);
        let total_lenth = buffer.len() + len_prefix;
        let mut v = Vec::with_capacity(total_lenth);
        trace!(
            "buffer.len():{}, iso_spec.specs[index].length:{}",
            buffer.len(),
            self.iso_spec.specs[index].length
        );
        if len_prefix > 0 {
            v.extend_from_slice(format!("{:0w$}", buffer.len(), w = len_prefix).as_bytes());
        }
        v.extend_from_slice(buffer);

        trace!(
            "index:{}, set_extend_from_slice : v {}",
            index,
            str::from_utf8(&v).unwrap()
        );
        trace!("set_field: v.len:{}", v.len());
        self.fields[index].exist = true;
        Ok(())
    }

    pub fn get_field_length_prefix(&self, index: usize) -> usize {
        match self.iso_spec.specs[index].size_type {
            FieldSizeType::LlVar => 2,
            FieldSizeType::LllVar => 3,
            FieldSizeType::LlllVar => 4,
            _ => 0,
        }
    }

    pub fn present_fields(&self) -> Vec<&FieldPayload> {
        self.fields.iter().filter(|f| f.exist).collect()
    }

    #[deprecated(
        since = "0.1.1",
        note = "please use `FieldPayload#iso_field_value` instead"
    )]
    pub fn get_field(&self, index: usize, buffer: &mut [u8]) -> Result<usize, &str> {
        let res = self.get_field_raw(index, buffer);
        if res.is_err() {
            return Err(res.err().unwrap());
        }

        let (len, field_len_prefix) = res.unwrap();
        if field_len_prefix > 0 {
            let temp_buff = buffer[field_len_prefix..len].to_vec();
            buffer[0..len - field_len_prefix].copy_from_slice(&temp_buff[..]);
        }
        Ok(len - field_len_prefix)
    }

    fn get_field_raw(&self, index: usize, buffer: &mut [u8]) -> Result<(usize, usize), &str> {
        assert!(index < self.fields.len());
        let field = &self.fields[index];
        if !field.exist {
            return Err("Field not set");
        }

        if field.len == 0 {
            return Err("Field not set");
        }
        if buffer.len() >= field.len && self.payload.len() >= (field.len + field.index) {
            let len_prefix = self.get_field_length_prefix(index);
            buffer[0..field.len]
                .copy_from_slice(&self.payload[field.index..field.index + field.len]);
            Ok((field.len, len_prefix))
        } else {
            Err("Input buffer is smaller than field value")
        }
    }

    pub fn is_bit_set(input: u32, n: u8) -> bool {
        if n < 32 {
            input & (1 << n) != 0
        } else {
            false
        }
    }

    pub fn process_bitmap(bitmap_bytes: &[u8]) -> BitArray<u64, U128> {
        let bitmap = &bitmap_bytes[0..16]; //this is taking into account that there will always be a secundary bitmap
        let bit_array = BitArray::<u64, U128>::from_bytes(bitmap);

        bit_array
    }

    pub fn convert_u32_be(array: &[u8]) -> u32 {
        assert_eq!(array.len(), 4);
        (u32::from(array[0]) << 24)
            + (u32::from(array[1]) << 16)
            + (u32::from(array[2]) << 8)
            + (u32::from(array[3]) << 0)
    }

    pub fn convert_u32_le(array: &[u8]) -> u32 {
        assert_eq!(array.len(), 4);
        (u32::from(array[0]) << 0)
            + (u32::from(array[1]) << 8)
            + (u32::from(array[2]) << 16)
            + (u32::from(array[3]) << 24)
    }

    pub fn to_byte_array(&self, buffer: &mut [u8]) -> usize {
        let mut buffer_index = 0usize;
        let num_iteration: usize = (self.iso_spec.specs.len() - 1 + 63) / 128;
        let mut bit_arrays = Vec::<BitArray<u64, U128>>::with_capacity(num_iteration);
        for _ in 0..num_iteration {
            bit_arrays.push(BitArray::<u64, U128>::from_elem(false));
        }
        let mut bit_array_index = 0;
        let mut bit_index = 0;
        let mut bitmap_field_index = 0;

        let mut bitmap_found = false;

        //XXX seems here that the lib is expecting to parse the mti; the bitmap; the fields itself lol
        // TODO maybe implement a pointer solution, since we will get a lot of messages one after another
        for index in 0..self.fields.len() {
            bit_array_index = index / 128;

            let is_a_bitmap = !bitmap_found
                && (self.iso_spec.specs[index].char_type == FieldCharType::Iso8583_bmps);
            if is_a_bitmap {
                bitmap_field_index = index;
                bitmap_found = true;
                bit_index = buffer_index;
                let res = self.get_field_raw(index, &mut buffer[buffer_index..]);
                if res.is_ok() {
                    let (field_total_len, _) = res.unwrap();
                    buffer_index += field_total_len;
                }
            } else {
                let res = self.get_field_raw(index, &mut buffer[buffer_index..]);
                if res.is_ok() {
                    if bitmap_found {
                        bit_arrays[bit_array_index].set(index - bitmap_field_index, true);
                        trace!(
                            "index:{}, buffer[buffer_index..]:{}",
                            index,
                            str::from_utf8(&buffer[buffer_index..]).unwrap()
                        );
                    }
                    let (field_total_len, _) = res.unwrap();
                    buffer_index += field_total_len;
                }
            }
        }
        //override bitmap XXX but why??????
        let mut bitmap = String::with_capacity(bit_array_index * 16);
        for (i, bit_array_item) in bit_arrays.iter_mut().enumerate().take(bit_array_index) {
            if i == 0 && bit_array_item.len() > 64 {
                bit_array_item.set(0, true);
            }
            let bytes = bit_array_item.to_bytes();
            let mut byte_index = 0;

            while byte_index < bytes.len() {
                let ms_str = IsoMsg::convert_u32_be(&bytes[byte_index..byte_index + 4]);
                byte_index += 4;
                bitmap.push_str(&format!("{:08X}", ms_str));
            }
        }
        buffer[bit_index..bitmap.len() + bit_index]
            .copy_from_slice(&bitmap.as_bytes()[0..bitmap.len()]);
        buffer_index
    }

    pub fn get_field_length(iso_field: &IsoField, input_buffer: &[u8]) -> usize {
        match iso_field.size_type {
            FieldSizeType::Fixed | FieldSizeType::BitMap => iso_field.length,
            FieldSizeType::LlVar => {
                let str_digits = str::from_utf8(&input_buffer[0..2]).unwrap();
                usize::from_str_radix(str_digits, 10).unwrap() + 2
            }
            FieldSizeType::LllVar => {
                let str_digits = str::from_utf8(&input_buffer[0..3]).unwrap();
                usize::from_str_radix(str_digits, 10).unwrap() + 3
            }
            FieldSizeType::LlllVar => {
                let str_digits = str::from_utf8(&input_buffer[0..4]).unwrap();
                usize::from_str_radix(str_digits, 10).unwrap() + 4
            }
        }
    }

    //return a Result, create a debug param?
    pub fn from_byte_array(iso_spec: &IsoSpecs, input_buffer: &[u8]) -> Vec<FieldPayload> {
        let mut payload_index = 0usize;

        let bit_array = &IsoMsg::process_bitmap(&input_buffer[4..4 + 16]);

        let mut fields = Vec::with_capacity(iso_spec.specs.len());

        for iso_spec_index in 0..iso_spec.specs.len() {
            let iso_field: &IsoField = &iso_spec.specs[iso_spec_index];
            let is_a_mti_or_bitmap = iso_spec_index == 0 || iso_spec_index == 1;

            // i0 and i1 are bitmap and mti
            let field_exist = is_a_mti_or_bitmap || bit_array.get(iso_spec_index - 1).unwrap();

            let field = if field_exist {
                FieldPayload {
                    index: payload_index,
                    len: IsoMsg::get_field_length(iso_field, &input_buffer[payload_index..]),
                    exist: true,
                    iso_field_label: Some(iso_field.label.clone()), //TODO use the reference instead of cloning everytime
                }
            } else {
                FieldPayload::default()
            };

            if field.exist {
                //fill a array and use thiserror (?)
                println!("{:?},", field);
            }

            payload_index += field.len;
            fields.push(field)
        }
        fields
    }
}
