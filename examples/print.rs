use std::io::Error;
use std::{thread, time};
use winspool::{
    close_printer, end_doc_printer, end_page_printer, open_printer, start_doc_printer,
    start_page_printer, write_printer,
};

const receipt: &str = r#"
Hello world.

This took way to long to figure out :)


"#;

// const printer_name: &str = "NPI642118 (HP LaserJet M110w)";
const printer_name: &str = "EPSON TM-P20 Receipt";

fn main() {
    match open_printer(printer_name) {
        Some(handle) => match start_doc_printer(handle, "My Document", "RAW") {
            0 => {
                println!("start_doc_printer: {}", Error::last_os_error());
                close_printer(handle);
            }
            _job_id => match start_page_printer(handle) {
                0 => {
                    println!("start_page_printer: {}", Error::last_os_error());
                    end_doc_printer(handle);
                    close_printer(handle);
                }
                _ => {
                    match write_printer(handle, receipt) {
                        0 => {
                            println!("write_printer: {}", Error::last_os_error());
                        }
                        _ => {
                            // Sleep for a bit so that the printer isn't terminated while printing.
                            // TODO: Use FlushPrinter instead or poll for the job to be complete?
                            thread::sleep(time::Duration::from_millis(1000));
                        }
                    }

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
