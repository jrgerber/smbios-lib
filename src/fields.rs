use std::convert::TryInto;
use std::fmt;

pub fn get_field_byte(offset:usize, data:&[u8]) -> Option<u8> {    
    match data.get(offset .. offset + 1) {
        Some(val) => Some(val[0]),
        None => None,
    }
}

pub fn get_field_word(offset:usize, data:&[u8]) -> Option<u16> {    
    match data.get(offset .. offset + 2) {
        Some(val) => Some(u16::from_le_bytes(val.try_into().expect("array length does not match type width"))),
        None => None,
    }    
}

pub fn get_field_dword(offset:usize, data:&[u8]) -> Option<u32> {    
    match data.get(offset .. offset + 4) {
        Some(val) => Some(u32::from_le_bytes(val.try_into().expect("array length does not match type width"))),
        None => None,
    }    
}

pub fn get_field_qword(offset:usize, data:&[u8]) -> Option<u64> {    
    match data.get(offset .. offset + 8) {
        Some(val) => Some(u64::from_le_bytes(val.try_into().expect("array length does not match type width"))),
        None => None,
    }    
}