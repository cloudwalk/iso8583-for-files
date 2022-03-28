use iso8583::iso_msg::IsoMsg;
#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use std::io::Read;

#[test]
fn parse_bitmap_binary() {
    let bitmap: &[u8] = &[128, 0, 1, 0, 0, 1, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0];
    let bit_arrays = IsoMsg::process_bitmap(bitmap);
    assert_eq!(format!("{:?}", bit_arrays), "10000000000000000000000100000000000000000000000100000000000000000000001000000000000000000000000000000000000000000000000000000000");
}

#[test]
fn parse_r111_binary() {
    let file_name = "tests/R111_sample.ipm";
    let mut file = File::open(file_name).expect("no file found");
    let metadata = std::fs::metadata(file_name).expect("unable to read metadata");

    let mut payload = vec![0; metadata.len() as usize];

    file.read(&mut payload).expect("buffer overflow");

    let _iso8583_file: iso8583::Iso8583File = iso8583::parse_file(payload).unwrap();
}

#[test]
fn parse_invalid_file() {
    let file_name = "tests/T112_sample.ipm";
    let mut file = File::open(file_name).expect("no file found");
    let metadata = std::fs::metadata(file_name).expect("unable to read metadata");

    let mut payload = vec![0; metadata.len() as usize];

    file.read(&mut payload).expect("buffer overflow");

    let _iso8583_file: iso8583::Iso8583File = iso8583::parse_file(payload).unwrap();
}

#[test]
fn parse_t113_blocked_with_rdw_binary() {
    let file_name = "tests/T113_sample.ipm";
    let mut file = File::open(file_name).expect("no file found");
    let metadata = std::fs::metadata(file_name).expect("unable to read metadata");

    let mut payload = vec![0; metadata.len() as usize];

    file.read(&mut payload).expect("buffer overflow");

    let iso8583_file: iso8583::Iso8583File = iso8583::parse_file(payload).unwrap();

    let financial_position = iso8583_file.groups.get(2usize).unwrap();
    assert_eq!(
        financial_position.pds.clone().get("0300").unwrap(),
        "0022203170000002337906128"
    );

    let messages_indexes = iso8583_file.clone().messages_indexes();
    assert_eq!(messages_indexes.get("trailers").unwrap(), &vec![3usize]);

    assert_eq!(
        messages_indexes.get("financial_positions").unwrap(),
        &vec![2usize]
    );

    assert_eq!(messages_indexes.get("headers").unwrap(), &vec![0usize]);

    assert_eq!(iso8583_file.groups[2].messages[5].utf8_value(), "986");
}
