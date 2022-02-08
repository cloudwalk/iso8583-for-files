#![feature(test)]
#![allow(non_camel_case_types)]
extern crate bit_array;
extern crate typenum;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

pub mod iso_field;
pub mod iso_msg;
pub mod iso_specs;

pub struct ParsedMessage<'a> {
    label: String,
    value: &'a [u8],
}

pub fn parse_file(payload: &[u8]) -> Vec<ParsedMessage> {
    let handle = iso_specs::IsoSpecs::new();

    let mut current_message_pointer: usize = 0;
    let mut messages_vec: Vec<ParsedMessage> = vec![];

    // +2 because of the index being 0 and we need to surpass it by 1
    while payload.len() > (current_message_pointer + 2) {
        let iso_msg = iso_msg::IsoMsg::new(&handle, &payload[current_message_pointer..]);
        for field in iso_msg.present_fields() {
            let mut parsed_message = vec![ParsedMessage {
                label: field.iso_field_label.clone().unwrap(),
                value: field.iso_field_value(&payload[current_message_pointer..]),
            }];
            messages_vec.append(&mut parsed_message);
        }
        current_message_pointer += iso_msg.length();
    }
    messages_vec
}
