use std::fs::File;
use pcsc::*;

fn lookup(rapdu: &[u8]) -> String {

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

    return String::from("");
}

fn send_apdu(card: &Card, apdu: &[u8]) -> Vec<u8> {
    println!("APDU: {:X?}", apdu);

    let mut res_buf = [0; MAX_BUFFER_SIZE];
    let res = match card.transmit(apdu, &mut res_buf) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Failed to transmit APDU command to card: {}", err);
            return [0x00, 0x00].to_vec();
            // std::process::exit(1);
        }
    };

    let res = res.to_vec();
    // res_codes.clone_from_slice(res);
    
    println!("RAPDU: {:X?}", res);
    println!("RAPDU TEXT: {}", lookup(&res));
    // println!("FULL RESPONSE: {:X?}", res_buf);


    return res;
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
    let card = match ctx.connect(reader, ShareMode::Shared, Protocols::ANY) {
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

    let class_byte: &[u8] = &[0x00];
    let select: &[u8] = &[0xA4]; // Select command
    let read_record: &[u8] = &[0xB2]; // Select command
    let get_response: &[u8] = &[0xC0]; // Select command
    let p1: &[u8] = &[0x00]; // Select by name
    let p1_by_name: &[u8] = &[0x04]; // Select by name
    let p2: &[u8] = &[0x00]; // Leave empty
    let le: &[u8] = &[0x00];

    let aid = "2PAY.SYS.DDF01".as_bytes();
    // let aid = 0xA0000000031010u64.to_be_bytes();
    // println!("AID: {:X?}", aid);
    // println!("AID: {:?}", aid);

    // let apdu = [class_byte, select, p1, p2, le].concat();
    // let apdu = [class_byte, instruction_byte, p1, p2, &aid, le].concat();
    // let apdu: Vec<u8> = [&[0x00u8], &[0xa4u8], &[0x04u8], &[0x00u8], &aid, &[0x00]].concat();

    // let rec = &[0x02];
    // let sfi:u8 = (1 << 3) | 4;
    // let sfi = &[sfi];

    println!("");
    println!("SELECT");
    println!("");

    let apdu = [class_byte, select, p1_by_name, p2, &aid, le].concat();
    send_apdu(&card, &apdu);

    println!("");
    println!("READ");
    println!("");

    let mut data:Vec<String> = Vec::new();

    let mut i = 0;
    for sfi in 1u8..32 {
        for rec in 1u8..17 {
    // for sfi in 1u8..32 {
    //     for rec in 1u8..17 {
            
            let sfi_mod = (sfi << 3) | 4;
            let apdu = [class_byte, read_record, &[rec], &[sfi_mod], le].concat();
            
            println!("");
            println!("FIND:");
            println!("");
            let res = send_apdu(&card, &apdu);
            let le: &[u8] = &[res[1]];
            let get_response_apdu = [class_byte, get_response, p1, p2, le].concat();
            
            if res[0] == 0x61 {
                println!("");
                println!("GET:");
                println!("");
                let res = send_apdu(&card, &get_response_apdu);
                
                if res[0] == 0x70 {
                    i = i + 1;
                    data.push(format!("{:02X?}", &res[..res.len() - 2]).replace(",", ""));
                }
                // let tlv = Tlv::from_vec( &res[..res.len() - 2] ).unwrap();

                // println!("TLV VAL: {:?}", tlv.val().to_vec())
            }
            

            println!("");
            println!("------");
            println!("");
        }
    }

    for r in data {
        print!("\n");
        println!("{}", r);
    }
    
    print!("{}", i);
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