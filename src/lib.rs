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
pub mod pds;

use crate::iso_specs::Category;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Serialize)]
pub struct Message {
    label: String,
    value: Vec<u8>,
}

impl Message {
    pub fn utf8_value(&self) -> String {
        String::from_utf8_lossy(&self.value).to_string()
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match std::str::from_utf8(&self.value) {
            Ok(message_value) => write!(f, "{}: {}", self.label, message_value),
            Err(_) => write!(f, "{}: {:02X?}", self.label, self.value),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Group {
    //FIXME this could be a hashmap just like pds, and also named to DE
    pub messages: Vec<Message>,
    //FIXME for now pds are only implemented for when de48 is present
    pub pds: HashMap<String, String>,
    pub category: Category,
}

impl Group {
    fn get_category(function_code_message: Message) -> Option<Category> {
        if function_code_message.label != "Function Code" {
            return None;
        }
        let category = match std::str::from_utf8(&function_code_message.value) {
            Ok("697") => Category::Header,
            Ok("200") => Category::FirstPresentment,
            Ok("688") => Category::Settlement,
            Ok("685") => Category::FinancialPosition,
            Ok("691") => Category::MessageException,
            Ok("699") => Category::FileReject,
            Ok("695") => Category::Trailer,
            _ => Category::Unknown,
        };

        Some(category)
    }
}

#[derive(Clone, Serialize)]
pub struct Iso8583File {
    pub groups: Vec<Group>,
    headers: Vec<usize>,
    first_presentments: Vec<usize>,
    settlements: Vec<usize>,
    financial_positions: Vec<usize>,
    message_exceptions: Vec<usize>,
    file_rejects: Vec<usize>,
    trailers: Vec<usize>,
    unknowns: Vec<usize>,
}

impl fmt::Debug for Iso8583File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut formatted_messages: Vec<String> = vec![];

        for gg in self.groups.iter() {
            let formatted_message = gg.messages.iter().fold("".to_string(), |acc, message| {
                format!("{} \n {:?} => {}", acc, gg.category, message)
            });

            formatted_messages.push(formatted_message);
            formatted_messages.push(format!(" {:?}(pds) => {:?}", gg.category, gg.pds));
        }

        let result: String = formatted_messages
            .iter()
            .fold("".to_string(), |acc, x| format!("{}\n{}", acc, x));

        write!(f, "{}", result)
    }
}

impl Iso8583File {
    fn new(groups: Vec<Group>) -> Result<Self, String> {
        let mut parsed_file = Iso8583File {
            groups,
            headers: vec![],
            first_presentments: vec![],
            settlements: vec![],
            financial_positions: vec![],
            message_exceptions: vec![],
            file_rejects: vec![],
            trailers: vec![],
            unknowns: vec![],
        };

        parsed_file.assign_messages()?;

        Ok(parsed_file)
    }

    pub fn messages_indexes(self) -> HashMap<String, Vec<usize>> {
        std::collections::HashMap::from([
            ("headers".to_string(), self.headers),
            ("first_presentments".to_string(), self.first_presentments),
            ("settlements".to_string(), self.settlements),
            ("financial_positions".to_string(), self.financial_positions),
            ("message_exceptions".to_string(), self.message_exceptions),
            ("file_rejects".to_string(), self.file_rejects),
            ("trailers".to_string(), self.trailers),
            ("unknowns".to_string(), self.unknowns),
        ])
    }

    fn assign_messages(&mut self) -> Result<(), String> {
        let mut iterable_groups = self.groups.iter().enumerate();
        for (index, group) in iterable_groups {
            match &group.category {
                Category::Header => self.headers.push(index),
                Category::FirstPresentment => self.first_presentments.push(index),
                Category::Settlement => self.settlements.push(index),
                Category::FinancialPosition => self.financial_positions.push(index),
                Category::MessageException => self.message_exceptions.push(index),
                Category::FileReject => self.file_rejects.push(index),
                Category::Trailer => self.trailers.push(index),
                Category::Unknown => self.unknowns.push(index),
            };
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
        let mut category = Category::Unknown;
        let mut pds: HashMap<String, String> = HashMap::new();
        let iso_msg = iso_msg::IsoMsg::new(&handle, &clean_payload[current_message_pointer..]);
        for field in iso_msg.present_fields() {
            let parsed_message = Message {
                label: field.iso_field_label.clone().unwrap(),
                value: field.iso_field_value(&clean_payload[current_message_pointer..]),
            };

            if let Some(matched_category) = Group::get_category(parsed_message.clone()) {
                category = matched_category
            }

            if let Some(matched_pds_values) = pds::get_pds_values(parsed_message.clone())? {
                pds = matched_pds_values
            }

            let mut parsed_message_vec = vec![parsed_message];
            messages_vec.append(&mut parsed_message_vec);

            if check_for_repeated_messages(&messages_vec) {
                return Err(format!("duplicated message should not exist on iso8583",));
            }
        }
        current_message_pointer += iso_msg.length();

        let message_group = Group {
            messages: messages_vec,
            pds: pds,
            category: category,
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
