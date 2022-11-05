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
    
    let res_lookup = lookup(&res);

    if res[0] != 0x6a {
    // if res_lookup != "File not found" && res_lookup != "Incorrect P1 or P2 parameter." {
        println!("APDU: {:X?}", apdu);
        println!("RAPDU: {:X?}", res);
        // println!("RAPDU TEXT: {}", res_lookup);
    }
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


    let p1_better: &[u8] = &[0x04];
    let p2_better: &[u8] = &[0x00];

    println!("");
    println!("SELECT");
    println!("");
    
    let aid = b"\xA0\x00\x00\x00\x25";
    let lc: &[u8] = &[0x05];
    let apdu = [class_byte, select, p1_better, p2_better, lc, aid, le].concat();
    // send_apdu(&card, &apdu);

    println!("");
    println!("READ");
    println!("");

    let mut data:Vec<String> = Vec::new();

    let mut i = 0;
    for sfi in 1u8..4 {
        for rec in 1u8..17 {
            
            let sfi_mod = (sfi << 3) | 4;
            let apdu = [class_byte, read_record, &[rec], &[sfi_mod], le].concat();
            
            // println!("");
            // println!("FIND:");
            // println!("");
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
            }

            // println!("");
            // println!("------");
            // println!("");
        }
    }

    for r in data {
        print!("\n");
        println!("{}", r);
    }
    
    print!("{}", i);
}