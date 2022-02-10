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

#[derive(Debug, Clone, Serialize)]
pub struct Message<'a> {
    label: String,
    value: &'a [u8],
}

#[derive(Debug, Clone, Serialize)]
pub struct Group<'a> {
    messages: Vec<Message<'a>>,
}

pub fn parse_file(payload: &[u8]) -> Result<Vec<Group>, String> {
    let handle = iso_specs::IsoSpecs::new();

    let mut current_message_pointer: usize = 0;
    let mut message_groups: Vec<Group> = vec![];

    // +2 because of the index being 0 and we need to surpass it by 1
    while payload.len() > (current_message_pointer + 2) {
        let mut messages_vec: Vec<Message> = vec![];
        let iso_msg = iso_msg::IsoMsg::new(&handle, &payload[current_message_pointer..]);
        for field in iso_msg.present_fields() {
            let parsed_message = Message {
                label: field.iso_field_label.clone().unwrap(),
                value: field.iso_field_value(&payload[current_message_pointer..]),
            };

            let mut parsed_message_vec = vec![parsed_message];
            messages_vec.append(&mut parsed_message_vec);

            if check_for_repeated_messages(&messages_vec) {
                return Err(format!("duplicated message should not exist on iso8583",));
            }
        }
        current_message_pointer += iso_msg.length();

        let message_group = Group {
            messages: messages_vec,
        };
        let mut new_message_group_vec = vec![message_group];
        message_groups.append(&mut new_message_group_vec);
    }
    Ok(message_groups)
}

fn check_for_repeated_messages(messages_vec: &Vec<Message>) -> bool {
    let max_repeated_messages_count = 4;
    let scan_messages_count = 4;

    let last_parsed_value = match messages_vec.last() {
        Some(x) => x.value,
        None => &[],
    };

    let repeated_messages_count = messages_vec
        .iter()
        .rev()
        .take(scan_messages_count)
        .map(|x| x.value)
        .filter(|v| *v == last_parsed_value)
        .count();

    max_repeated_messages_count == repeated_messages_count
}
