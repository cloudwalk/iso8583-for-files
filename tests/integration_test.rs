use iso8583::iso_msg::IsoMsg;
use iso8583::iso_specs::IsoSpecs;
use std::fs::File;
use std::io::Read;

#[test]
fn parse_bitmap_binary() {
    let bitmap: &[u8] = &[128, 0, 1, 0, 0, 1, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0];
    let bit_arrays = IsoMsg::process_bitmap(bitmap);
    assert_eq!(format!("{:?}", bit_arrays), "[10000000000000000000000100000000000000000000000100000000000000000000001000000000000000000000000000000000000000000000000000000000]");
}

#[test]
fn parse_file_binary() {
    let mut f = File::open("tests/R111_sample.ipm").expect("no file found");
    let metadata = std::fs::metadata("tests/R111_sample.ipm").expect("unable to read metadata");
    let mut payload = vec![0; metadata.len() as usize];
    f.read(&mut payload).expect("buffer overflow");

    let parsed_messages: Vec<iso8583::ParsedMessage> = iso8583::parse_file(&payload);
}
