use super::*;

use iso_field::FieldCharType;
use iso_field::FieldSizeType;
use iso_field::IsoField;

#[derive(Debug, Clone, Serialize)]
pub enum Category {
    Header,
    FirstPresentment,
    Settlement,
    FinancialPosition,
    MessageException,
    FileReject,
    Trailer,
    Unknown,
}

/// Auth spec defines the format of Iso8583 message
pub struct IsoSpecs {
    pub specs: Vec<IsoField>,
}

impl IsoSpecs {
    pub fn new() -> IsoSpecs {
        IsoSpecs {
            specs: IsoSpecs::define_specs(),
        }
    }

    pub fn define_specs() -> Vec<IsoField> {
        let h = vec![
            IsoField::new(
                "Message Type Indicator",
                FieldCharType::Iso8583_mti,
                4,
                FieldSizeType::Fixed,
            ), // Message Type Indicator
            IsoField::new(
                "Bitmap",
                FieldCharType::Iso8583_bmps,
                16,
                FieldSizeType::BitMap,
            ), // Bitmap
            IsoField::new(
                "Primary Account Number",
                FieldCharType::Iso8583_ns,
                19,
                FieldSizeType::LlVar,
            ), // Primary Account Number
            IsoField::new(
                "Processing Code",
                FieldCharType::Iso8583_ns,
                6,
                FieldSizeType::Fixed,
            ), // Processing Code
            IsoField::new(
                "Amount, Txn",
                FieldCharType::Iso8583_ns,
                12,
                FieldSizeType::Fixed,
            ), // Amount, Txn
            IsoField::new(
                "Amount, Reconciliation",
                FieldCharType::Iso8583_ns,
                12,
                FieldSizeType::Fixed,
            ), // Amount, Reconciliation
            IsoField::new(
                "Amount, Cardholder Billing",
                FieldCharType::Iso8583_ns,
                12,
                FieldSizeType::Fixed,
            ), // Amount, Cardholder Billing
            IsoField::new(
                "Date and Time, Transmission",
                FieldCharType::Iso8583_ns,
                10,
                FieldSizeType::Fixed,
            ), // Date and Time, Transmission
            IsoField::new(
                "Amount, Cardholder Billing Fee",
                FieldCharType::Iso8583_ns,
                8,
                FieldSizeType::Fixed,
            ), // Amount, Cardholder Billing Fee
            IsoField::new(
                "Conversion Rate, Reconciliation",
                FieldCharType::Iso8583_ns,
                8,
                FieldSizeType::Fixed,
            ), // Conversion Rate, Reconciliation
            IsoField::new(
                "Conversion Rate, Cardholder Billing",
                FieldCharType::Iso8583_ns,
                8,
                FieldSizeType::Fixed,
            ), // Conversion Rate, Cardholder Billing
            IsoField::new(
                "Systems Trace Audit Number",
                FieldCharType::Iso8583_ns,
                6,
                FieldSizeType::Fixed,
            ), // Systems Trace Audit Number
            IsoField::new(
                "Date and Time, Local Txn",
                FieldCharType::Iso8583_ns,
                12,
                FieldSizeType::Fixed,
            ), // Date and Time, Local Txn
            IsoField::new(
                "Date, Effective",
                FieldCharType::Iso8583_ns,
                4,
                FieldSizeType::Fixed,
            ), // Date, Effective
            IsoField::new(
                "Date, Expiration",
                FieldCharType::Iso8583_ns,
                4,
                FieldSizeType::Fixed,
            ), // Date, Expiration
            IsoField::new(
                "Date, Settlement",
                FieldCharType::Iso8583_ns,
                4,
                FieldSizeType::Fixed,
            ), // Date, Settlement
            IsoField::new(
                "Date, Conversion",
                FieldCharType::Iso8583_ns,
                4,
                FieldSizeType::Fixed,
            ), // Date, Conversion
            IsoField::new(
                "Date, Capture",
                FieldCharType::Iso8583_ns,
                4,
                FieldSizeType::Fixed,
            ), // Date, Capture
            IsoField::new(
                "Merchant Type",
                FieldCharType::Iso8583_ns,
                4,
                FieldSizeType::Fixed,
            ), // Merchant Type
            IsoField::new(
                "Country Code, Acquiring Inst",
                FieldCharType::Iso8583_ns,
                3,
                FieldSizeType::Fixed,
            ), // Country Code, Acquiring Inst
            IsoField::new(
                "Country Code, Primary Account Number",
                FieldCharType::Iso8583_ns,
                3,
                FieldSizeType::Fixed,
            ), // Country Code, Primary Account Number
            IsoField::new(
                "Country Code, Forwarding Inst",
                FieldCharType::Iso8583_ns,
                3,
                FieldSizeType::Fixed,
            ), // Country Code, Forwarding Inst
            IsoField::new(
                "Point of Service Data Code",
                FieldCharType::Iso8583_ns,
                12,
                FieldSizeType::Fixed,
            ), // Point of Service Data Code
            IsoField::new(
                "Card Sequence Number",
                FieldCharType::Iso8583_ns,
                3,
                FieldSizeType::Fixed,
            ), // Card Sequence Number
            IsoField::new(
                "Function Code",
                FieldCharType::Iso8583_ns,
                3,
                FieldSizeType::Fixed,
            ), // Function Code
            IsoField::new(
                "Message Reason Code",
                FieldCharType::Iso8583_ns,
                4,
                FieldSizeType::Fixed,
            ), // Message Reason Code
            IsoField::new(
                "Card Acceptor Business Code",
                FieldCharType::Iso8583_ns,
                4,
                FieldSizeType::Fixed,
            ), // Card Acceptor Business Code
            IsoField::new(
                "Approval Code Length",
                FieldCharType::Iso8583_ns,
                1,
                FieldSizeType::Fixed,
            ), // Approval Code Length
            IsoField::new(
                "Date, Reconciliation",
                FieldCharType::Iso8583_ns,
                9,
                FieldSizeType::Fixed,
            ), // Date, Reconciliation
            IsoField::new(
                "Reconciliation Indicator",
                FieldCharType::Iso8583_ns,
                9,
                FieldSizeType::Fixed,
            ), // Reconciliation Indicator
            IsoField::new(
                "Amounts, Original",
                FieldCharType::Iso8583_ns,
                24,
                FieldSizeType::Fixed,
            ), // Amounts, Original
            IsoField::new(
                "Acquirer Reference Data",
                FieldCharType::Iso8583_ans,
                99,
                FieldSizeType::LlVar,
            ), // Acquirer Reference Data
            IsoField::new(
                "Acquirer Inst Id Code",
                FieldCharType::Iso8583_ns,
                11,
                FieldSizeType::LlVar,
            ), // Acquirer Inst Id Code
            IsoField::new(
                "Forwarding Inst Id Code",
                FieldCharType::Iso8583_ns,
                11,
                FieldSizeType::LlVar,
            ), // Forwarding Inst Id Code
            IsoField::new(
                "Primary Account Number, Extended",
                FieldCharType::Iso8583_ns,
                28,
                FieldSizeType::LlVar,
            ), // Primary Account Number, Extended
            IsoField::new(
                "Track 2 Data",
                FieldCharType::ISO8583_z,
                37,
                FieldSizeType::LlVar,
            ), // Track 2 Data
            IsoField::new(
                "Track 3 Data",
                FieldCharType::ISO8583_z,
                104,
                FieldSizeType::LllVar,
            ), // Track 3 Data
            IsoField::new(
                "Retrieval Reference Number",
                FieldCharType::Iso8583_anp,
                12,
                FieldSizeType::Fixed,
            ), // Retrieval Reference Number
            IsoField::new(
                "Approval Code",
                FieldCharType::Iso8583_anp,
                6,
                FieldSizeType::Fixed,
            ), // Approval Code
            IsoField::new(
                "Action Code",
                FieldCharType::Iso8583_ns,
                2,
                FieldSizeType::Fixed,
            ), // Action Code
            IsoField::new(
                "Service Code",
                FieldCharType::Iso8583_ns,
                3,
                FieldSizeType::Fixed,
            ), // Service Code
            IsoField::new(
                "Card Acceptor Terminal Id",
                FieldCharType::Iso8583_ans,
                8,
                FieldSizeType::Fixed,
            ), // Card Acceptor Terminal Id
            IsoField::new(
                "Card Acceptor Id Code",
                FieldCharType::Iso8583_ans,
                15,
                FieldSizeType::Fixed,
            ), // Card Acceptor Id Code
            IsoField::new(
                "Card Acceptor Name/Location",
                FieldCharType::Iso8583_ans,
                99,
                FieldSizeType::LlVar,
            ), // Card Acceptor Name/Location
            IsoField::new(
                "dditional Response Data",
                FieldCharType::Iso8583_ans,
                99,
                FieldSizeType::LlVar,
            ), // Additional Response Data
            IsoField::new(
                "Track 1 Data",
                FieldCharType::Iso8583_ans,
                76,
                FieldSizeType::LlVar,
            ), // Track 1 Data
            IsoField::new(
                "Amounts, Fees",
                FieldCharType::Iso8583_ans,
                204,
                FieldSizeType::LllVar,
            ), // Amounts, Fees
            IsoField::new(
                "Additional Data - National",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Additional Data - National
            IsoField::new(
                "Additional Data - Private",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Additional Data - Private
            IsoField::new(
                "Currency Code, Txn",
                FieldCharType::Iso8583_an,
                3,
                FieldSizeType::Fixed,
            ), // Currency Code, Txn
            IsoField::new(
                "Currency Code, Reconciliation",
                FieldCharType::Iso8583_an,
                3,
                FieldSizeType::Fixed,
            ), // Currency Code, Reconciliation
            IsoField::new(
                "Currency Code, Cardholder Billing",
                FieldCharType::Iso8583_an,
                3,
                FieldSizeType::Fixed,
            ), // Currency Code, Cardholder Billing
            IsoField::new(
                "Personal Id Number (PIN) Data",
                FieldCharType::Iso8583_ans,
                16,
                FieldSizeType::Fixed,
            ), // Personal Id Number (PIN) Data
            IsoField::new(
                "Security Related Control Information",
                FieldCharType::Iso8583_ns,
                16,
                FieldSizeType::Fixed,
            ), // Security Related Control Information
            IsoField::new(
                "Amounts, Additional",
                FieldCharType::Iso8583_ans,
                120,
                FieldSizeType::LllVar,
            ), // Amounts, Additional
            IsoField::new(
                "IC Card System Related Data",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // IC Card System Related Data
            IsoField::new(
                "Original Data Elements",
                FieldCharType::Iso8583_ans,
                35,
                FieldSizeType::LlVar,
            ), // Original Data Elements
            IsoField::new(
                "Authorization Life Cycle Code",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Authorization Life Cycle Code
            IsoField::new(
                "Authorizing Agent Inst Id Cod",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Authorizing Agent Inst Id Code
            IsoField::new(
                "Transport Data",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Transport Data
            IsoField::new(
                "Reserved for National use",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for National use
            IsoField::new(
                "Reserved for National use",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for National use
            IsoField::new(
                "Reserved for Private use",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for Private use
            IsoField::new(
                "Reserved for Private use",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for Private use
            IsoField::new(
                "Message Authentication Code Field",
                FieldCharType::Iso8583_b,
                8,
                FieldSizeType::Fixed,
            ), // Message Authentication Code Field
            IsoField::new(
                "Reserved for ISO use",
                FieldCharType::Iso8583_b,
                8,
                FieldSizeType::Fixed,
            ), // Reserved for ISO use
            IsoField::new(
                "Reconciliation code , Original Fees",
                FieldCharType::Iso8583_ans,
                1,
                FieldSizeType::Fixed,
            ), //Reconciliation code , Original Fees
            IsoField::new(
                "Extended Payment Data",
                FieldCharType::Iso8583_ns,
                2,
                FieldSizeType::Fixed,
            ), // Extended Payment Data
            IsoField::new(
                "Country Code, Receiving Inst",
                FieldCharType::Iso8583_ns,
                3,
                FieldSizeType::Fixed,
            ), // Country Code, Receiving Inst
            IsoField::new(
                "Country Code, Settlement Inst",
                FieldCharType::Iso8583_ns,
                3,
                FieldSizeType::Fixed,
            ), // Country Code, Settlement Inst
            IsoField::new(
                "Network Management Information Code",
                FieldCharType::Iso8583_ns,
                3,
                FieldSizeType::Fixed,
            ), // Network Management Information Code
            IsoField::new(
                "Message Number",
                FieldCharType::Iso8583_ns,
                8,
                FieldSizeType::Fixed,
            ), // Message Number
            IsoField::new(
                "Data Record",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Data Record
            IsoField::new(
                "Date, Action",
                FieldCharType::Iso8583_ns,
                6,
                FieldSizeType::Fixed,
            ), // Date, Action
            IsoField::new(
                "Credits, Number",
                FieldCharType::Iso8583_ns,
                10,
                FieldSizeType::Fixed,
            ), // Credits, Number
            IsoField::new(
                "Credits, Reversal Number",
                FieldCharType::Iso8583_ns,
                10,
                FieldSizeType::Fixed,
            ), // Credits, Reversal Number
            IsoField::new(
                "Debits, Number",
                FieldCharType::Iso8583_ns,
                10,
                FieldSizeType::Fixed,
            ), // Debits, Number
            IsoField::new(
                "Debits, Reversal Number",
                FieldCharType::Iso8583_ns,
                10,
                FieldSizeType::Fixed,
            ), // Debits, Reversal Number
            IsoField::new(
                "Transfer, Number",
                FieldCharType::Iso8583_ns,
                10,
                FieldSizeType::Fixed,
            ), // Transfer, Number
            IsoField::new(
                "Transfer, Reversal Number",
                FieldCharType::Iso8583_ns,
                10,
                FieldSizeType::Fixed,
            ), // Transfer, Reversal Number
            IsoField::new(
                "Inquiries, Number",
                FieldCharType::Iso8583_ns,
                10,
                FieldSizeType::Fixed,
            ), // Inquiries, Number
            IsoField::new(
                "Authorizations, Number",
                FieldCharType::Iso8583_ns,
                10,
                FieldSizeType::Fixed,
            ), // Authorizations, Number
            IsoField::new(
                "Inquiries, Reversal Number",
                FieldCharType::Iso8583_ns,
                10,
                FieldSizeType::Fixed,
            ), // Inquiries, Reversal Number
            IsoField::new(
                "Payments, Number",
                FieldCharType::Iso8583_ns,
                10,
                FieldSizeType::Fixed,
            ), // Payments, Number
            IsoField::new(
                "Payments, Reversal Number",
                FieldCharType::Iso8583_ns,
                10,
                FieldSizeType::Fixed,
            ), // Payments, Reversal Number
            IsoField::new(
                "Fee Collections, Number",
                FieldCharType::Iso8583_ns,
                10,
                FieldSizeType::Fixed,
            ), // Fee Collections, Number
            IsoField::new(
                "Credits, Amount",
                FieldCharType::Iso8583_ns,
                16,
                FieldSizeType::Fixed,
            ), // Credits, Amount
            IsoField::new(
                "Credits, Reversal Amount",
                FieldCharType::Iso8583_ns,
                16,
                FieldSizeType::Fixed,
            ), // Credits, Reversal Amount
            IsoField::new(
                "Debits, Amount",
                FieldCharType::Iso8583_ns,
                16,
                FieldSizeType::Fixed,
            ), // Debits, Amount
            IsoField::new(
                "Debits, Reversal Amount",
                FieldCharType::Iso8583_ns,
                16,
                FieldSizeType::Fixed,
            ), // Debits, Reversal Amount
            IsoField::new(
                "Authorizations, Reversal Number",
                FieldCharType::Iso8583_ns,
                42,
                FieldSizeType::Fixed,
            ), // Authorizations, Reversal Number
            IsoField::new(
                "Country Code, Txn Destination Inst",
                FieldCharType::Iso8583_ns,
                3,
                FieldSizeType::Fixed,
            ), // Country Code, Txn Destination Inst
            IsoField::new(
                "Country Code, Txn Originator Inst",
                FieldCharType::Iso8583_ns,
                3,
                FieldSizeType::Fixed,
            ), // Country Code, Txn Originator Inst
            IsoField::new(
                "Txn Destination Inst Id Code",
                FieldCharType::Iso8583_ns,
                11,
                FieldSizeType::LlVar,
            ), // Txn Destination Inst Id Code
            IsoField::new(
                "Txn Originator Inst Id Code",
                FieldCharType::Iso8583_ns,
                11,
                FieldSizeType::LlVar,
            ), // Txn Originator Inst Id Code
            IsoField::new(
                "Card Issuer Reference Data",
                FieldCharType::Iso8583_ans,
                42,
                FieldSizeType::LlVar,
            ), // Card Issuer Reference Data
            IsoField::new(
                "Key Management Data",
                FieldCharType::Iso8583_b,
                999,
                FieldSizeType::LllVar,
            ), // Key Management Data
            IsoField::new(
                "Amount, Net Reconciliation",
                FieldCharType::Iso8583_xn,
                17,
                FieldSizeType::Fixed,
            ), // Amount, Net Reconciliation
            IsoField::new(
                "Payee",
                FieldCharType::Iso8583_ans,
                25,
                FieldSizeType::Fixed,
            ), // Payee
            IsoField::new(
                "Settlement Inst Id Code",
                FieldCharType::Iso8583_an,
                11,
                FieldSizeType::LlVar,
            ), // Settlement Inst Id Code
            IsoField::new(
                "Receiving Inst Id Code",
                FieldCharType::Iso8583_ns,
                11,
                FieldSizeType::LlVar,
            ), // Receiving Inst Id Code
            IsoField::new(
                "File Name",
                FieldCharType::Iso8583_ans,
                17,
                FieldSizeType::LlVar,
            ), // File Name
            IsoField::new(
                "Account Id 1",
                FieldCharType::Iso8583_ans,
                28,
                FieldSizeType::LlVar,
            ), // Account Id 1
            IsoField::new(
                "Account Id 2",
                FieldCharType::Iso8583_ans,
                28,
                FieldSizeType::LlVar,
            ), // Account Id 2
            IsoField::new(
                "Txn Description",
                FieldCharType::Iso8583_ans,
                255,
                FieldSizeType::LllVar,
            ), // Txn Description
            IsoField::new(
                "Credits, Chargeback Amount",
                FieldCharType::Iso8583_ns,
                16,
                FieldSizeType::Fixed,
            ), // Credits, Chargeback Amount
            IsoField::new(
                "Debits, Chargeback Amount",
                FieldCharType::Iso8583_ns,
                16,
                FieldSizeType::Fixed,
            ), // Debits, Chargeback Amount
            IsoField::new(
                "Credits, Chargeback Number",
                FieldCharType::Iso8583_ns,
                10,
                FieldSizeType::Fixed,
            ), // Credits, Chargeback Number
            IsoField::new(
                "Debits, Chargeback Number",
                FieldCharType::Iso8583_ns,
                10,
                FieldSizeType::Fixed,
            ), // Debits, Chargeback Number
            IsoField::new(
                "Credits, Fee Amounts",
                FieldCharType::Iso8583_ans,
                84,
                FieldSizeType::LlVar,
            ), // Credits, Fee Amounts
            IsoField::new(
                "Debits, Fee Amounts",
                FieldCharType::Iso8583_ans,
                84,
                FieldSizeType::LlVar,
            ), // Debits, Fee Amounts
            IsoField::new(
                "Reserved for ISO use",
                FieldCharType::Iso8583_ns,
                12,
                FieldSizeType::Fixed,
            ), // Reserved for ISO use
            IsoField::new(
                "Reserved for ISO use",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for ISO use
            IsoField::new(
                "Reserved for ISO use",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for ISO use
            IsoField::new(
                "Reserved for ISO use",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for ISO use
            IsoField::new(
                "Reserved for ISO use",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for ISO use
            IsoField::new(
                "Reserved for National use",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for National use
            IsoField::new(
                "Reserved for National use",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for National use
            IsoField::new(
                "Reserved for National use",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for National use
            IsoField::new(
                "Reserved for National use",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for National use
            IsoField::new(
                "Reserved for National use",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for National use
            IsoField::new(
                "Reserved for National use",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for National use
            IsoField::new(
                "Reserved for National use",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for National use
            IsoField::new(
                "Reserved for Private use",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for Private use
            IsoField::new(
                "Reserved for Private use",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for Private use
            IsoField::new(
                "Reserved for Private use",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for Private use
            IsoField::new(
                "Reserved for Private use",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for Private use
            IsoField::new(
                "Reserved for Private use",
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for Private use
            IsoField::new(
                "Message Authentication Code Field",
                FieldCharType::Iso8583_b,
                8,
                FieldSizeType::Fixed,
            ), // Message Authentication Code Field
        ];
        return h;
    }
}
