use eyre::{eyre, Result};
use std::collections::HashMap;

const PDS_LEN_SIZE: usize = 3;
const PDS_ID_SIZE: usize = 4;

/// Each pds comes with a fixed id, length and value, each are concatenated to each other
/// so a typical pds is (IIIILLLV(V+)+) where Id is always length 4, and the Length is always 3 characters
pub fn get_pds_values(additional_data: &Vec<u8>) -> Result<Option<HashMap<String, String>>> {
    if additional_data.len() < PDS_LEN_SIZE {
        return Ok(None);
    }
    if let Ok(full_pds_text) = std::str::from_utf8(additional_data) {
        let mut position = 0usize;
        let mut pds_values: HashMap<String, String> = HashMap::new();
        while let Some((pds_size, pds_id, pds_value)) = pds_details(full_pds_text, position) {
            pds_values.insert(pds_id.to_string(), pds_value.to_string());

            position = position + PDS_LEN_SIZE + PDS_ID_SIZE + pds_size;
        }
        Ok(Some(pds_values))
    } else {
        Err(eyre!("unable to get pds values for {:?}", &additional_data))
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
#[ignore]
fn test_pds_reading() {
    let additional_data = "302014800498620165001M03000250022203170000002337906128037200712402000374002000378001O0390017D00000000000000000391017C0000000000302149039201800D000000000004777039301800C0000000000000000394017C00000000003021490395016D0000000000047770396017C0000000000297372040001000000000000401010000000003604020100000000036".as_bytes().to_vec();

    let pds_values = get_pds_values(&additional_data).unwrap().unwrap();
    assert_eq!(pds_values.get("0148").unwrap(), "9862");
    assert_eq!(pds_values.get("0300").unwrap(), "0022203170000002337906128");
}

#[test]
fn test_pds_reading_from_r119() {
    let additional_data = "0002003MCG0003003MCG0023003NA 0146036001901986000000000005986000000000000014800498620158012          IV0165001M022001413711975214307".as_bytes().to_vec();

    let pds_values = get_pds_values(&additional_data).unwrap().unwrap();
    assert_eq!(pds_values.get("0002").unwrap(), "MCG");
    assert_eq!(pds_values.get("0003").unwrap(), "MCG");
    assert_eq!(pds_values.get("0158").unwrap(), "          IV");
    assert_eq!(pds_values.get("0220").unwrap(), "13711975214307");
}

#[test]
fn test_pds_reading_from_t121() {
    let additional_data = "014800498620165001M03000250012303040000002337904401037200717407000374002190378001O0390017D00000000000374010391017C0000000000000000039201800D000000000000000039301800C0000000000000000394017D00000000000374010395016D0000000000000000396017D0000000000037401040001000000000020401010000000000004020100000000002".as_bytes().to_vec();

    let pds_values = get_pds_values(&additional_data).unwrap().unwrap();
    assert_eq!(pds_values.get("0300").unwrap(), "0012303040000002337904401");
}
