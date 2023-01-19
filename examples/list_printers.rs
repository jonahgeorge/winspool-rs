use winspool::list_printers;

fn main() {
    let printers = list_printers().expect("failed to list printers");

    for p in printers {
        let w_printer_name = unsafe { widestring::WideCString::from_ptr_str(p.pPrinterName) };
        println!("{}", w_printer_name.to_string_lossy(),);
    }
}
