use serde::{ser::SerializeSeq, Serialize, Serializer};
use std::fmt;

/// # SMBIOS Strings
///
/// The strings part/section of a structure
pub struct Strings {
    strings: Vec<Vec<u8>>,
    current_string_index: usize,
}

impl Strings {
    /// Creates a new strings section of a structure
    pub fn new(string_area: Vec<u8>) -> Strings {
        Strings {
            strings: {
                if string_area == &[] {
                    vec![]
                } else {
                    string_area
                        .split(|num| *num == 0)
                        .into_iter()
                        .map(|string_slice| string_slice.to_vec())
                        .collect()
                }
            },
            current_string_index: 0,
        }
    }

    fn reset(&mut self) {
        self.current_string_index = 0;
    }

    /// Returns a [String] at the given `index`
    ///
    /// BIOS strings are 1 based indexing
    pub fn get_string(&self, index: u8) -> Option<String> {
        let index_usize = index as usize;

        if index_usize == 0 || index_usize > self.strings.len() {
            // BIOS strings are 1 based indexing, ignore bad input
            return None;
        }

        // Create an ISO-8859-1 String.  Each `u8 as char` operation maps a u8
        // value (0xNN) to a Unicode code point (0x00NN).
        //
        // SMBIOS specification does not state that a BIOS string is ISO-8859-1 or
        // ASCII (or anything else).  The reason it is important to use ISO-8859-1
        // is that every u8 value (0-255) is represented and mapped 1:1 with a Unicode
        // value.  Therefore, it is possible to reverse the process, starting from a
        // Rust String or str and produce the original u8 array of values.
        //
        // Presently there is no need to convert back to a u8 array. If there were,
        // the Rust char functions len_utf8() and encode_utf8() can be used.  If len_utf8()
        // == 2 then the original u8 can be arrived at by combining bits from the two bytes.
        Some(
            self.strings[index_usize - 1]
                .iter()
                .map(|x| *x as char)
                .collect(),
        )
    }

    /// Iterates the raw bytes of the strings. The terminating 0 is not included in each string.
    pub fn iter(&self) -> std::slice::Iter<'_, Vec<u8>> {
        self.strings.iter()
    }
}

impl Iterator for Strings {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_string_index == self.strings.len() {
            self.reset();
            return None;
        }

        // "*x as char" is ISO-8859-1.
        let result: String = self.strings[self.current_string_index]
            .iter()
            .map(|x| *x as char)
            .collect();
        self.current_string_index = self.current_string_index + 1;

        Some(result)
    }
}

impl IntoIterator for &Strings {
    type Item = String;
    type IntoIter = Strings;

    fn into_iter(self) -> Self::IntoIter {
        Strings {
            strings: self.strings.clone(),
            current_string_index: 0,
        }
    }
}

impl fmt::Debug for Strings {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_list().entries(self.into_iter()).finish()
    }
}

impl Serialize for Strings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.count()))?;
        for e in self {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
