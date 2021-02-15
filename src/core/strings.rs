use crate::*;

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

    /// Returns a String at the given _index_
    pub fn get_string(&self, index: u8) -> Option<String> {
        let index_usize = index as usize;

        if index_usize == 0 || index_usize > self.strings.len() {
            // BIOS strings are 1 based indexing, ignore bad input
            return None;
        }

        // TODO: "*x as char" is not ISO-8859-1.  This should be made ISO-8859-1.
        Some(
            self.strings[index_usize - 1]
                .iter()
                .map(|x| *x as char)
                .collect(),
        )
    }
}

impl Iterator for Strings {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_string_index == self.strings.len() {
            self.reset();
            return None;
        }

        // TODO: "*x as char" is not ISO-8859-1.  This should be made ISO-8859-1.
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
