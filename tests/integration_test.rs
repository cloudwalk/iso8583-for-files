use iso8583::iso_msg::IsoMsg;
#[cfg(test)]
use std::collections::HashMap;
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

    let categories_indexes = iso8583_file.clone().categories_indexes;

    dbg!(&iso8583_file.clone().messages_count());
    dbg!(categories_indexes.get("message_exceptions").unwrap());

    assert_eq!(
        categories_indexes.get("financial_positions").unwrap(),
        &vec![2usize]
    );

    assert_eq!(categories_indexes.get("headers").unwrap(), &vec![0usize]);

    assert_eq!(iso8583_file.groups[2].messages[5].utf8_value(), "986");
}

#[test]
fn search_and_filter_t113_deblocked_sample() {
    let file_name = "tests/T113_sample.ipm";
    let mut file = File::open(file_name).expect("no file found");
    let metadata = std::fs::metadata(file_name).expect("unable to read metadata");

    let mut payload = vec![0; metadata.len() as usize];

    file.read(&mut payload).expect("buffer overflow");

    let iso8583_file: iso8583::Iso8583File = iso8583::parse_file(payload).unwrap();

    let searched = iso8583_file.search(HashMap::from([("Function Code".to_string(), vec!["200".to_string(), "691".to_string()])]));

    dbg!(searched);
    // assert_eq!(searched.groups.len(), 2318);
    // let filtered = iso8583_file.filter(vec!["Message Number"]);
}

#[test]
fn parse_t113_deblocked_sample() {
    let file_name = "tests/T113_sample.ipm";
    let mut file = File::open(file_name).expect("no file found");
    let metadata = std::fs::metadata(file_name).expect("unable to read metadata");

    let mut payload = vec![0; metadata.len() as usize];

    file.read(&mut payload).expect("buffer overflow");

    let iso8583_file: iso8583::Iso8583File = iso8583::parse_file(payload).unwrap();

    let categories_indexes = iso8583_file.clone().categories_indexes;

    dbg!(&iso8583_file.clone().messages_count());
    dbg!(&categories_indexes);
    let exceptions = categories_indexes.get("message_exceptions").unwrap();
    for g in exceptions {
        println!("\n\n");
        let original_group = iso8583_file.clone().groups.get(*g + 1).unwrap().clone();
        let original_messages = original_group.messages.clone();
        for m in original_messages {
            println!("orig: {} => {}", &m.get_label(), &m.utf8_value());
        }
        println!("\n\n{:?}\n\n", &original_group.pds);

        println!("\n\n");
        let group = iso8583_file.clone().groups.get(*g).unwrap().clone();
        let messages = group.messages.clone();
        for m in messages {
            println!("post: {} => {}", &m.get_label(), &m.utf8_value());
        }

        println!("\n\n{:?}\n\n", &original_group.messages);
        println!("\n\n{:?}\n\n", &group.pds);
    }
}

#[test]
fn parse_r119_binary() {
    let file_name = "tests/R119_files_processor.IPM";
    let mut file = File::open(file_name).expect("no file found");
    let metadata = std::fs::metadata(file_name).expect("unable to read metadata");

    let mut payload = vec![0; metadata.len() as usize];

    file.read(&mut payload).expect("buffer overflow");

    let iso8583_file: iso8583::Iso8583File = iso8583::parse_file(payload).unwrap();

    for group in iso8583_file.groups.iter() {
        println!("{:?}", group.data_elements);
        println!("{:?}", group.pds);
    }
}
