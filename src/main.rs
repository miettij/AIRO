use std::fs::File;
use pcsc::*;

fn lookup(rapdu: &[u8]) -> String {
// fn lookup(rapdu: &[u8]) -> Result<(), Box<dyn std::error::Error>> {

    let file = File::open("./src/responses.csv").unwrap();

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .double_quote(false)
        .flexible(true)
        .from_reader(file);

    for result in rdr.records() {
        let record = result.unwrap();

        if record[0] == format!("{:X}", &rapdu[0]) && record[1] == format!("{:X}", &rapdu[1]) {
            return format!("{:?}", &record[3]);
        }
    }

    return String::from("ERROR: No lookup match found!");
    // Ok(())
}

fn send_apdu(card: Card, apdu: &[u8]) {
    let mut res_buf = [0; MAX_BUFFER_SIZE];
    let res = match card.transmit(apdu, &mut res_buf) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Failed to transmit APDU command to card: {}", err);
            std::process::exit(1);
        }
    };
    
    println!("APDU: {:?}", apdu);
    println!("RAPDU: {:X} {:X}", res[0], res[1]);
    println!("RAPDU TEXT: {}", lookup(&res));
}

fn main() {
    // Establish a PC/SC context.
    let ctx = match Context::establish(Scope::User) {
        Ok(ctx) => ctx,
        Err(err) => {
            eprintln!("Failed to establish context: {}", err);
            std::process::exit(1);
        }
    };

    // List available readers.
    let mut readers_buf = [0; 2048];
    let mut readers = match ctx.list_readers(&mut readers_buf) {
        Ok(readers) => readers,
        Err(err) => {
            eprintln!("Failed to list readers: {}", err);
            std::process::exit(1);
        }
    };

    // Use the first reader.
    let reader = match readers.next() {
        Some(reader) => reader,
        None => {
            println!("No readers are connected.");
            return;
        }
    };
    println!("Using reader: {:?}", reader);

    // Connect to the card.
    let mut card = match ctx.connect(reader, ShareMode::Shared, Protocols::ANY) {
        Ok(card) => card,
        Err(Error::NoSmartcard) => {
            println!("A smartcard is not present in the reader.");
            return;
        }
        Err(err) => {
            eprintln!("Failed to connect to card: {}", err);
            std::process::exit(1);
        }
    };

    // let tran = match card.transaction() {
    //     Ok(tran) => tran,
    //     Err(err) => {
    //         eprintln!("Failed to establish connection to card: {}", err);
    //         std::process::exit(1);
    //     }
    // };

    // let attr_len: usize = tran.get_attribute_len(Attribute::VendorName).unwrap();
    // println!("{:?}", attr_len);
    
    // let mut buffer = [0; MAX_BUFFER_SIZE];
    // let attr = tran.get_attribute(Attribute::VendorName, &mut buffer).unwrap();
    // println!("{:x?}", attr);

    let class_byte: &[u8] = &[0x00];
    let instruction_byte: &[u8] = &[0xA4]; // Select command
    let p1: &[u8] = &[0x00]; // Select by name
    // let p1: &[u8] = &[0x04]; // Select by name
    let p2: &[u8] = &[0x00]; // Leave empty
    let lc: &[u8] = &[0x02]; // Get max len
    // let aid = &[0x31, 0x50, 0x41, 0x59, 0x2E, 0x53, 0x59, 0x53, 0x2E, 0x44, 0x44, 0x46, 0x30, 0x31];
    // let aid = "1PAY.SYS.DDF01".as_bytes();
    // let aid = "A0000000049999".as_bytes();
    // let id = &[0x00, 0x03];
    let le: &[u8] = &[0x00];

    let apdu_master = &[0x00, 0xa4, 0x00, 0x00, 0x00];

    send_apdu(card, apdu_master);
    
    // for i in 0u16..65535 {
    //     if i % 100 == 0 {
    //         println!("{:?}", i);
    //     }
    //     let id = i.to_le_bytes();
    //     let apdu: Vec<u8> = [class_byte, instruction_byte, p1, p2, lc, &id, le].concat();
    //     // let apdu: Vec<u8> = [class_byte, instruction_byte, p1, p2, &asd, le].concat();
    //     // println!("APDU command: {:x?}", apdu);
        
    //     let mut res_buf = [0; MAX_BUFFER_SIZE];
    //     let res = match card.transmit(&apdu, &mut res_buf) {
    //         Ok(res) => res,
    //         Err(err) => {
    //             eprintln!("Failed to transmit APDU command to card: {}", err);
    //             std::process::exit(1);
    //         }
    //     };
        
    //     // println!("{:x} - {:x}", res[0], res[1]);

    //     if format!("{:X}", res[0]) != "6A" || format!("{:X}", res[1]) != "6A" {
    //         println!("AAAAAAAAA: {:?}", i);
    //         std::process::exit(0);
    //     }
    //     // println!("Res: {:x?}", res);
    //     // println!("Buffer: {:x?}", res_buf);
    // }

    // let mut res_buf2 = [0; MAX_BUFFER_SIZE];
    // for sfi in 1..32 {
    //     for rec in 1..17 {
    //         println!("{:?}, {:?}", sfi, rec);

    //         let a1: &[u8] = &[0x00];
    //         let a2: &[u8] = &[0xB2];
    //         let a3: &[u8] = &[rec];
    //         let a4: &[u8] = &[(sfi << 3) | 4];
    //         let a5: &[u8] = &[0x00];
    //         let apdu2: Vec<u8> = [a1, a2, a3, a4, a5].concat();
    //         let tlv = match tran.transmit(&apdu2, &mut res_buf2) {
    //             Ok(res2) => res2,
    //             Err(err) => {
    //                 eprintln!("APUA: {}", err);
    //                 std::process::exit(1);
    //             }
    //         };

    //         println!("{:x?}", tlv);
    //     }
    // }



    // Send an APDU command.
    // let apdu = b"\x80\xca\x9f\x4f\x00";
    // // let apdu = b"\x00\xa4\x04\x00\x0A\xA0\x00\x00\x00\x62\x03\x01\x0C\x06\x01";
    // println!("Sending APDU: {:?}", apdu);
    // let mut rapdu_buf = [0; MAX_BUFFER_SIZE];
    // let rapdu = match card.transmit(apdu, &mut rapdu_buf) {
    //     Ok(rapdu) => rapdu,
    //     Err(err) => {
    //         eprintln!("Failed to transmit APDU command to card: {}", err);
    //         std::process::exit(1);
    //     }
    // };
    // println!("APDU response: {:?}", rapdu);
}