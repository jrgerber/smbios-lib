use crate::*;
use std::io::Error;
use std::{cmp::Ordering, slice::Iter};
use std::{fs::read, iter::Filter};

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

    /// Iterator of the contained [UndefinedStruct] items
    pub fn iter(&self) -> Iter<'_, UndefinedStruct> {
        self.table.iter()
    }

    /// Finds the first occurance of the structure
    pub fn first_defined_struct<'a, T>(&'a self) -> Option<T>
    where
        T: SMBiosStruct<'a>,
    {
        self.table.first_defined_struct()
    }

    /// Finds the first occurance of the structure that satisfies a predicate.
    pub fn find_defined_struct<'a, T, P>(&'a self, predicate: P) -> Option<T>
    where
        T: SMBiosStruct<'a>,
        P: FnMut(&T) -> bool,
    {
        self.table.find_defined_struct(predicate)
    }

    /// Finds the instances of the structure that satisfies a predicate.
    pub fn filter_defined_struct<'a, T: 'a, P: 'a>(
        &'a self,
        predicate: P,
    ) -> Filter<impl Iterator<Item = T> + 'a, P>
    where
        T: SMBiosStruct<'a>,
        P: FnMut(&T) -> bool,
    {
        self.table.filter_defined_struct(predicate)
    }

    /// Finds the structure matching the given handle
    pub fn find_handle<'a>(&'a self, handle: &Handle) -> Option<&UndefinedStruct> {
        self.table.find_handle(handle)
    }

    /// Finds all occurances of the structure
    pub fn collect_defined_struct<'a, T>(&'a self) -> Vec<T>
    where
        T: SMBiosStruct<'a>,
    {
        self.table.collect_defined_struct()
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
