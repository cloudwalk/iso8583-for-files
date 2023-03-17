use super::*;
use iso_field::FieldCharType;
use iso_field::FieldSizeType;
use iso_field::IsoField;
use strum_macros;

#[derive(Debug, Clone, Serialize, strum_macros::EnumProperty, strum_macros::EnumIter)]
pub enum Category {
    // File layout messages
    #[strum(props(mti = "1644", function_code = "697", name = "headers", kind = "file_layout_messages"))]
    Header,

    #[strum(props(
        mti = "1644",
        function_code = "695",
        name = "trailers",
        kind = "file_layout_messages"
    ))]
    Trailer,

    // Financial messages
    #[strum(props(
        mti = "1240",
        function_code = "200",
        name = "first_presentments",
        kind = "financial_messages"
    ))]
    FirstPresentment,

    #[strum(props(
        mti = "1240",
        function_code = "205",
        name = "second_presentments_full",
        kind = "financial_messages"
    ))]
    SecondPresentmentFull,

    #[strum(props(
        mti = "1240",
        function_code = "282",
        name = "second_presentments_partial",
        kind = "financial_messages"
    ))]
    SecondPresentmentPartial,

    #[strum(props(
        mti = "1442",
        function_code = "450",
        name = "first_chargebacks",
        kind = "financial_messages"
    ))]
    FirstChargeback,

    #[strum(props(
        mti = "1644",
        function_code = "696",
        name = "financial_details_addenda",
        kind = "financial_messages"
    ))]
    FinancialDetailAddendum,

    // Retrieval messages
    #[strum(props(
        mti = "1644",
        function_code = "603",
        name = "retrieval_requests",
        kind = "retrieval_messages"
    ))]
    RetrievalRequest,

    #[strum(props(
        mti = "1644",
        function_code = "605",
        name = "retrieval_requests_acknowledgement",
        kind = "retrieval_messages"
    ))]
    RetrievalRequestAcknowledgement,

    // Reconciliation messages
    #[strum(props(
        mti = "1644",
        function_code = "685",
        name = "financial_positions",
        kind = "reconciliation_messages"
    ))]
    FinancialPosition,

    #[strum(props(
        mti = "1644",
        function_code = "688",
        name = "settlements",
        kind = "reconciliation_messages"
    ))]
    Settlement,

    #[strum(props(
        mti = "1644",
        function_code = "680",
        name = "file_currencies",
        kind = "reconciliation_messages"
    ))]
    FileCurrency,

    // Administrative messages
    #[strum(props(
        mti = "1644",
        function_code = "691",
        name = "message_exceptions",
        kind = "administrative_messages"
    ))]
    MessageException,

    #[strum(props(
        mti = "1644",
        function_code = "699",
        name = "file_rejects",
        kind = "administrative_messages"
    ))]
    FileReject,

    #[strum(props(
        mti = "1644",
        function_code = "693",
        name = "text_messages",
        kind = "administrative_messages"
    ))]
    TextMessage,

    #[strum(props(
        mti = "1644",
        function_code = "640",
        name = "currency_updates",
        kind = "administrative_messages"
    ))]
    CurrencyUpdate,

    // Fee collection messages
    #[strum(props(
        mti = "1740",
        function_code = "700",
        name = "fee_collections_customer",
        kind = "fee_collection_messages"
    ))]
    FeeCollectionCustomer,

    #[strum(props(
        mti = "1740",
        function_code = "780",
        name = "fee_collections_customer_return",
        kind = "fee_collection_messages"
    ))]
    FeeCollectionCustomerReturn,

    #[strum(props(
        mti = "1740",
        function_code = "781",
        name = "fee_collections_customer_resubmission",
        kind = "fee_collection_messages"
    ))]
    FeeCollectionCustomerResubmission,

    #[strum(props(
        mti = "1740",
        function_code = "782",
        name = "fee_collections_customer_arbitration_return",
        kind = "fee_collection_messages"
    ))]
    FeeCollectionCustomerArbitrationReturn,

    #[strum(props(
        mti = "1740",
        function_code = "783",
        name = "fee_collections_clearing",
        kind = "fee_collection_messages"
    ))]
    FeeCollectionClearing,

    // not intended to be used
    #[strum(props(mti = "unknown", function_code = "unknown", name = "unknown", kind = "unknown"))]
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
                "mti",
                FieldCharType::Iso8583_mti,
                4,
                FieldSizeType::Fixed,
            ), // Message Type Indicator
            IsoField::new(
                "Bitmaps",
                "bitmaps", // Primary Bitmap (8 bytes) + DE 1 (8 bytes) = 16 bytes
                FieldCharType::Iso8583_bmps,
                16,
                FieldSizeType::BitMap,
            ), // Bitmap
            IsoField::new(
                "Primary Account Number",
                "002", // DE 2
                FieldCharType::Iso8583_ns, // Using ns due to pans with `*`
                19,
                FieldSizeType::LlVar,
            ), // Primary Account Number
            IsoField::new(
                "Processing Code",
                "003", // DE 3
                FieldCharType::Iso8583_ns,
                6,
                FieldSizeType::Fixed,
            ), // Processing Code
            IsoField::new(
                "Amount, Txn",
                "004", // DE 4
                FieldCharType::Iso8583_n,
                12,
                FieldSizeType::Fixed,
            ), // Amount, Txn
            IsoField::new(
                "Amount, Reconciliation",
                "005", // DE 5
                FieldCharType::Iso8583_n,
                12,
                FieldSizeType::Fixed,
            ), // Amount, Reconciliation
            IsoField::new(
                "Amount, Cardholder Billing",
                "006", // DE 6
                FieldCharType::Iso8583_n,
                12,
                FieldSizeType::Fixed,
            ), // Amount, Cardholder Billing
            IsoField::new(
                "Date and Time, Transmission",
                "007", // DE 7
                FieldCharType::Iso8583_ns,
                10,
                FieldSizeType::Fixed,
            ), // Date and Time, Transmission
            IsoField::new(
                "Amount, Cardholder Billing Fee",
                "008", // DE 8
                FieldCharType::Iso8583_n,
                8,
                FieldSizeType::Fixed,
            ), // Amount, Cardholder Billing Fee
            IsoField::new(
                "Conversion Rate, Reconciliation",
                "009", // DE 9
                FieldCharType::Iso8583_n,
                8,
                FieldSizeType::Fixed,
            ), // Conversion Rate, Reconciliation
            IsoField::new(
                "Conversion Rate, Cardholder Billing",
                "010", // DE 10
                FieldCharType::Iso8583_n,
                8,
                FieldSizeType::Fixed,
            ), // Conversion Rate, Cardholder Billing
            IsoField::new(
                "Systems Trace Audit Number",
                "011", // DE 11
                FieldCharType::Iso8583_n,
                6,
                FieldSizeType::Fixed,
            ), // Systems Trace Audit Number
            IsoField::new(
                "Date and Time, Local Txn",
                "012", // DE 12
                FieldCharType::Iso8583_ns,
                12,
                FieldSizeType::Fixed,
            ), // Date and Time, Local Txn
            IsoField::new(
                "Date, Effective",
                "013", // DE 13
                FieldCharType::Iso8583_ns,
                4,
                FieldSizeType::Fixed,
            ), // Date, Effective
            IsoField::new(
                "Date, Expiration",
                "014", // DE 14
                FieldCharType::Iso8583_ns,
                4,
                FieldSizeType::Fixed,
            ), // Date, Expiration
            IsoField::new(
                "Date, Settlement",
                "015", // DE 15
                FieldCharType::Iso8583_ns,
                6,
                FieldSizeType::Fixed,
            ), // Date, Settlement
            IsoField::new(
                "Date, Conversion",
                "016", // DE 16
                FieldCharType::Iso8583_ns,
                4,
                FieldSizeType::Fixed,
            ), // Date, Conversion
            IsoField::new(
                "Date, Capture",
                "017", // DE 17
                FieldCharType::Iso8583_ns,
                4,
                FieldSizeType::Fixed,
            ), // Date, Capture
            IsoField::new(
                "Merchant Type",
                "018", // DE 18
                FieldCharType::Iso8583_n,
                4,
                FieldSizeType::Fixed,
            ), // Merchant Type
            IsoField::new(
                "Country Code, Acquiring Inst",
                "019", // DE 19
                FieldCharType::Iso8583_n,
                3,
                FieldSizeType::Fixed,
            ), // Country Code, Acquiring Inst
            IsoField::new(
                "Country Code, Primary Account Number",
                "020", // DE 20
                FieldCharType::Iso8583_n,
                3,
                FieldSizeType::Fixed,
            ), // Country Code, Primary Account Number
            IsoField::new(
                "Country Code, Forwarding Inst",
                "021", // DE 21
                FieldCharType::Iso8583_n,
                3,
                FieldSizeType::Fixed,
            ), // Country Code, Forwarding Inst
            IsoField::new(
                "Point of Service Data Code",
                "022", // DE 22
                FieldCharType::Iso8583_an,
                12,
                FieldSizeType::Fixed,
            ), // Point of Service Data Code
            IsoField::new(
                "Card Sequence Number",
                "023", // DE 23
                FieldCharType::Iso8583_n,
                3,
                FieldSizeType::Fixed,
            ), // Card Sequence Number
            IsoField::new(
                "Function Code",
                "024", // DE 24
                FieldCharType::Iso8583_an,
                3,
                FieldSizeType::Fixed,
            ), // Function Code
            IsoField::new(
                "Message Reason Code",
                "025", // DE 25
                FieldCharType::Iso8583_n,
                4,
                FieldSizeType::Fixed,
            ), // Message Reason Code
            IsoField::new(
                "Card Acceptor Business Code",
                "026", // DE 26
                FieldCharType::Iso8583_n,
                4,
                FieldSizeType::Fixed,
            ), // Card Acceptor Business Code
            IsoField::new(
                "Approval Code Length",
                "027", // DE 27
                FieldCharType::Iso8583_n,
                1,
                FieldSizeType::Fixed,
            ), // Approval Code Length
            IsoField::new(
                "Date, Reconciliation",
                "028", // DE 28
                FieldCharType::Iso8583_ns,
                6,
                FieldSizeType::Fixed,
            ), // Date, Reconciliation
            IsoField::new(
                "Reconciliation Indicator",
                "029", // DE 29
                FieldCharType::Iso8583_n,
                3,
                FieldSizeType::Fixed,
            ), // Reconciliation Indicator
            IsoField::new(
                "Amounts, Original",
                "030", // DE 30
                FieldCharType::Iso8583_n,
                24,
                FieldSizeType::Fixed,
            ), // Amounts, Original
            IsoField::new(
                "Acquirer Reference Data",
                "031", // DE 31
                FieldCharType::Iso8583_ns,
                23,
                FieldSizeType::LlVar,
            ), // Acquirer Reference Data
            IsoField::new(
                "Acquirer Inst Id Code",
                "032", // DE 32
                FieldCharType::Iso8583_n,
                11,
                FieldSizeType::LlVar,
            ), // Acquirer Inst Id Code
            IsoField::new(
                "Forwarding Inst Id Code",
                "033", // DE 33
                FieldCharType::Iso8583_n,
                11,
                FieldSizeType::LlVar,
            ), // Forwarding Inst Id Code
            IsoField::new(
                "Primary Account Number, Extended",
                "034", // DE 34
                FieldCharType::Iso8583_ns,
                28,
                FieldSizeType::LlVar,
            ), // Primary Account Number, Extended
            IsoField::new(
                "Track 2 Data",
                "035", // DE 35
                FieldCharType::Iso8583_z,
                37,
                FieldSizeType::LlVar,
            ), // Track 2 Data
            IsoField::new(
                "Track 3 Data",
                "036", // DE 36
                FieldCharType::Iso8583_z,
                104,
                FieldSizeType::LllVar,
            ), // Track 3 Data
            IsoField::new(
                "Retrieval Reference Number",
                "037", // DE 37
                FieldCharType::Iso8583_ans,
                12,
                FieldSizeType::Fixed,
            ), // Retrieval Reference Number
            IsoField::new(
                "Approval Code",
                "038", // DE 38
                FieldCharType::Iso8583_ans,
                6,
                FieldSizeType::Fixed,
            ), // Approval Code
            IsoField::new(
                "Action Code",
                "039", // DE 39
                FieldCharType::Iso8583_n,
                3,
                FieldSizeType::Fixed,
            ), // Action Code
            IsoField::new(
                "Service Code",
                "040", // DE 40
                FieldCharType::Iso8583_n,
                3,
                FieldSizeType::Fixed,
            ), // Service Code
            IsoField::new(
                "Card Acceptor Terminal Id",
                "041", // DE 41
                FieldCharType::Iso8583_ans,
                8,
                FieldSizeType::Fixed,
            ), // Card Acceptor Terminal Id
            IsoField::new(
                "Card Acceptor Id Code",
                "042", // DE 42
                FieldCharType::Iso8583_ans,
                15,
                FieldSizeType::Fixed,
            ), // Card Acceptor Id Code
            IsoField::new(
                "Card Acceptor Name/Location",
                "043", // DE 43
                FieldCharType::Iso8583_ans,
                99,
                FieldSizeType::LlVar,
            ), // Card Acceptor Name/Location
            IsoField::new(
                "Additional Response Data",
                "044", // DE 44
                FieldCharType::Iso8583_ans,
                99,
                FieldSizeType::LlVar,
            ), // Additional Response Data
            IsoField::new(
                "Track 1 Data",
                "045", // DE 45
                FieldCharType::Iso8583_ans,
                76,
                FieldSizeType::LlVar,
            ), // Track 1 Data
            IsoField::new(
                "Amounts, Fees",
                "046", // DE 46
                FieldCharType::Iso8583_ans,
                204,
                FieldSizeType::LllVar,
            ), // Amounts, Fees
            IsoField::new(
                "Additional Data - National",
                "047", // DE 47
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Additional Data - National
            IsoField::new(
                "Additional Data - Private",
                "048", // DE 48
                FieldCharType::Iso8583_an,
                999,
                FieldSizeType::LllVar,
            ), // Additional Data - Private
            IsoField::new(
                "Currency Code, Txn",
                "049", // DE 49
                FieldCharType::Iso8583_n,
                3,
                FieldSizeType::Fixed,
            ), // Currency Code, Txn
            IsoField::new(
                "Currency Code, Reconciliation",
                "050", // DE 50
                FieldCharType::Iso8583_n,
                3,
                FieldSizeType::Fixed,
            ), // Currency Code, Reconciliation
            IsoField::new(
                "Currency Code, Cardholder Billing",
                "051", // DE 51
                FieldCharType::Iso8583_n,
                3,
                FieldSizeType::Fixed,
            ), // Currency Code, Cardholder Billing
            IsoField::new(
                "Personal Id Number (PIN) Data",
                "052", // DE 52
                FieldCharType::Iso8583_b,
                8,
                FieldSizeType::Fixed,
            ), // Personal Id Number (PIN) Data
            IsoField::new(
                "Security Related Control Information",
                "053", // DE 53
                FieldCharType::Iso8583_b,
                48,
                FieldSizeType::LlVar,
            ), // Security Related Control Information
            IsoField::new(
                "Amounts, Additional",
                "054", // DE 54
                FieldCharType::Iso8583_ans,
                240,
                FieldSizeType::LllVar,
            ), // Amounts, Additional
            IsoField::new(
                "IC Card System Related Data",
                "055", // DE 55
                FieldCharType::Iso8583_b,
                255,
                FieldSizeType::LllVar,
            ), // IC Card System Related Data
            IsoField::new(
                "Original Data Elements",
                "056", // DE 56
                FieldCharType::Iso8583_n,
                35,
                FieldSizeType::LlVar,
            ), // Original Data Elements
            IsoField::new(
                "Authorization Life Cycle Code",
                "057", // DE 57
                FieldCharType::Iso8583_n,
                3,
                FieldSizeType::Fixed,
            ), // Authorization Life Cycle Code
            IsoField::new(
                "Authorizing Agent Inst Id Cod",
                "058", // DE 58
                FieldCharType::Iso8583_n,
                11,
                FieldSizeType::LlVar,
            ), // Authorizing Agent Inst Id Code
            IsoField::new(
                "Transport Data",
                "059", // DE 59
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Transport Data
            IsoField::new(
                "Reserved for National use",
                "060", // DE 60
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for National use
            IsoField::new(
                "Reserved for National use",
                "061", // DE 61
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for National use
            IsoField::new(
                "Additional Data 2",
                "062", // DE 62
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for Private use
            IsoField::new(
                "Txn Life Cycle ID",
                "063", // DE 63
                FieldCharType::Iso8583_ans,
                16,
                FieldSizeType::LllVar,
            ), // Reserved for Private use
            IsoField::new(
                "Message Authentication Code Field",
                "064", // DE 64
                FieldCharType::Iso8583_b,
                8,
                FieldSizeType::Fixed,
            ), // Message Authentication Code Field
            IsoField::new(
                "Reserved for ISO use",
                "065", // DE 65
                FieldCharType::Iso8583_b,
                8,
                FieldSizeType::Fixed,
            ), // Reserved for ISO use
            IsoField::new(
                "Amounts, Original Fees",
                "066", // DE 66
                FieldCharType::Iso8583_ans,
                204,
                FieldSizeType::LllVar,
            ), //Reconciliation code , Original Fees
            IsoField::new(
                "Extended Payment Data",
                "067", // DE 67
                FieldCharType::Iso8583_n,
                2,
                FieldSizeType::Fixed,
            ), // Extended Payment Data
            IsoField::new(
                "Country Code, Receiving Inst",
                "068", // DE 68
                FieldCharType::Iso8583_n,
                3,
                FieldSizeType::Fixed,
            ), // Country Code, Receiving Inst
            IsoField::new(
                "Country Code, Settlement Inst",
                "069", // DE 69
                FieldCharType::Iso8583_n,
                3,
                FieldSizeType::Fixed,
            ), // Country Code, Settlement Inst
            IsoField::new(
                "Country Code, Authorizing Agent Inst",
                "070", // DE 70
                FieldCharType::Iso8583_n,
                3,
                FieldSizeType::Fixed,
            ), // Country Code, Authorizing Agent Inst
            IsoField::new(
                "Message Number",
                "071", // DE 71
                FieldCharType::Iso8583_n,
                8,
                FieldSizeType::Fixed,
            ), // Message Number
            IsoField::new(
                "Data Record",
                "072", // DE 72
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Data Record
            IsoField::new(
                "Date, Action",
                "073", // DE 73
                FieldCharType::Iso8583_ns,
                6,
                FieldSizeType::Fixed,
            ), // Date, Action
            IsoField::new(
                "Credits, Number",
                "074", // DE 74
                FieldCharType::Iso8583_n,
                10,
                FieldSizeType::Fixed,
            ), // Credits, Number
            IsoField::new(
                "Credits, Reversal Number",
                "075", // DE 75
                FieldCharType::Iso8583_n,
                10,
                FieldSizeType::Fixed,
            ), // Credits, Reversal Number
            IsoField::new(
                "Debits, Number",
                "076", // DE 76
                FieldCharType::Iso8583_n,
                10,
                FieldSizeType::Fixed,
            ), // Debits, Number
            IsoField::new(
                "Debits, Reversal Number",
                "077", // DE 77
                FieldCharType::Iso8583_n,
                10,
                FieldSizeType::Fixed,
            ), // Debits, Reversal Number
            IsoField::new(
                "Transfer, Number",
                "078", // DE 78
                FieldCharType::Iso8583_n,
                10,
                FieldSizeType::Fixed,
            ), // Transfer, Number
            IsoField::new(
                "Transfer, Reversal Number",
                "079", // DE 79
                FieldCharType::Iso8583_n,
                10,
                FieldSizeType::Fixed,
            ), // Transfer, Reversal Number
            IsoField::new(
                "Inquiries, Number",
                "080", // DE 80
                FieldCharType::Iso8583_n,
                10,
                FieldSizeType::Fixed,
            ), // Inquiries, Number
            IsoField::new(
                "Authorizations, Number",
                "081", // DE 81
                FieldCharType::Iso8583_n,
                10,
                FieldSizeType::Fixed,
            ), // Authorizations, Number
            IsoField::new(
                "Inquiries, Reversal Number",
                "082", // DE 82
                FieldCharType::Iso8583_n,
                10,
                FieldSizeType::Fixed,
            ), // Inquiries, Reversal Number
            IsoField::new(
                "Payments, Number",
                "083", // DE 83
                FieldCharType::Iso8583_n,
                10,
                FieldSizeType::Fixed,
            ), // Payments, Number
            IsoField::new(
                "Payments, Reversal Number",
                "084", // DE 84
                FieldCharType::Iso8583_n,
                10,
                FieldSizeType::Fixed,
            ), // Payments, Reversal Number
            IsoField::new(
                "Fee Collections, Number",
                "085", // DE 85
                FieldCharType::Iso8583_n,
                10,
                FieldSizeType::Fixed,
            ), // Fee Collections, Number
            IsoField::new(
                "Credits, Amount",
                "086", // DE 86
                FieldCharType::Iso8583_n,
                16,
                FieldSizeType::Fixed,
            ), // Credits, Amount
            IsoField::new(
                "Credits, Reversal Amount",
                "087", // DE 87
                FieldCharType::Iso8583_n,
                16,
                FieldSizeType::Fixed,
            ), // Credits, Reversal Amount
            IsoField::new(
                "Debits, Amount",
                "088", // DE 88
                FieldCharType::Iso8583_n,
                16,
                FieldSizeType::Fixed,
            ), // Debits, Amount
            IsoField::new(
                "Debits, Reversal Amount",
                "089", // DE 89
                FieldCharType::Iso8583_n,
                16,
                FieldSizeType::Fixed,
            ), // Debits, Reversal Amount
            IsoField::new(
                "Authorizations, Reversal Number",
                "090", // DE 90
                FieldCharType::Iso8583_n,
                10,
                FieldSizeType::Fixed,
            ), // Authorizations, Reversal Number
            IsoField::new(
                "Country Code, Txn Destination Inst",
                "091", // DE 91
                FieldCharType::Iso8583_n,
                3,
                FieldSizeType::Fixed,
            ), // Country Code, Txn Destination Inst
            IsoField::new(
                "Country Code, Txn Originator Inst",
                "092", // DE 92
                FieldCharType::Iso8583_n,
                3,
                FieldSizeType::Fixed,
            ), // Country Code, Txn Originator Inst
            IsoField::new(
                "Txn Destination Inst Id Code",
                "093", // DE 93
                FieldCharType::Iso8583_n,
                11,
                FieldSizeType::LlVar,
            ), // Txn Destination Inst Id Code
            IsoField::new(
                "Txn Originator Inst Id Code",
                "094", // DE 94
                FieldCharType::Iso8583_n,
                11,
                FieldSizeType::LlVar,
            ), // Txn Originator Inst Id Code
            IsoField::new(
                "Card Issuer Reference Data",
                "095", // DE 95
                FieldCharType::Iso8583_n,
                10,
                FieldSizeType::LlVar,
            ), // Card Issuer Reference Data
            IsoField::new(
                "Key Management Data",
                "096", // DE 96
                FieldCharType::Iso8583_b,
                999,
                FieldSizeType::LllVar,
            ), // Key Management Data
            IsoField::new(
                "Amount, Net Reconciliation",
                "097", // DE 97
                FieldCharType::Iso8583_xn,
                17,
                FieldSizeType::Fixed,
            ), // Amount, Net Reconciliation
            IsoField::new(
                "Payee",
                "098", // DE 98
                FieldCharType::Iso8583_ans,
                25,
                FieldSizeType::Fixed,
            ), // Payee
            IsoField::new(
                "Settlement Inst Id Code",
                "099", // DE 99
                FieldCharType::Iso8583_an,
                11,
                FieldSizeType::LlVar,
            ), // Settlement Inst Id Code
            IsoField::new(
                "Receiving Inst Id Code",
                "100", // DE 100
                FieldCharType::Iso8583_n,
                11,
                FieldSizeType::LlVar,
            ), // Receiving Inst Id Code
            IsoField::new(
                "File Name",
                "101", // DE 101
                FieldCharType::Iso8583_ans,
                17,
                FieldSizeType::LlVar,
            ), // File Name
            IsoField::new(
                "Account Id 1",
                "102", // DE 102
                FieldCharType::Iso8583_ans,
                28,
                FieldSizeType::LlVar,
            ), // Account Id 1
            IsoField::new(
                "Account Id 2",
                "103", // DE 103
                FieldCharType::Iso8583_ans,
                28,
                FieldSizeType::LlVar,
            ), // Account Id 2
            IsoField::new(
                "Txn Description",
                "104", // DE 104
                FieldCharType::Iso8583_ans,
                100,
                FieldSizeType::LllVar,
            ), // Txn Description
            IsoField::new(
                "Credits, Chargeback Amount",
                "105", // DE 105
                FieldCharType::Iso8583_n,
                16,
                FieldSizeType::Fixed,
            ), // Credits, Chargeback Amount
            IsoField::new(
                "Debits, Chargeback Amount",
                "106", // DE 106
                FieldCharType::Iso8583_n,
                16,
                FieldSizeType::Fixed,
            ), // Debits, Chargeback Amount
            IsoField::new(
                "Credits, Chargeback Number",
                "107", // DE 107
                FieldCharType::Iso8583_n,
                10,
                FieldSizeType::Fixed,
            ), // Credits, Chargeback Number
            IsoField::new(
                "Debits, Chargeback Number",
                "108", // DE 108
                FieldCharType::Iso8583_n,
                10,
                FieldSizeType::Fixed,
            ), // Debits, Chargeback Number
            IsoField::new(
                "Credits, Fee Amounts",
                "109", // DE 109
                FieldCharType::Iso8583_ans,
                84,
                FieldSizeType::LlVar,
            ), // Credits, Fee Amounts
            IsoField::new(
                "Debits, Fee Amounts",
                "110", // DE 110
                FieldCharType::Iso8583_ans,
                84,
                FieldSizeType::LlVar,
            ), // Debits, Fee Amounts
            IsoField::new(
                "Amount, Currency Conversion Assessment",
                "111", // DE 111
                FieldCharType::Iso8583_n,
                12,
                FieldSizeType::LllVar,
            ), // Reserved for ISO use
            IsoField::new(
                "Reserved for ISO use",
                "112", // DE 112
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for ISO use
            IsoField::new(
                "Reserved for ISO use",
                "113", // DE 113
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for ISO use
            IsoField::new(
                "Reserved for ISO use",
                "114", // DE 114
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for ISO use
            IsoField::new(
                "Reserved for ISO use",
                "115", // DE 115
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for ISO use
            IsoField::new(
                "Reserved for National use",
                "116", // DE 116
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for National use
            IsoField::new(
                "Reserved for National use",
                "117", // DE 117
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for National use
            IsoField::new(
                "Reserved for National use",
                "118", // DE 118
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for National use
            IsoField::new(
                "Reserved for National use",
                "119", // DE 119
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for National use
            IsoField::new(
                "Reserved for National use",
                "120", // DE 120
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for National use
            IsoField::new(
                "Reserved for National use",
                "121", // DE 121
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for National use
            IsoField::new(
                "Reserved for National use",
                "122", // DE 122
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for National use
            IsoField::new(
                "Additional Data 3",
                "123", // DE 123
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Additional Data 3
            IsoField::new(
                "Additional Data 4)",
                "124", // DE 124
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Additional Data 4)
            IsoField::new(
                "Additional Data 5",
                "125", // DE 125
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Additional Data 5
            IsoField::new(
                "Reserved for Private use",
                "126", // DE 126
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Reserved for Private use
            IsoField::new(
                "Network Data",
                "127", // DE 127
                FieldCharType::Iso8583_ans,
                999,
                FieldSizeType::LllVar,
            ), // Network Data
            IsoField::new(
                "Message Authentication Code Field",
                "128", // DE 128
                FieldCharType::Iso8583_b,
                8,
                FieldSizeType::Fixed,
            ), // Message Authentication Code Field
        ];
        h
    }
}
