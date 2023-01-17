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
use strum::{EnumProperty, IntoEnumIterator};
use std::collections::HashMap;
use std::fmt;
use std::num::FpCategory;

#[derive(Debug, Clone, Serialize)]
pub struct Message {
    label: String,
    value: Vec<u8>,
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

#[derive(Debug, Clone, Serialize)]
pub struct Group {
    //FIXME this could be a hashmap just like pds, and also named to DE
    pub messages: Vec<Message>,
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
            .map(|p| return (p.get_label(), p.utf8_value()))
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
                },
                None => None,
            };
        };

        Some(category)
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
            headers: vec![],
            trailers: vec![],
            first_presentments: vec![],
            second_presentments_full: vec![],
            second_presentments_partial: vec![],
            first_chargebacks: vec![],
            financial_details_addenda: vec![],
            retrieval_requests: vec![],
            retrieval_requests_acknowledgement: vec![],
            file_currency: vec![],
            financial_positions: vec![],
            settlements: vec![],
            message_exceptions: vec![],
            file_rejects: vec![],
            text_messages: vec![],
            currency_updates: vec![],
            fee_collections_customer: vec![],
            fee_collections_customer_return: vec![],
            fee_collections_customer_resubimission: vec![],
            fee_collections_customer_arbitration_return: vec![],
            fee_collections_clearing: vec![],
            unknowns: vec![],
        };

        parsed_file.assign_messages()?;

        Ok(parsed_file)
    }

    pub fn messages_count(self) -> HashMap<String, usize> {
        std::collections::HashMap::from([
            ("headers".to_string(), self.headers.len()),
            ("trailers".to_string(), self.trailers.len()),
            ("first_presentments".to_string(), self.first_presentments.len()),
            (
                "second_presentments_full".to_string(),
                self.second_presentments_full.len(),
            ),
            (
                "second_presentments_partial".to_string(),
                self.second_presentments_partial.len(),
            ),
            ("first_chargebacks".to_string(), self.first_chargebacks.len()),
            (
                "financial_details_addenda".to_string(),
                self.financial_details_addenda.len(),
            ),
            ("retrieval_requests".to_string(), self.retrieval_requests.len()),
            (
                "retrieval_requests_acknowledgement".to_string(),
                self.retrieval_requests_acknowledgement.len(),
            ),
            ("file_currency".to_string(), self.file_currency.len()),
            ("financial_positions".to_string(), self.financial_positions.len()),
            ("settlements".to_string(), self.settlements.len()),
            ("message_exceptions".to_string(), self.message_exceptions.len()),
            ("file_rejects".to_string(), self.file_rejects.len()),
            ("text_messages".to_string(), self.text_messages.len()),
            ("currency_updates".to_string(), self.currency_updates.len()),
            (
                "fee_collections_customer".to_string(),
                self.fee_collections_customer.len(),
            ),
            (
                "fee_collections_customer_return".to_string(),
                self.fee_collections_customer_return.len(),
            ),
            (
                "fee_collections_customer_resubimission".to_string(),
                self.fee_collections_customer_resubimission.len(),
            ),
            (
                "fee_collections_customer_arbitration_return".to_string(),
                self.fee_collections_customer_arbitration_return.len(),
            ),
            (
                "fee_collections_clearing".to_string(),
                self.fee_collections_clearing.len(),
            ),
            ("unknowns".to_string(), self.unknowns.len()),
        ])
    }
    pub fn messages_indexes(self) -> HashMap<String, Vec<usize>> {
        std::collections::HashMap::from([
            ("headers".to_string(), self.headers),
            ("trailers".to_string(), self.trailers),
            ("first_presentments".to_string(), self.first_presentments),
            (
                "second_presentments_full".to_string(),
                self.second_presentments_full,
            ),
            (
                "second_presentments_partial".to_string(),
                self.second_presentments_partial,
            ),
            ("first_chargebacks".to_string(), self.first_chargebacks),
            (
                "financial_details_addenda".to_string(),
                self.financial_details_addenda,
            ),
            ("retrieval_requests".to_string(), self.retrieval_requests),
            (
                "retrieval_requests_acknowledgement".to_string(),
                self.retrieval_requests_acknowledgement,
            ),
            ("file_currency".to_string(), self.file_currency),
            ("financial_positions".to_string(), self.financial_positions),
            ("settlements".to_string(), self.settlements),
            ("message_exceptions".to_string(), self.message_exceptions),
            ("file_rejects".to_string(), self.file_rejects),
            ("text_messages".to_string(), self.text_messages),
            ("currency_updates".to_string(), self.currency_updates),
            (
                "fee_collections_customer".to_string(),
                self.fee_collections_customer,
            ),
            (
                "fee_collections_customer_return".to_string(),
                self.fee_collections_customer_return,
            ),
            (
                "fee_collections_customer_resubimission".to_string(),
                self.fee_collections_customer_resubimission,
            ),
            (
                "fee_collections_customer_arbitration_return".to_string(),
                self.fee_collections_customer_arbitration_return,
            ),
            (
                "fee_collections_clearing".to_string(),
                self.fee_collections_clearing,
            ),
            ("unknowns".to_string(), self.unknowns),
        ])
    }

    fn assign_messages(&mut self) -> Result<()> {
        let iterable_groups = self.groups.iter().enumerate();
        for (index, group) in iterable_groups {
            match &group.category {
                Category::Header => self.headers.push(index),
                Category::Trailer => self.trailers.push(index),
                Category::FirstPresentment => self.first_presentments.push(index),
                Category::SecondPresentmentFull => self.second_presentments_full.push(index),
                Category::SecondPresentmentPartial => self.second_presentments_partial.push(index),
                Category::FirstChargeback => self.first_chargebacks.push(index),
                Category::FinancialDetailAddendum => self.financial_details_addenda.push(index),
                Category::RetrievalRequest => self.retrieval_requests.push(index),
                Category::RetrievalRequestAcknowledgement => {
                    self.retrieval_requests_acknowledgement.push(index)
                }
                Category::FileCurrency => self.file_currency.push(index),
                Category::FinancialPosition => self.financial_positions.push(index),
                Category::Settlement => self.settlements.push(index),
                Category::MessageException => self.message_exceptions.push(index),
                Category::FileReject => self.file_rejects.push(index),
                Category::TextMessage => self.text_messages.push(index),
                Category::CurrencyUpdate => self.currency_updates.push(index),
                Category::FeeCollectionCustomer => self.fee_collections_customer.push(index),
                Category::FeeCollectionCustomerReturn => {
                    self.fee_collections_customer_return.push(index)
                }
                Category::FeeCollectionCustomerResubmission => {
                    self.fee_collections_customer_resubimission.push(index)
                }
                Category::FeeCollectionCustomerArbitrationReturn => {
                    self.fee_collections_customer_arbitration_return.push(index)
                }
                Category::FeeCollectionClearing => self.fee_collections_clearing.push(index),
                Category::Unknown => self.unknowns.push(index),
            };
        }
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
                return Err(eyre!("duplicated message should not exist on iso8583",));
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
