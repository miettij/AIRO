use std::fs::File;
use std::error::Error;
use std::str::from_utf8;
//use std::String::from_utf8_lossy;
use pcsc::*;
use hex::encode;

fn lookup(rapdu: &[u8]) -> Result<(), Box<dyn std::error::Error>> {

    let file = File::open("./src/responses.csv")?;

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .double_quote(false)
        .flexible(true)
        .from_reader(file);

    for result in rdr.records() {
        let record = result?;
        if record[0] == format!("{:X}", &rapdu[0]) && record[1] == format!("{:X}", &rapdu[1]) {
            println!("{:?}", &record[3]);
        }
    }

    if rapdu[0] == 0x6a && rapdu[1] == 0x82{
        println!("{:?}",0x6a)
    }

    let file = File::open("./src/responses.csv")?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .double_quote(false)
        .flexible(true)
        .from_reader(file);
    Ok(())
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
        Err(pcsc::Error::NoSmartcard) => {
            println!("A smartcard is not present in the reader.");
            return;
        }
        Err(err) => {
            eprintln!("Failed to connect to card: {}", err);
            std::process::exit(1);
        }
    };

    // Send an APDU command.
    let apdu = b"\x00\xa4\x04\x00\x0A\xA0\x00\x00\x00\x62\x03\x01\x0C\x06\x01";
    println!("Sending APDU: {:?}", apdu);
    let mut rapdu_buf = [0; MAX_BUFFER_SIZE];
    let rapdu = match card.transmit(apdu, &mut rapdu_buf) {
        Ok(rapdu) => rapdu,
        Err(err) => {
            eprintln!("Failed to transmit APDU command to card: {}", err);
            std::process::exit(1);
        }
    };
    println!("APDU response: {:?}", rapdu);

    lookup(&rapdu);
    //lookup([106,130])

}
