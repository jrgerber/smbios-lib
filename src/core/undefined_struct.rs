use std::{convert::TryInto, slice::Iter};

use crate::*;

/// # Embodies the three basic parts of an SMBIOS structure
///
/// Every SMBIOS structure contains three distinct sections:
/// - A header
/// - A formatted structure of fields (offsets and widths)
/// - String data
///
/// A consumer of BIOS data ultimately wants to work with a [DefinedStruct].
/// [UndefinedStruct] provides a set of fields and functions that enables
/// downcasting to a [DefinedStruct].  Further, the OEM is allowed to define
/// their own structures and in such cases working with UndefinedStruct is
/// necessary.  Therefore, [UndefinedStruct] is public for the case of OEM,
/// as well as when working with structures that are defined in an SMBIOS
/// standard newer than the one this library currently supports.
pub struct UndefinedStruct {
    /// The [Header] of the structure
    pub header: Header,

    /// The raw data for the header and fields
    ///
    /// `fields` is used by the `get_field_*()` functions. `fields` does not
    /// include _strings_; therefore, preventing accidentally retrieving
    /// data from the _strings_ area.  This avoids a need to check
    /// `header.length()` during field retrieval.
    ///
    /// Note: A better design is for this to only hold the fields, however,
    /// that will shift field offsets given in code by 4 (the header size).
    /// The SMBIOS specification gives offsets relative to the start of the
    /// header, and therefore maintaining this library code is easier to
    /// keep the header.
    ///
    /// An alternative would be to make the `get_field_*()` functions adjust
    /// for the header offset though this adds a small cost to every field
    /// retrieval in comparison to just keeping an extra 4 bytes for every
    /// structure.
    pub fields: Vec<u8>,

    /// The strings of the structure
    pub strings: Strings,
}

impl<'a> UndefinedStruct {
    /// Creates a structure instance of the given byte array slice
    pub fn new(raw: &Vec<u8>) -> Self {
        let header_length = UndefinedStruct::header_length(raw);
        match header_length {
            0 => UndefinedStruct{..Default::default()},
            _ =>
                UndefinedStruct {
                    header: Header::new(raw[..Header::SIZE].try_into().expect("4 bytes")),
                    fields: raw.get(..header_length).unwrap_or(&[]).to_vec(),
                    strings: {
                        Strings::new(
                            raw.get(header_length..raw.len() - 2)
                                .unwrap_or(&[])
                                .to_vec(),
                        )
                    },
                }
        }
    }

    fn header_length(raw: &Vec<u8>) -> usize {
        raw.get(Header::LENGTH_OFFSET..Header::LENGTH_OFFSET + 1)
            .unwrap_or(&[0])[0] as usize
    }

    /// Retrieve a byte at the given offset from the structure's data section
    pub fn get_field_byte(&self, offset: usize) -> Option<u8> {
        match self.fields.get(offset..offset + 1) {
            Some(val) => Some(val[0]),
            None => None,
        }
    }

    /// Retrieve a WORD at the given offset from the structure's data section
    pub fn get_field_word(&self, offset: usize) -> Option<u16> {
        match self.fields.get(offset..offset + 2) {
            Some(val) => Some(u16::from_le_bytes(val.try_into().expect("u16 is 2 bytes"))),
            None => None,
        }
    }

    /// Retrieve a [Handle] at the given offset from the structure's data section
    pub fn get_field_handle(&self, offset: usize) -> Option<Handle> {
        match self.fields.get(offset..offset + Handle::SIZE) {
            Some(val) => Some(Handle(u16::from_le_bytes(
                val.try_into().expect("u16 is 2 bytes"),
            ))),
            None => None,
        }
    }

    /// Retrieve a DWORD at the given offset from the structure's data section
    pub fn get_field_dword(&self, offset: usize) -> Option<u32> {
        match self.fields.get(offset..offset + 4) {
            Some(val) => Some(u32::from_le_bytes(val.try_into().expect("u32 is 4 bytes"))),
            None => None,
        }
    }

    /// Retrieve a QWORD at the given offset from the structure's data section
    pub fn get_field_qword(&self, offset: usize) -> Option<u64> {
        match self.fields.get(offset..offset + 8) {
            Some(val) => Some(u64::from_le_bytes(val.try_into().expect("u64 is 8 bytes"))),
            None => None,
        }
    }

    /// Retrieve a String of the given offset
    ///
    /// Retrieval of strings is a two part operation. The given offset
    /// contains a byte whose value is a 1 based index into the strings section.
    /// The string is thus retrieved from the strings section based on the
    /// byte value at the given offset.
    pub fn get_field_string(&self, offset: usize) -> Option<String> {
        match self.get_field_byte(offset) {
            Some(val) => self.strings.get_string(val),
            None => None,
        }
    }

    // todo: learn how to pass an index range (SliceIndex?) rather than start/end indices.
    // This would better conform to the Rust design look and feel.

    /// Retrieve a block of bytes from the structure's data section
    pub fn get_field_data(&self, start_index: usize, end_index: usize) -> Option<&[u8]> {
        return self.fields.get(start_index..end_index);
    }

    /// Cast to a given structure
    ///
    /// When this library does not contain a [DefinedStruct] variant
    /// matching the SMBiosStruct::STRUCT_TYPE, this function affords a cast to the
    /// given type. Such would be the case with OEM structure type T
    /// (which implements the [SMBiosStruct] trait).
    ///
    /// TODO: This should panic (not be Option) when the STRUCT_TYPE does not match because
    /// this would be a logic error in code, not a runtime error.
    ///
    /// Make this a "try_into"
    pub fn as_type<T: SMBiosStruct<'a>>(&'a self) -> Option<T> {
        if T::STRUCT_TYPE == self.header.struct_type() {
            Some(T::new(self))
        } else {
            None
        }
    }

    /// Down casts the current structure to its specific defined BIOS structure type
    pub fn defined_struct(&self) -> DefinedStruct<'_> {
        self.into()
    }
}

