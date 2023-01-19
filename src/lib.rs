use memalloc::allocate;
use std::ffi::{CString, OsStr, OsString};
use std::os::windows::ffi::OsStrExt;
use std::{ptr, slice};
use winapi::ctypes::c_void;
use winapi::shared::minwindef::{BOOL, DWORD, FALSE, LPBYTE};
use winapi::um::winspool::{
    ClosePrinter, EndDocPrinter, EndPagePrinter, EnumPrintersW, OpenPrinterW, StartDocPrinterW,
    StartPagePrinter, WritePrinter, DOC_INFO_1W, PPRINTER_INFO_4W, PRINTER_ENUM_LOCAL,
    PRINTER_INFO_4W,
};

pub fn open_printer(printer_name: String) -> Option<*mut c_void> {
    let mut handle = ptr::null_mut();

    let name = OsString::from(printer_name)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>();

    match unsafe { OpenPrinterW(name.as_ptr() as *mut _, &mut handle, ptr::null_mut()) } {
        FALSE => None,
        _ => Some(handle),
    }
}

pub fn start_doc_printer(handle: *mut c_void) -> DWORD {
    let doc_name = OsStr::new("RAW Document")
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>();

    // https://learn.microsoft.com/en-us/windows-hardware/drivers/print/raw-data-type
    // EMF, RAW, TEXT, PSCRIPT1
    let data_type = OsStr::new("RAW")
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>();

    let info = DOC_INFO_1W {
        pDocName: doc_name.as_ptr() as *mut _,
        pOutputFile: ptr::null_mut(),

        pDatatype: data_type.as_ptr() as *mut _,
    };

    unsafe { StartDocPrinterW(handle, 1, &info as *const DOC_INFO_1W as LPBYTE) }
}

pub fn start_page_printer(handle: *mut c_void) -> BOOL {
    unsafe { StartPagePrinter(handle) }
}

pub fn write_printer(handle: *mut c_void, data: CString) {
    let mut written = 0;

    let buf = data.as_bytes_with_nul();

    unsafe { WritePrinter(handle, buf.as_ptr() as *mut _, buf.len() as _, &mut written) };

    println!("written: {}", written);
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
