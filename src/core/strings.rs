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
    type Item = Result<String, FromUtf8Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_string_index == self.strings.len() {
            self.reset();
            return None;
        }

        let result = String::from_utf8(self.strings[self.current_string_index].clone());

        self.current_string_index = self.current_string_index + 1;

        Some(result)
    }
}

impl IntoIterator for &Strings {
    type Item = Result<String, FromUtf8Error>;
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
        let strings: Vec<Result<String, FromUtf8Error>> = self.into_iter().collect();
        let mut seq = serializer.serialize_seq(Some(strings.len()))?;
        for e in strings {
            match e {
                Ok(val) => seq.serialize_element(&val)?,
                Err(err) => seq.serialize_element(format!("{}", err).as_str())?,
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_parsing() {
        let string_set_bytes = vec![
            // "en|US|iso8859-1"
            0x65, 0x6E, 0x7C, 0x55, 0x53, 0x7C, 0x69, 0x73, 0x6F, 0x38, 0x38, 0x35, 0x39, 0x2D,
            0x31, 0x00, // "Heart=💖"
            b'H', b'e', b'a', b'r', b't', b'=', 240, 159, 146, 150, 0x00, // "Error="
            b'E', b'r', b'r', b'o', b'r', b'=', 1, 159, 146, 150, 0x00,
            // "ja|JP|unicode"
            0x6A, 0x61, 0x7C, 0x4A, 0x50, 0x7C, 0x75, 0x6E, 0x69, 0x63, 0x6F, 0x64, 0x65,
        ];

        let string_set = Strings::new(string_set_bytes);

        let mut string_iterator = string_set.into_iter();

        let first_string = string_iterator.next().unwrap().unwrap();
        assert_eq!(first_string, "en|US|iso8859-1".to_string());

        let second_string = string_iterator.next().unwrap().unwrap();
        assert_eq!(second_string, "Heart=💖".to_string());

        // Err(FromUtf8Error { bytes: [69, 114, 114, 111, 114, 61, 1, 159, 146, 150], error: Utf8Error { valid_up_to: 7, error_len: Some(1) } })
        match string_iterator.next().unwrap() {
            Ok(_) => panic!("This should have been a UTF8 error"),
            Err(err) => {
                assert_eq!(7, err.utf8_error().valid_up_to());
                assert_eq!(
                    "Error=\u{1}���",
                    String::from_utf8_lossy(err.as_bytes()).to_string()
                );
            }
        }

        let fourth_string = string_iterator.next().unwrap().unwrap();
        assert_eq!(fourth_string, "ja|JP|unicode".to_string());
    }
}
