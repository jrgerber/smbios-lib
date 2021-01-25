use crate::*;

/// # SMBIOS Raw Table Data
///
/// Contains the raw data of BIOS and provides iteration of
/// the structures contained within the raw data.
pub struct SMBiosTableData {
    data: Vec<u8>,
}

impl SMBiosTableData {
    /// Creates a wrapper around raw SMBIOS data
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// Loads raw SMBios table data and return [Self] or [io:Error]
    pub fn from_file(filename: &str) -> Result<Self, io::Error> {
        // TODO: implement a fn that checks whether the structure is valid table data.
        // If it's not return that error here.
        let data = fs::read(filename)?;
        let result = Self { data };
        Ok(result)
    }
}

impl<'a> IntoIterator for &'a SMBiosTableData {
    type Item = SMBiosStructParts<'a>;

    type IntoIter = RawStructIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        RawStructIterator::new(self.data.as_slice())
    }
}

impl<'a> fmt::Debug for SMBiosTableData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: format as an array, make a function on SMBiosStructParts to return an enum of variants of the struct types
        self.into_iter().map(|x| writeln!(f, "{:?}", x)).collect()
    }
}

/// # Iterator of [SMBiosTableData]
///
/// Allows iteration of [SMBiosTableData] and returns [SMBiosStructParts].
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
