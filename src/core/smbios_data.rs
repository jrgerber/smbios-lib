use crate::*;
use std::fs::read;
use std::io::Error;
use std::{cmp::Ordering, slice::Iter};

/// # SMBIOS Data
///
/// Contains an optional SMBIOS version and a collection of SMBIOS structures.
pub struct SMBiosData {
    table: UndefinedStructTable,
    /// Version of the contained SMBIOS structures.
    pub version: Option<SMBiosVersion>,
}

impl SMBiosData {
    /// Creates an SMBIOS table parser which can be iterated
    ///
    /// `data` is a block of bytes representing the raw table data.
    /// `version` is optional and represents the DMTF SMBIOS Standard version of the bytes in `data`.
    pub fn from_vec_and_version(data: Vec<u8>, version: Option<SMBiosVersion>) -> Self {
        Self {
            table: UndefinedStructTable::from(data),
            version,
        }
    }

    /// Loads raw SMBios table data from a file
    pub fn try_load_from_file(
        filename: &str,
        version: Option<SMBiosVersion>,
    ) -> Result<SMBiosData, Error> {
        let data = read(filename)?;
        let result = Self {
            table: UndefinedStructTable::from(data),
            version,
        };
        Ok(result)
    }

    /// Iterator of the contained [UndefinedStruct] items.
    pub fn iter(&self) -> Iter<'_, UndefinedStruct> {
        self.table.iter()
    }

    /// Finds the first occurance of the structure
    pub fn find_first<'a, T>(&'a self) -> Option<T>
    where
        T: SMBiosStruct<'a>,
    {
        self.table.find_first()
    }
}

impl IntoIterator for SMBiosData {
    type Item = UndefinedStruct;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.table.into_iter()
    }
}

impl fmt::Debug for SMBiosData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Convert to defined structures to see the structure fields
        let defined_table: DefinedStructTable<'_> = self.table.iter().collect();

        fmt.debug_struct(std::any::type_name::<SMBiosData>())
            .field("version", &self.version)
            .field("table", &defined_table)
            .finish()
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
