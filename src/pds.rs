use std::collections::HashMap;

const PDS_LEN_SIZE: usize = 3;
const PDS_ID_SIZE: usize = 4;

/// Each pds comes with a fixed id, length and value, each are concatenated to each other
/// so a typical pds is (IIIILLLV(V+)+) where Id is always length 4, and the Length is always 3 characters
pub fn get_pds_values(message: crate::Message) -> Result<Option<HashMap<String, String>>, String> {
    if message.label != "Additional Data - Private" || message.value.len() < PDS_LEN_SIZE {
        return Ok(None);
    }
    if let Ok(full_pds_text) = std::str::from_utf8(&message.value) {
        let mut position = 3usize; // the position starts at 3 because DE48 is an LLLVAR, so we can ignore the 3 first characters
        let mut pds_values: HashMap<String, String> = HashMap::new();
        while let Some((pds_size, pds_id, pds_value)) = pds_details(&full_pds_text, position) {
            pds_values.insert(pds_id.to_string(), pds_value.to_string());

            position = position + PDS_LEN_SIZE + PDS_ID_SIZE + pds_size;
        }
        Ok(Some(pds_values))
    } else {
        Err("".to_string())
    }
}

fn pds_details(pds_buffer: &str, position: usize) -> Option<(usize, &str, &str)> {
    if pds_buffer.len() > position + PDS_LEN_SIZE + PDS_ID_SIZE {
        let size_position = position + PDS_ID_SIZE;
        let value_position = size_position + PDS_LEN_SIZE;

        match pds_buffer[size_position..value_position].parse::<usize>() {
            Ok(i) => Some((
                i,
                &pds_buffer[position..size_position],
                &pds_buffer[value_position..value_position + i],
            )),
            Err(_) => None,
        }
    } else {
        None
    }
}

#[test]
fn test_pds_reading() {
    let message = crate::Message {
        label: "Additional Data - Private".to_string(),
        value: "302014800498620165001M03000250022203170000002337906128037200712402000374002000378001O0390017D00000000000000000391017C0000000000302149039201800D000000000004777039301800C0000000000000000394017C00000000003021490395016D0000000000047770396017C0000000000297372040001000000000000401010000000003604020100000000036".as_bytes().to_vec(),
    };

    let pds_values = get_pds_values(message).unwrap().unwrap();
    assert_eq!(pds_values.get("0148").unwrap(), "9862");
    assert_eq!(pds_values.get("0300").unwrap(), "0022203170000002337906128");
}