impl fmt::Debug for UndefinedStruct {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fields = &self.fields[Header::SIZE..];
        fmt.debug_struct(std::any::type_name::<UndefinedStruct>())
            .field("header", &self.header)
            .field("fields", &fields)
            .field("strings", &self.strings)
            .finish()
    }
}

impl Default for UndefinedStruct {
    fn default() -> Self {
        let v : [u8; 4] = [0; 4];
        UndefinedStruct {
            header: Header::new(v),
            fields: (&[]).to_vec(),
            strings: {
                Strings::new((&[]).to_vec())
            }
        }
    }
}

/// # Undefined Struct Table
///
/// A collection of [UndefinedStruct] items.
#[derive(Debug)]
pub struct UndefinedStructTable(Vec<UndefinedStruct>);

impl<'a> UndefinedStructTable {
    fn new() -> UndefinedStructTable {
        UndefinedStructTable(Vec::new())
    }

    fn add(&mut self, elem: UndefinedStruct) {
        self.0.push(elem);
    }

    /// Iterator of the contained [UndefinedStruct] items.
    pub fn iter(&self) -> Iter<'_, UndefinedStruct> {
        self.0.iter()
    }

    /// Finds the first occurance of the structure
    pub fn find_first<T>(&'a self) -> Option<T>
    where
        T: SMBiosStruct<'a>,
    {
        self.iter()
            .find(|smbios_struct| smbios_struct.header.struct_type() == T::STRUCT_TYPE)
            .and_then(|undefined_struct| Some(T::new(&undefined_struct)))
    }

    /// Finds the structure matching the given handle
    pub fn find_by_handle(&'a self, handle: &Handle) -> Option<&'a UndefinedStruct> {
        self.iter()
            .find(|smbios_struct| smbios_struct.header.handle() == *handle)
            .and_then(|undefined_struct| Some(undefined_struct))
    }

    /// Finds all occurances of the structure
    pub fn find_all<T>(&'a self) -> Vec<T>
    where
        T: SMBiosStruct<'a>,
    {
        self.iter()
            .filter_map(|smbios_struct| {
                if smbios_struct.header.struct_type() == T::STRUCT_TYPE {
                    Some(T::new(smbios_struct))
                } else {
                    None
                }
            })
            .collect()
    }
}

impl From<Vec<u8>> for UndefinedStructTable {
    fn from(data: Vec<u8>) -> Self {
        let mut result = Self::new();
        let mut current_index = 0usize;
        let len = data.len();

        loop {
            let mut next_index = current_index;

            // We are done iterating if current_index points beyond the end of "data".
            if next_index >= len {
                return result;
            }

            // A valid structure has:
            // - At least 6 bytes.  A header of 4 bytes plus the terminating two bytes (\0\0) in the string area.
            // - The second byte indicates the structure length (header plus structure data).
            //   The length does not include the string area (which at a minimum the last two bytes of zero)
            // - The last two bytes are 0 (the end of the string area)
            if len < Header::SIZE + 2 // struct is too short
            || (data[next_index + Header::LENGTH_OFFSET] as usize) > len - 2 // struct header specifies a length too long
            || data[len - 2] != 0 // 2nd to last byte should be zero and it is not
            || data[len - 1] != 0
            // Last byte should be zero and it is not
            {
                return result;
            }

            // next_index is pointing at the start of the structure header.
            // Read the struct header length at offset 1 of the header (next_index + 1) and advance to the
            // string area which follows the stucture.
            next_index += data[current_index + Header::LENGTH_OFFSET] as usize;

            // next_index is pointing at the start of the string area.
            // The string area is terminated with \0\0.  If no strings exist then its contents is \0\0.
            // Search for \0\0 and point at the byte immediately after it.  That point is either the start of the
            // next structure header or one byte beyond the end of "data".
            let mut a: bool;
            let mut b = true;
            loop {
                if next_index >= len {
                    break;
                }
                a = data[next_index] != 0;
                next_index = next_index + 1;
                if a || b {
                    b = data[next_index] != 0;
                    next_index = next_index + 1;
                }
                if !(a || b) {
                    break;
                }
            }

            let previous_index = current_index;
            current_index = next_index;

            match data.get(previous_index..current_index) {
                Some(val) => result.add(UndefinedStruct::new(&val.to_vec())),
                None => break,
            }
        }

        result
    }
}

impl IntoIterator for UndefinedStructTable {
    type Item = UndefinedStruct;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
