#![deny(clippy::all)]

use pdf_extract::{extract_text_from_mem, extract_text_from_mem_encrypted};

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub fn get_pdf_text(file_data: &[u8]) -> String {
  // 将接收到的二进制数据转换为 Vec<u8>
  let pdf_data = file_data.to_vec();
  let data = extract_text_from_mem(&pdf_data).unwrap();
  return data;
}

#[napi]
pub fn get_pdf_pw_text(file_data: &[u8], password: String) -> String {
  // 将接收到的二进制数据转换为 Vec<u8>
  let pdf_data = file_data.to_vec();
  let data = extract_text_from_mem_encrypted(&pdf_data, password).unwrap();
  return data;
}
