use memalloc::allocate;
use std::ffi::OsString;
use std::io::Error;
use std::os::windows::prelude::*;
use std::ptr;
use std::slice;
use std::str::FromStr;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::BOOL;
use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::FALSE;
use winapi::um::winspool::{
    ClosePrinter, EndDocPrinter, EndPagePrinter, EnumPrintersW, OpenPrinterW, StartDocPrinterW,
    StartPagePrinter, WritePrinter, DOC_INFO_1W, PPRINTER_INFO_4W, PRINTER_ENUM_LOCAL,
    PRINTER_INFO_4W,
};

pub fn open_printer(printer_name: &str) -> Option<*mut c_void> {
    let mut handle = ptr::null_mut();

    let name = OsString::from_str(printer_name)
        .unwrap()
        .encode_wide()
        .collect::<Vec<_>>()
        .as_mut_ptr();

    match unsafe { OpenPrinterW(name, &mut handle, ptr::null_mut()) } {
        FALSE => {
            println!("{}", Error::last_os_error());

            None
        }
        _ => Some(handle),
    }
}

pub fn start_doc_printer(handle: *mut c_void, doc_name: &str) -> DWORD {
    let info = DOC_INFO_1W {
        pDocName: OsString::from_str(doc_name)
            .unwrap()
            .encode_wide()
            .collect::<Vec<_>>()
            .as_mut_ptr(),

        pOutputFile: ptr::null_mut(),

        // https://learn.microsoft.com/en-us/windows-hardware/drivers/print/raw-data-type
        // EMF, RAW, TEXT, PSCRIPT1
        pDatatype: OsString::from_str("EMF")
            .unwrap()
            .encode_wide()
            .collect::<Vec<_>>()
            .as_mut_ptr(),
    };

    let b = unsafe {
        std::slice::from_raw_parts_mut(
            &info as *const DOC_INFO_1W as *mut u8,
            std::mem::size_of::<DOC_INFO_1W>(),
        )
    };

    unsafe { StartDocPrinterW(handle, 1, b.as_mut_ptr()) }
}

pub fn start_page_printer(handle: *mut c_void) -> BOOL {
    unsafe { StartPagePrinter(handle) }
}

pub fn write_printer(handle: *mut c_void, data: &str) {
    let mut written = 0;

    let write = unsafe {
        WritePrinter(
            handle,
            data.as_ptr() as *mut c_void,
            data.len() as DWORD,
            &mut written,
        )
    };
}

pub fn close_printer(handle: *mut c_void) -> BOOL {
    unsafe { ClosePrinter(handle) }
}

pub fn end_doc_printer(handle: *mut c_void) {
    unsafe { EndDocPrinter(handle) };
}

pub fn end_page_printer(handle: *mut c_void) {
    unsafe { EndPagePrinter(handle) };
}

pub fn list_printers() -> Option<Vec<PRINTER_INFO_4W>> {
    let flags = PRINTER_ENUM_LOCAL;
    let level = 4; // PPRINTER_INFO_4W

    let mut bytes_needed = 0;
    let mut count_returned = 0;

    let mut result = unsafe {
        EnumPrintersW(
            flags,
            ptr::null_mut(),
            level,
            ptr::null_mut(),
            0,
            &mut bytes_needed,
            &mut count_returned,
        )
    };

    if result != 0 {
        return None;
    }

    let buffer = unsafe { allocate(bytes_needed as usize) };

    result = unsafe {
        EnumPrintersW(
            flags,
            ptr::null_mut(),
            level,
            buffer,
            bytes_needed,
            &mut bytes_needed,
            &mut count_returned,
        )
    };

    if result == 0 {
        return None;
    }

    let printer_info = buffer as PPRINTER_INFO_4W;
    let printers = unsafe { slice::from_raw_parts(printer_info, count_returned as usize) };

    Some(printers.to_vec())
}
