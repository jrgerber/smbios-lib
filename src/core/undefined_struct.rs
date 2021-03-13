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
        match raw.get(Header::LENGTH_OFFSET) {
            Some(&header_length) => UndefinedStruct {
                header: Header::new(raw[..Header::SIZE].try_into().expect("4 bytes")),
                fields: raw.get(..(header_length as usize)).unwrap_or(&[]).to_vec(),
                strings: {
                    Strings::new(
                        raw.get((header_length as usize)..raw.len() - 2)
                            .unwrap_or(&[])
                            .to_vec(),
                    )
                },
            },
            None => UndefinedStruct {
                ..Default::default()
            },
        }
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
        let v: [u8; 4] = [0; 4];
        UndefinedStruct {
            header: Header::new(v),
            fields: (&[]).to_vec(),
            strings: { Strings::new((&[]).to_vec()) },
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

    /// An iterator over the defined type instances within the table.
    pub fn defined_struct_iter<T>(&'a self) -> impl Iterator<Item = T> + 'a
    where
        T: SMBiosStruct<'a>,
    {
        self.iter()
            .take_while(|undefined_struct| {
                undefined_struct.header.struct_type() != SMBiosEndOfTable::STRUCT_TYPE
            })
            .filter_map(|undefined_struct| {
                if undefined_struct.header.struct_type() == T::STRUCT_TYPE {
                    Some(T::new(undefined_struct))
                } else {
                    None
                }
            })
    }

    /// Tests if every element of the defined struct iterator matches a predicate.
    pub fn all<T, F>(&'a self, f: F) -> bool
    where
        T: SMBiosStruct<'a>,
        F: FnMut(T) -> bool,
    {
        self.defined_struct_iter().all(f)
    }

    /// Tests if any element of the defined struct iterator matches a predicate.
    pub fn any<T, F>(&'a self, f: F) -> bool
    where
        T: SMBiosStruct<'a>,
        F: FnMut(T) -> bool,
    {
        self.defined_struct_iter().any(f)
    }

    /// Finds the first occurance of the structure
    pub fn first<T>(&'a self) -> Option<T>
    where
        T: SMBiosStruct<'a>,
    {
        self.defined_struct_iter().next()
    }

    /// Finds the first occurance of the structure that satisfies a predicate.
    pub fn find<T, P>(&'a self, predicate: P) -> Option<T>
    where
        T: SMBiosStruct<'a>,
        P: FnMut(&T) -> bool,
    {
        self.defined_struct_iter().find(predicate)
    }

    /// Applies function to the defined struct elements and returns the first non-none result.
    pub fn find_map<A, B, F>(&'a self, f: F) -> Option<B>
    where
        A: SMBiosStruct<'a>,
        F: FnMut(A) -> Option<B>,
    {
        self.defined_struct_iter().find_map(f)
    }

    /// Creates an iterator of the defined structure which uses a closure to determine if an element should be yielded.
    pub fn filter<T: 'a, P: 'a>(&'a self, predicate: P) -> impl Iterator<Item = T> + 'a
    where
        T: SMBiosStruct<'a>,
        P: FnMut(&T) -> bool,
    {
        self.defined_struct_iter().filter(predicate)
    }

    /// Maps the defined struct to another type given by the closure.
    pub fn map<A: 'a, B, F: 'a>(&'a self, f: F) -> impl Iterator<Item = B> + 'a
    where
        A: SMBiosStruct<'a>,
        F: FnMut(A) -> B,
    {
        self.defined_struct_iter().map(f)
    }

    /// Creates an iterator that both filters and maps from the defined struct iterator.
    pub fn filter_map<A: 'a, B, F: 'a>(&'a self, f: F) -> impl Iterator<Item = B> + 'a
    where
        A: SMBiosStruct<'a>,
        F: FnMut(A) -> Option<B>,
    {
        self.defined_struct_iter().filter_map(f)
    }

    /// Finds the structure matching the given handle
    ///
    /// To downcast to the defined struct, call .defined_struct() on the result.
    pub fn find_by_handle(&'a self, handle: &Handle) -> Option<&'a UndefinedStruct> {
        self.iter()
            .find(|smbios_struct| smbios_struct.header.handle() == *handle)
            .and_then(|undefined_struct| Some(undefined_struct))
    }

    /// Returns all occurances of the structure
    pub fn collect<T>(&'a self) -> Vec<T>
    where
        T: SMBiosStruct<'a>,
    {
        self.defined_struct_iter().collect()
    }
}

impl From<Vec<u8>> for UndefinedStructTable {
    fn from(data: Vec<u8>) -> Self {
        const DOUBLE_ZERO_SIZE: usize = 2usize;
        const MIN_STRUCT_SIZE: usize = Header::SIZE + DOUBLE_ZERO_SIZE;
        let mut result = Self::new();
        let mut current_index = 0usize;

        loop {
            // Is the next structure long enough?
            match data.get(current_index..current_index + MIN_STRUCT_SIZE) {
                Some(min_struct) => {
                    // Read the structure's self-reported length in its header
                    let struct_len = min_struct[Header::LENGTH_OFFSET] as usize;

                    // Bad reported length
                    if struct_len < Header::SIZE {
                        break;
                    }

                    // Beyond the structure length are the structure's strings
                    // Find the /0/0 which marks the end of this structure and the
                    // beginning of the next.
                    match data.get(current_index + struct_len..) {
                        Some(strings_etc) => {
                            match strings_etc
                                .windows(DOUBLE_ZERO_SIZE)
                                .position(|x| x[0] == x[1] && x[1] == 0)
                            {
                                Some(double_zero_position) => {
                                    // The next structure will start at this index
                                    let next_index = current_index
                                        + struct_len
                                        + double_zero_position
                                        + DOUBLE_ZERO_SIZE;

                                    // Copy the current structure to the collection
                                    result.add(UndefinedStruct::new(
                                        &data[current_index..next_index].to_vec(),
                                    ));
                                    current_index = next_index;
                                }
                                None => break,
                            }
                        }
                        None => break,
                    };
                }
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
