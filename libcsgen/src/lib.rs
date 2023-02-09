use std::ffi::{CStr, CString, c_char};

use operations::{service, repository, sql_insert, db_enum};

extern crate libc;

pub mod operations;
pub mod error;
mod helpers;

#[no_mangle]
pub extern "C" fn gen_service(service_name: *const c_char) -> *const c_char{
    let name = unsafe {to_rust_str(service_name)};
    let result = service::generate(name).unwrap();
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn gen_repository(repository_name: *const c_char, dbo_name: *const c_char) -> *const c_char{
    let name = unsafe {to_rust_str(repository_name)};
    let dbname = unsafe {to_rust_str(dbo_name)};
    let result = repository::generate(&name, dbname).unwrap();
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn gen_sql_insert_copy(table_csv: *const c_char, is_identity: bool) -> *const c_char{
    let tcsv = unsafe {to_rust_str(table_csv)};
    let result = sql_insert::generate_copy(&tcsv, is_identity).unwrap();
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn gen_sql_insert_empty(table_csv: *const c_char, is_identity: bool) -> *const c_char{
    let tcsv = unsafe {to_rust_str(table_csv)};
    let result = sql_insert::generate_empty(&tcsv, is_identity).unwrap();
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn gen_db_enum_code(enum_name: *const c_char, table_csv: *const c_char, skip_col: bool, extra_docs: bool) -> *const c_char{
    let ename = unsafe {to_rust_str(enum_name)};
    let tcsv = unsafe {to_rust_str(table_csv)};
    let result = db_enum::generate(&ename, &tcsv, skip_col, extra_docs).unwrap();
    CString::new(result).unwrap().into_raw()
}


unsafe fn to_rust_str<'a>(c_buf: *const c_char) -> &'a str{
    let c_str = CStr::from_ptr(c_buf);
    c_str.to_str().unwrap()
} 
