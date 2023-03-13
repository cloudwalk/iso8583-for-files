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
pub struct Field {
    pub label: String,
    pub value: Vec<u8>,
}

impl Field {
    pub fn utf8_value(&self) -> String {
        String::from_utf8_lossy(&self.value).to_string()
    }

    pub fn get_label(&self) -> String {
        self.label.to_string()
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match std::str::from_utf8(&self.value) {
            Ok(field_value) => write!(f, "{}: {}", self.label, field_value),
            Err(_) => write!(f, "{}: {:02X?}", self.label, self.value),
        }
    }
}

/// A Message represents a set of fields e.g Data elements or PDS
///
/// Usually a message represents something based on it's categories, for example a FirstPresentment
/// Although some messages rely on being chained, like a MessageException, linked to a FirstPresentment on a TT113 file
#[derive(Debug, Clone, Serialize)]
pub struct Message {
    pub mti: String,
    pub category: Category,
    pub primary_bitmap: [u8; 8],
    pub data_elements: HashMap<String, iso_field::IPMValue>,
    pub pds: HashMap<String, String>,
}

impl Message {
    /// Returns a HashMap [message.label => message.utf8_value] of all self.messages
    pub fn get_messages_hash(&self) -> Result<HashMap<String, String>> {
        let messages_hash: HashMap<String, String> = self
            .data_elements
            .clone()
            .into_iter()
            .map(|(k, v)| (k, v.get_string()))
            .collect();
        Ok(messages_hash)
    }

    fn get_category(mti: &str, ipm_function_code: &iso_field::IPMValue) -> Category {
        let mut category = Category::Unknown;

        let function_code = ipm_function_code.get_string();

        for spec_category in Category::iter() {
            if spec_category.get_str("mti") == Some(mti)
                && spec_category.get_str("function_code") == Some(&function_code) {
                category = spec_category;
            }
        };

        category
    }
}

#[derive(Clone, Serialize)]
pub struct Iso8583File {
    pub messages: Vec<Message>,
    pub categories_indexes: HashMap<String, Vec<usize>>,
}

impl fmt::Debug for Iso8583File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut formatted_messages: Vec<String> = vec![];

        for message in self.messages.iter() {
            formatted_messages.push(format!(" {:?}(de) => {:?}", message.category, message.data_elements));
            formatted_messages.push(format!(" {:?}(pds) => {:?}", message.category, message.pds));
        }

        let result: String = formatted_messages
            .iter()
            .fold("".to_string(), |acc, x| format!("{}\n{}", acc, x));

        write!(f, "{}", result)
    }
}

impl Iso8583File {
    fn new(messages: Vec<Message>) -> Result<Self> {
        let mut parsed_file = Iso8583File {
            messages,
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
        let mut search_messages_result: Vec<Message> = vec![];

        //TODO fix the performance for the search
        // since this loops inside each message and uses a hash to find a match, this is memory intensive
        for message in self.messages {
            let message_hash = message.get_messages_hash().expect("Unable to find message hash for message. Maybe the file is broken");
            for search_key in search.keys() {
                match message_hash.get(search_key) {
                    Some(msg) => {
                        if search.get(search_key).unwrap().contains(msg) {
                            search_messages_result.push(message);
                            break;
                        }
                    },
                    None => {}
                }
            }
        }

        let mut new_iso8583_files = Iso8583File {
            messages: search_messages_result,
            categories_indexes: HashMap::new(),
        };

        new_iso8583_files.assign_messages_categories().expect("Unable to assign categories messages");

        new_iso8583_files
    }


    fn assign_messages_categories(&mut self) -> Result<()> {
        let mut categories_indexes: HashMap<String, Vec<usize>> = HashMap::new();
        let iterable_messages = self.messages.iter().enumerate();
        for (index, message) in iterable_messages {
            let category_name = message.category.get_str("name").unwrap().to_string();
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

pub fn parse_file(payload: Vec<u8>) -> Result<Iso8583File> {
    //checks if file has rdw at head and blocks at tail

    let handle = iso_specs::IsoSpecs::new();

    let mut current_vec_index: usize = 0;
    let mut messages: Vec<Message> = vec![];

    let clean_payload = file_utils::deblock_and_remove_rdw_from(payload)?;

    while clean_payload.len() > (current_vec_index + 2) {
        let mut mti = "".to_owned();
        let mut primary_bitmap: [u8; 8] = Default::default();
        let mut data_elements: HashMap<String, iso_field::IPMValue> = HashMap::new();
        let mut pds: HashMap<String, String> = HashMap::new();
        let iso_msg = iso_msg::IsoMsg::new(&handle, &clean_payload[current_vec_index..]);
        for field in iso_msg.present_fields() {
            let value = field.iso_field_value(&clean_payload[current_vec_index..]);
            let field_id = field.iso_field_label_id.clone();
            let ipm_value = field.get_ipm_value(&clean_payload[current_vec_index..])?;

            // this is an additional security to avoid a stack level too deep or endless-loops
            if data_elements.contains_key(&field_id) {
                return Err(eyre!("duplicated field should not exist on iso8583 message",));
            }

            // Parse PDSs
            // FIXME: for now pds are only implemented for de48
            if field_id == "048" {
                pds = match pds::get_pds_values(&value)? {
                    Some(v) => v,
                    None => pds,
                }
            }

            // Parse MTI, bitmaps and DEs
            if field_id == "mti" {
                mti = ipm_value.get_string();
            } else if field_id == "bitmaps" {
                let (primary_bitmap_slice, secondary_bitmap_slice) = value.split_at(8);

                primary_bitmap = primary_bitmap_slice.try_into()?;

                data_elements.insert("001".to_owned(), iso_field::IPMValue::Binary(secondary_bitmap_slice.to_vec()));
            } else {
                data_elements.insert(field_id, ipm_value);
            }
        }

        let message = Message {
            category: Message::get_category(&mti, &data_elements["024"]),
            mti,
            primary_bitmap,
            data_elements,
            pds,
        };

        messages.append(&mut vec![message]);

        current_vec_index += iso_msg.length();
    }
    let iso8583_file = Iso8583File::new(messages)?;

    Ok(iso8583_file)
}
