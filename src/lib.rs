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
use eyre::{eyre, Result, WrapErr};
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
        let category = match std::str::from_utf8(&function_code_message.value) {
            // File layout messages
            Ok("697") => Category::Header,
            Ok("695") => Category::Trailer,
            // Financial messages
            Ok("200") => Category::FirstPresentment,
            Ok("205") => Category::SecondPresentmentFull,
            Ok("282") => Category::SecondPresentmentPartial,
            Ok("450") => Category::FirstChargebackFull,
            Ok("453") => Category::FirstChargebackPartial,
            Ok("696") => Category::FinancialDetailAddendum,
            // Retrieval messages
            Ok("603") => Category::RetrievalRequest,
            Ok("605") => Category::RetrievalRequestAcknowledgement,
            // Reconciliation messages
            Ok("680") => Category::FileCurrency,
            Ok("685") => Category::FinancialPosition,
            Ok("688") => Category::Settlement,
            // Administrative messages
            Ok("691") => Category::MessageException,
            Ok("699") => Category::FileReject,
            Ok("693") => Category::TextMessage,
            Ok("640") => Category::CurrencyUpdate,
            // Fee collection messages
            Ok("700") => Category::FeeCollectionCustomer,
            Ok("780") => Category::FeeCollectionCustomerReturn,
            Ok("781") => Category::FeeCollectionCustomerResubmission,
            Ok("782") => Category::FeeCollectionCustomerArbitrationReturn,
            Ok("783") => Category::FeeCollectionClearing,
            // Unknown messages
            _ => Category::Unknown,
        };

        Some(category)
    }
}

#[derive(Clone, Serialize)]
pub struct Iso8583File {
    pub groups: Vec<Group>,
    headers: Vec<usize>,
    trailers: Vec<usize>,
    first_presentments: Vec<usize>,
    second_presentments_full: Vec<usize>,
    second_presentments_partial: Vec<usize>,
    first_chargebacks_full: Vec<usize>,
    first_chargebacks_partial: Vec<usize>,
    financial_details_addenda: Vec<usize>,
    retrieval_requests: Vec<usize>,
    retrieval_requests_acknowledgement: Vec<usize>,
    file_currency: Vec<usize>,
    financial_positions: Vec<usize>,
    settlements: Vec<usize>,
    message_exceptions: Vec<usize>,
    file_rejects: Vec<usize>,
    text_messages: Vec<usize>,
    currency_updates: Vec<usize>,
    fee_collections_customer: Vec<usize>,
    fee_collections_customer_return: Vec<usize>,
    fee_collections_customer_resubimission: Vec<usize>,
    fee_collections_customer_arbitration_return: Vec<usize>,
    fee_collections_clearing: Vec<usize>,
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
    fn new(groups: Vec<Group>) -> Result<Self> {
        let mut parsed_file = Iso8583File {
            groups,
            headers: vec![],
            trailers: vec![],
            first_presentments: vec![],
            second_presentments_full: vec![],
            second_presentments_partial: vec![],
            first_chargebacks_full: vec![],
            first_chargebacks_partial: vec![],
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

    pub fn messages_indexes(self) -> HashMap<String, Vec<usize>> {
        std::collections::HashMap::from(
            [
                ("headers", self.headers),
                ("trailers", self.trailers),
                ("first_presentments", self.first_presentments),
                ("second_presentments_full", self.second_presentments_full),
                (
                    "second_presentments_partial",
                    self.second_presentments_partial,
                ),
                ("first_chargebacks_full", self.first_chargebacks_full),
                ("first_chargebacks_partial", self.first_chargebacks_partial),
                ("financial_details_addenda", self.financial_details_addenda),
                ("retrieval_requests", self.retrieval_requests),
                (
                    "retrieval_requests_acknowledgement",
                    self.retrieval_requests_acknowledgement,
                ),
                ("file_currency", self.file_currency),
                ("financial_positions", self.financial_positions),
                ("settlements", self.settlements),
                ("message_exceptions", self.message_exceptions),
                ("file_rejects", self.file_rejects),
                ("text_messages", self.text_messages),
                ("currency_updates", self.currency_updates),
                ("fee_collections_customer", self.fee_collections_customer),
                (
                    "fee_collections_customer_return",
                    self.fee_collections_customer_return,
                ),
                (
                    "fee_collections_customer_resubimission",
                    self.fee_collections_customer_resubimission,
                ),
                (
                    "fee_collections_customer_arbitration_return",
                    self.fee_collections_customer_arbitration_return,
                ),
                ("fee_collections_clearing", self.fee_collections_clearing),
                ("unknowns", self.unknowns),
            ]
            .map(|v| (v.0.to_string(), v.1)),
        )
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
                Category::FirstChargebackFull => self.first_chargebacks_full.push(index),
                Category::FirstChargebackPartial => self.first_chargebacks_partial.push(index),
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
