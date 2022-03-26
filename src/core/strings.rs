use serde::{ser::SerializeSeq, Serialize, Serializer};
use std::error;
use std::{fmt, string::FromUtf8Error};

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

    /// Returns a UTF-8 [String] at the given 1 based `index`
    ///
    /// If the index is 0 an empty string "" is returned.
    /// If SMBiosStringError::InvalidStringNumber is returned, either the field value is corrupt or the string-set is corrupt.
    /// If SMBiosStringError::Utf8 is returned, the string is corrupt.
    pub fn get_string(&self, index: u8) -> Result<String, SMBiosStringError> {
        let index_usize = index as usize;

        // As of 3.5.0 DMTF has decided to make UTF-8 the standard for how to interpret strings.
        //
        // section 6.1.3:
        // "Strings must be encoded as UTF-8 with no byte order mark (BOM). For compatibility
        // with older SMBIOS parsers, US-ASCII characters should be used.
        //
        // When the formatted portion of an SMBIOS structure references a string, it does so by specifying
        // a non-zero string number within the structure's string-set.
        //
        // If a string field references no string, a null (0) is placed in that string field."

        // Referential transparency:
        // In rust we can return the empty string ("") when index is 0. This is idempotent because
        // the structure's string-set, by design, is incapable of producing an empty string.

        match index_usize == 0 {
            true => Ok(String::new()),
            false => match index_usize <= self.strings.len() {
                true => Ok(String::from_utf8(self.strings[index_usize - 1].clone())?),
                false => Err(SMBiosStringError::InvalidStringNumber(index)),
            },
        }
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
        let strings: Vec<String> = self.into_iter().collect();
        let mut seq = serializer.serialize_seq(Some(strings.len()))?;
        for e in strings {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}

/// # SMBiosStringError
///
/// An SMBIOS String retrival error
#[derive(Serialize, Debug)]
pub enum SMBiosStringError {
    /// The structure's field is out of bounds of the formatted portion of the SMBIOS structure
    FieldOutOfBounds,
    /// The given string number was outside the range of the SMBIOS structure's string-set
    InvalidStringNumber(u8),
    /// UTF8 parsing error
    #[serde(serialize_with = "ser_from_utf8_error")]
    Utf8(FromUtf8Error),
}

fn ser_from_utf8_error<S>(data: &FromUtf8Error, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(format!("{}", data).as_str())
}

impl fmt::Display for SMBiosStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SMBiosStringError::FieldOutOfBounds => {
                write!(
                    f,
                    "TThe structure's field is out of bounds of the formatted portion of the SMBIOS structure"
                )
            }
            SMBiosStringError::InvalidStringNumber(_) => {
                write!(
                    f,
                    "The given string number was outside the range of the SMBIOS structure's string-set"
                )
            }
            // The wrapped error contains additional information and is available
            // via the source() method.
            SMBiosStringError::Utf8(..) => {
                write!(f, "UTF8 parsing error")
            }
        }
    }
}

impl error::Error for SMBiosStringError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            SMBiosStringError::Utf8(ref e) => Some(e),
            _ => None,
        }
    }
}

// Implement the conversion from `FromUtf8Error` to `SMBiosStringError`.
// This will be automatically called by `?` if a `FromUtf8Error`
// needs to be converted into a `SMBiosStringError`.
impl From<FromUtf8Error> for SMBiosStringError {
    fn from(err: FromUtf8Error) -> SMBiosStringError {
        SMBiosStringError::Utf8(err)
    }
}
