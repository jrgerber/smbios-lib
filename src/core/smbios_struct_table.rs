use crate::*;
use std::cmp::Ordering;
use std::fs::read;
use std::io::Error;

/// # SMBIOS Struture Table
///
/// Contains an array of SMBIOS structures.
pub struct SMBiosStructTable {
    data: Vec<u8>,
    /// Version of the contained SMBIOS structures.
    pub version: Option<SMBiosVersion>,
}

impl SMBiosStructTable {
    /// Creates an SMBIOS table parser which can be iterated
    ///
    /// `data` is a block of bytes representing the raw table data.
    /// `version` is optional and represents the DMTF SMBIOS Standard version of the bytes in `data`.
    pub fn from_vec_and_version(data: Vec<u8>, version: Option<SMBiosVersion>) -> Self {
        Self { data, version }
    }

    /// Loads raw SMBios table data from a file
    pub fn try_load_from_file(
        filename: &str,
        version: Option<SMBiosVersion>,
    ) -> Result<SMBiosStructTable, Error> {
        let data = read(filename)?;
        let result = Self { data, version };
        Ok(result)
    }
}

impl<'a> IntoIterator for &'a SMBiosStructTable {
    type Item = SMBiosStructParts<'a>;

    type IntoIter = RawStructIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        RawStructIterator::new(self.data.as_slice())
    }
}

impl<'a> fmt::Debug for SMBiosStructTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: format as an array, make a function on SMBiosStructParts to return an enum of variants of the struct types
        self.into_iter().map(|x| writeln!(f, "{:?}", x)).collect()
    }
}

/// # Iterator of [SMBiosStructTable]
///
/// Allows iteration of [SMBiosStructTable] and returns [SMBiosStructParts].
pub struct RawStructIterator<'a> {
    data: &'a [u8],
    current_index: usize,
}

impl<'a> RawStructIterator<'a> {
    /// Creates an instance of this iterator
    pub fn new(data: &'a [u8]) -> Self {
        RawStructIterator {
            data: data,
            current_index: 0,
        }
    }
}

impl<'a> Iterator for RawStructIterator<'a> {
    type Item = SMBiosStructParts<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_index = self.current_index;
        let len = self.data.len();

        // We are done iterating if current_index points beyond the end of "data".
        if next_index >= len {
            self.current_index = 0;
            return None;
        }

        // When calling "next()" the first time, ensure "data" is valid before attempting iteration.
        // A valid structure has:
        // - At least 6 bytes.  A header of 4 bytes plus the terminating two bytes (\0\0) in the string area.
        // - The second byte indicates the structure length (header plus structure data).
        //   The length does not include the string area (which at a minimum the last two bytes of zero)
        // - The last two bytes are 0 (the end of the string area)
        if next_index == 0
            && (len < Header::SIZE + 2 // struct is too short
            || (self.data[next_index + 1] as usize) > len - 2 // struct header specifies a length too long
            || self.data[len - 2] != 0 // 2nd to last byte should be zero and it is not
            || self.data[len - 1] != 0)
        {
            // Last byte should be zero and it is not
            return None;
        }

        // next_index is pointing at the start of the structure header.
        // Read the struct header length at offset 1 of the header (next_index + 1) and advance to the
        // string area which follows the stucture.
        next_index = next_index + self.data[next_index + 1] as usize;

        // next_index is pointing at the start of the string area.
        // The string area is terminated with \0\0.  If no strings exist then its contents is \0\0.
        // Search for \0\0 and point at the byte immediately after it.  That point is either the start of the
        // next structure header or one byte beyond the end of "data".
        let mut a: bool;
        let mut b = true;
        loop {
            a = self.data[next_index] != 0;
            next_index = next_index + 1;
            if a || b {
                b = self.data[next_index] != 0;
                next_index = next_index + 1;
            }
            if !(a || b) {
                break;
            }
        }

        let previous_index = self.current_index;
        self.current_index = next_index;

        match self.data.get(previous_index..self.current_index) {
            Some(val) => Some(SMBiosStructParts::new(val)),
            None => None,
        }
    }
}

/// # Version of SMBIOS Structure
#[derive(Debug, Eq, PartialEq)]
pub struct SMBiosVersion {
    /// SMBIOS major version
    pub major: u8,
    /// SMBIOS minor version
    pub minor: u8,
    /// SMBIOS version revision
    pub revision: u8,
}

impl Ord for SMBiosVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.major < other.major {
            Ordering::Less
        } else if self.major > other.major {
            Ordering::Greater
        } else if self.minor < other.minor {
            Ordering::Less
        } else if self.minor > other.minor {
            Ordering::Greater
        } else if self.revision < other.revision {
            Ordering::Less
        } else if self.revision > other.revision {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for SMBiosVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
