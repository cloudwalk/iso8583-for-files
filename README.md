# ISO-8583-FOR-FILES
`iso8583-for-files` is a parser focused on translating RDW, blocking, Bitmaps and PDS directly into one data structure.

Initially forked from [rohitjoshi's parser][iso8583] and heavily adapted for files parsing.

## High level features
- Provide an easy to use interface
- Remove Record Descriptor Word ([RDW][rdw])
- Deblocking
- PDS reading

## Usage

```rust
use iso8583::iso_msg::IsoMsg;
use std::fs::File;
use std::io::Read;

fn parse_r111_binary() {
    let file_name = "tests/R111_sample.ipm";
    let mut file = File::open(file_name).expect("no file found");
    let metadata = std::fs::metadata(file_name).expect("unable to read metadata");

    let mut payload = vec![0; metadata.len() as usize];

    file.read(&mut payload).expect("buffer overflow");

    let _iso8583_file: iso8583::Iso8583File = iso8583::parse_file(payload).unwrap();
}
```

Other examples are available on the [tests file][test]. Tests can be executed directly via shell
```
cargo test
```

## License

This project is licensed under the [MIT license][license].

[iso8583]: https://github.com/rohitjoshi/iso8583
[rdw]: https://www.ibm.com/docs/en/zos/2.2.0?topic=records-record-descriptor-word-rdw
[license]: https://github.com/cloudwalk/iso8583-for-files/blob/main/LICENSE-MIT
[test]: https://github.com/cloudwalk/iso8583-for-files/blob/main/tests/integration_test.rs
