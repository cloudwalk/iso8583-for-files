#![allow(non_camel_case_types)]
extern crate bit_array;
extern crate typenum;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

pub mod file_utils;
pub mod iso_field;
pub mod iso_msg;
pub mod iso_specs;

#[derive(Debug, Clone, Serialize)]
pub struct Message {
    label: String,
    value: Vec<u8>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Group {
    pub messages: Vec<Message>,
}

//TODO dry me up
impl Group {
    fn is_header(self) -> bool {
        if let Some(Message { label, value }) = self.messages.get(2) {
            if label == "Function Code" && String::from_utf8_lossy(value) == "697" {
                return true;
            }
        }
        false
    }

    fn is_first_presentment(self) -> bool {
        if let Some(Message { label, value }) = self.messages.get(2) {
            if label == "Function Code" && String::from_utf8_lossy(value) == "200" {
                return true;
            }
        }
        false
    }

    fn is_settlement(self) -> bool {
        if let Some(Message { label, value }) = self.messages.get(2) {
            if label == "Function Code" && String::from_utf8_lossy(value) == "688" {
                return true;
            }
        }
        false
    }

    fn is_footer(self) -> bool {
        if let Some(Message { label, value }) = self.messages.get(2) {
            if label == "Function Code" && String::from_utf8_lossy(value) == "695" {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Iso8583File {
    pub groups: Vec<Group>,
    pub headers: Vec<usize>,
}

impl Iso8583File {
    fn new(groups: Vec<Group>) -> Result<Self, String> {
        let mut parsed_file = Iso8583File {
            groups,
            headers: vec![],
        };

        parsed_file.assign_messages()?;

        Ok(parsed_file)
    }

    fn assign_messages(&mut self) -> Result<(), String> {
        let mut iterable_group = self.groups.iter();
        while let Some(position) = iterable_group.rposition(|group| group.clone().is_header()) {
            self.headers.push(position);
        }
        Ok(())
    }
}

pub fn parse_file<'a>(payload: Vec<u8>) -> Result<Iso8583File, String> {
    //checks if file has rdw at head and blocks at tail

    let handle = iso_specs::IsoSpecs::new();

    let mut current_message_pointer: usize = 0;
    let mut message_groups: Vec<Group> = vec![];

    let clean_payload = file_utils::deblock_and_remove_rdw_from(payload);

    // +2 because of the index being 0 and we need to surpass it by 1
    while clean_payload.len() > (current_message_pointer + 2) {
        let mut messages_vec: Vec<Message> = vec![];
        let iso_msg = iso_msg::IsoMsg::new(&handle, &clean_payload[current_message_pointer..]);
        for field in iso_msg.present_fields() {
            let parsed_message = Message {
                label: field.iso_field_label.clone().unwrap(),
                value: field.iso_field_value(&clean_payload[current_message_pointer..]),
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
    let iso8583_file = Iso8583File::new(message_groups)?;

    Ok(iso8583_file)
}

// this is an additional security to avoid a stack level too deep or endless-loops
fn check_for_repeated_messages(messages_vec: &Vec<Message>) -> bool {
    let max_repeated_messages_count = 4;
    let scan_messages_count = 4;

    let last_parsed_value = match messages_vec.last() {
        Some(x) => x.value.clone(),
        None => vec![],
    };

    let repeated_messages_count = messages_vec
        .iter()
        .rev()
        .take(scan_messages_count)
        .map(|x| &x.value)
        .filter(|v| *v == &last_parsed_value)
        .count();

    max_repeated_messages_count <= repeated_messages_count
}
