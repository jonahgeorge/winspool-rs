use winspool::list_printers;

fn main() {
    let printers = list_printers().expect("failed to list printers");

    for p in printers {
        let printer_name =
            unsafe { widestring::WideCString::from_ptr_str(p.pPrinterName).to_string_lossy() };
        println!("{}", printer_name);
    }
}
