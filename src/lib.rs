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
use eyre::{eyre, Result};
use std::collections::HashMap;
use std::fmt;
use strum::{EnumProperty, IntoEnumIterator};

#[derive(Debug, Clone, Serialize)]
pub struct Message {
    pub label: String,
    pub value: Vec<u8>,
    pub de: String,
    pub ipm_value: iso_field::IPMValue,
}

impl Message {
    pub fn utf8_value(&self) -> String {
        String::from_utf8_lossy(&self.value).to_string()
    }

    pub fn get_label(&self) -> String {
        self.label.to_string()
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

/// A Group represents a set of messages e.g Data elements or PDS
/// Data elements (DE) are stored within `messages`
///
/// Usually a group represents something based on it's categories, for example a FirstPresentment
/// Although some messages rely on being chained, like a MessageException, linked to a FirstPresentment on a TT113 file
#[derive(Debug, Clone, Serialize)]
pub struct Group {
    //FIXME this could be a hashmap just like pds, and also named to DE (deprecated by data_elements field)
    pub messages: Vec<Message>,
    pub data_elements: HashMap<String, iso_field::IPMValue>,
    //FIXME for now pds are only implemented for when de48 is present
    pub pds: HashMap<String, String>,
    pub category: Category,
}

impl Group {
    /// Returns a HashMap [message.label => message.utf8_value] of all self.messages
    pub fn get_messages_hash(&self) -> Result<HashMap<String, String>> {
        let messages_hash: HashMap<String, String> = self
            .messages
            .iter()
            .map(|p| (p.get_label(), p.utf8_value()))
            .collect();
        Ok(messages_hash)
    }

    fn get_category(function_code_message: Message) -> Option<Category> {
        if function_code_message.label != "Function Code" {
            return None;
        }

        //XXX move this to the initialization
        let mut category_hash = HashMap::new();
        for category in Category::iter() {
            match category.get_str("function_code") {
                Some(category_function_code) => {
                    category_hash.insert(category_function_code, category)
                }
                None => None,
            };
        }

        let function_code_message_value =
            std::str::from_utf8(&function_code_message.value).unwrap();

        match category_hash.get(function_code_message_value) {
            Some(category) => Some(category.to_owned()),
            None => Some(Category::Unknown),
        }
    }
}

#[derive(Clone, Serialize)]
pub struct Iso8583File {
    pub groups: Vec<Group>,
    pub categories_indexes: HashMap<String, Vec<usize>>,
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
    fn new(groups: Vec<Group>) -> Result<Self> {
        let mut parsed_file = Iso8583File {
            groups,
            categories_indexes: HashMap::new(),
        };

        parsed_file.assign_messages_categories()?;

        Ok(parsed_file)
    }

    pub fn messages_count(self) -> HashMap<String, usize> {
        let mut messages_count = HashMap::new();
        for (category_name, indexes) in self.categories_indexes {
            messages_count.insert(category_name, indexes.len());
        }
        messages_count
    }

    /// Searches for a set of iso8583 keys and values in order to create a cloned structure
    /// containing only the searched fields.
    /// This process is memory intensive, due to the imutable nature of this method
    pub fn search(self, search: HashMap<String, Vec<String>>) -> Iso8583File {
        let mut search_groups_result: Vec<Group> = vec![];

        //TODO fix the performance for the search
        // since this loops inside each group and uses a hash to find a match, this is memory intensive
        for group in self.groups {
            let group_messages_hash = group.get_messages_hash().expect("Unable to find message hash for group. Maybe the file is broken");
            for search_key in search.keys() {
                match group_messages_hash.get(search_key) {
                    Some(message) => {
                        if search.get(search_key).unwrap().contains(message) {
                            search_groups_result.push(group);
                            break;
                        }
                    },
                    None => {}
                }
            }
        }

        let mut new_iso8583_files = Iso8583File {
            groups: search_groups_result,
            categories_indexes: HashMap::new(),
        };

        new_iso8583_files.assign_messages_categories().expect("Unable to assign categories messages");

        new_iso8583_files
    }


    fn assign_messages_categories(&mut self) -> Result<()> {
        let mut categories_indexes: HashMap<String, Vec<usize>> = HashMap::new();
        let iterable_groups = self.groups.iter().enumerate();
        for (index, group) in iterable_groups {
            let category_name = group.category.get_str("name").unwrap().to_string();
            let category_index_entry = categories_indexes.entry(category_name).or_default();
            category_index_entry.push(index);
        }
        self.categories_indexes = categories_indexes;
        Ok(())
    }
}

pub fn read_and_deblock_file<'a>(file_name: &str) -> Result<Vec<u8>> {
    let file = file_utils::read_file(file_name);
    let file_contents_base64 = file_utils::deblock_and_remove_rdw_from(file)?;
    Ok(file_contents_base64)
}

pub fn parse_file<'a>(payload: Vec<u8>) -> Result<Iso8583File> {
    //checks if file has rdw at head and blocks at tail

    let handle = iso_specs::IsoSpecs::new();

    let mut current_message_pointer: usize = 0;
    let mut message_groups: Vec<Group> = vec![];

    let clean_payload = file_utils::deblock_and_remove_rdw_from(payload)?;

    while clean_payload.len() > (current_message_pointer + 2) {
        let mut messages_vec: Vec<Message> = vec![];
        let mut data_elements: HashMap<String, iso_field::IPMValue> = HashMap::new();
        let mut category = Category::Unknown;
        let mut pds: HashMap<String, String> = HashMap::new();
        let iso_msg = iso_msg::IsoMsg::new(&handle, &clean_payload[current_message_pointer..]);
        for field in iso_msg.present_fields() {
            let de = field.iso_field_de.clone();
            let ipm_value = field.get_ipm_value(&clean_payload[current_message_pointer..]);

            data_elements.insert(de.clone(), ipm_value.clone());

            let parsed_message = Message {
                label: field.iso_field_label.clone().unwrap(),
                value: field.iso_field_value(&clean_payload[current_message_pointer..]),
                de,
                ipm_value,
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
                return Err(eyre!("duplicated message should not exist on iso8583",));
            }
        }
        current_message_pointer += iso_msg.length();

        let message_group = Group {
            messages: messages_vec,
            data_elements,
            pds,
            category,
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
