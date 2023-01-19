use std::io::Error;

use winspool::{close_printer, open_printer, start_doc_printer};

fn main() {
    let printer_name = "NPI642118 (HP LaserJet M110w)";

    match open_printer(printer_name) {
        Some(handle) => {
            let success = start_doc_printer(handle, "Test Document");
            if success == 0 {
                println!("close_printer: {}", close_printer(handle));
                println!("{}", Error::last_os_error());
            }
        }
        None => {
            println!("open_printer: {}", Error::last_os_error());
        }
    }
}
