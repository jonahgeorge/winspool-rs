use memalloc::allocate;
use std::ffi::{CString, OsStr};
use std::os::windows::ffi::OsStrExt;
use std::{ptr, slice};
use winapi::ctypes::c_void;
use winapi::shared::minwindef::{BOOL, DWORD, FALSE, LPBYTE};
use winapi::um::winspool::{
    ClosePrinter, EndDocPrinter, EndPagePrinter, EnumPrintersW, OpenPrinterW, StartDocPrinterW,
    StartPagePrinter, WritePrinter, DOC_INFO_1W, PPRINTER_INFO_4W, PRINTER_ENUM_LOCAL,
    PRINTER_INFO_4W,
};

trait ToU16VecWithNulTerminator {
    fn to_u16_vec_with_nul_terminator(&self) -> Vec<u16>;
}

impl ToU16VecWithNulTerminator for str {
    fn to_u16_vec_with_nul_terminator(&self) -> Vec<u16> {
        OsStr::new(self)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<_>>()
    }
}

pub fn open_printer(printer_name: &str) -> Option<*mut c_void> {
    let mut handle = ptr::null_mut();

    match unsafe {
        OpenPrinterW(
            printer_name.to_u16_vec_with_nul_terminator().as_ptr() as *mut _,
            &mut handle,
            ptr::null_mut(),
        )
    } {
        FALSE => None,
        _ => Some(handle),
    }
}

// https://learn.microsoft.com/en-us/windows-hardware/drivers/print/raw-data-type
// EMF, RAW, TEXT, PSCRIPT1
pub fn start_doc_printer(handle: *mut c_void, document_name: &str, data_type: &str) -> DWORD {
    let info = DOC_INFO_1W {
        pOutputFile: ptr::null_mut(),
        pDocName: document_name.to_u16_vec_with_nul_terminator().as_ptr() as *mut _,
        pDatatype: data_type.to_u16_vec_with_nul_terminator().as_ptr() as *mut _,
    };

    unsafe { StartDocPrinterW(handle, 1, &info as *const DOC_INFO_1W as LPBYTE) }
}

pub fn start_page_printer(handle: *mut c_void) -> BOOL {
    unsafe { StartPagePrinter(handle) }
}

pub fn write_printer(handle: *mut c_void, data: &str) -> BOOL {
    let mut written = 0;
    let cdat = CString::new(data).unwrap();
    let buf = cdat.as_bytes_with_nul();

    let res = unsafe { WritePrinter(handle, buf.as_ptr() as *mut _, buf.len() as _, &mut written) };

    println!("written: {}", written);

    res
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

    let buffer = unsafe { allocate(bytes_needed as _) };

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
    let printers = unsafe { slice::from_raw_parts(printer_info, count_returned as _) };

    Some(printers.to_vec())
}
