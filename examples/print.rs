use std::{ffi::CString, io::Error};
use std::{thread, time};
use winspool::{
    close_printer, end_doc_printer, end_page_printer, open_printer, start_doc_printer,
    start_page_printer, write_printer,
};

fn main() {
    // let printer_name = "NPI642118 (HP LaserJet M110w)".to_string();
    let printer_name = "EPSON TM-P20 Receipt".to_string();

    match open_printer(printer_name) {
        Some(handle) => match start_doc_printer(handle) {
            0 => {
                close_printer(handle);

                println!("{}", Error::last_os_error());
            }
            _job_id => match start_page_printer(handle) {
                0 => {
                    end_doc_printer(handle);
                    close_printer(handle);

                    println!("{}", Error::last_os_error());
                }
                _ => {
                    println!("SUCCESS");

                    let receipt = r#"
Hello world.

This took way too long to figure out :)


                    "#;

                    write_printer(handle, CString::new(receipt).unwrap());

                    // Sleep for a bit so that the printer isn't terminated while printing.
                    // TODO: Use FlushPrinter instead or poll for the job to be complete?
                    thread::sleep(time::Duration::from_millis(1000));

                    end_page_printer(handle);
                    end_doc_printer(handle);
                    close_printer(handle);
                }
            },
        },
        None => {
            println!("open_printer: {}", Error::last_os_error());
        }
    }
}
