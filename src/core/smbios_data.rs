use super::undefined_struct::{UndefinedStruct, UndefinedStructTable};
use crate::{DefinedStructTable, Handle, SMBiosStruct};
use std::io::Error;
use std::{cmp::Ordering, slice::Iter};
use std::{fmt, fs::read};

/// # SMBIOS Data
///
/// Contains an optional SMBIOS version and a collection of SMBIOS structures.
pub struct SMBiosData {
    table: UndefinedStructTable,
    /// Version of the contained SMBIOS structures.
    pub version: Option<SMBiosVersion>,
}

impl<'a> SMBiosData {
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

    /// An iterator over the defined type instances within the table.
    pub fn defined_struct_iter<T: 'a>(&'a self) -> impl Iterator<Item = T> + 'a
    where
        T: SMBiosStruct<'a>,
    {
        self.table.defined_struct_iter()
    }

    /// Tests if every element of the defined struct iterator matches a predicate.
    pub fn all<T, F>(&'a self, f: F) -> bool
    where
        T: SMBiosStruct<'a>,
        F: FnMut(T) -> bool,
    {
        self.table.all(f)
    }

    /// Tests if any element of the defined struct iterator matches a predicate.
    pub fn any<T, F>(&'a self, f: F) -> bool
    where
        T: SMBiosStruct<'a>,
        F: FnMut(T) -> bool,
    {
        self.table.any(f)
    }

    /// Finds the first occurance of the structure
    pub fn first<T>(&'a self) -> Option<T>
    where
        T: SMBiosStruct<'a>,
    {
        self.table.first()
    }

    /// Finds the first occurance of the structure that satisfies a predicate.
    pub fn find<T, P>(&'a self, predicate: P) -> Option<T>
    where
        T: SMBiosStruct<'a>,
        P: FnMut(&T) -> bool,
    {
        self.table.find(predicate)
    }

    /// Applies function to the defined struct elements and returns the first non-none result.
    pub fn find_map<A, B, F>(&'a self, f: F) -> Option<B>
    where
        A: SMBiosStruct<'a>,
        F: FnMut(A) -> Option<B>,
    {
        self.table.find_map(f)
    }

    /// Creates an iterator of the defined structure which uses a closure to determine if an element should be yielded.
    pub fn filter<T: 'a, P: 'a>(&'a self, predicate: P) -> impl Iterator<Item = T> + 'a
    where
        T: SMBiosStruct<'a>,
        P: FnMut(&T) -> bool,
    {
        self.table.filter(predicate)
    }

    /// Takes a closure and creates an iterator which calls that closure on each defined struct.
    pub fn map<A: 'a, B: 'a, F: 'a>(&'a self, f: F) -> impl Iterator<Item = B> + 'a
    where
        A: SMBiosStruct<'a>,
        F: FnMut(A) -> B,
    {
        self.table.map(f)
    }

    /// Creates an iterator that both filters and maps from the defined struct iterator.
    pub fn filter_map<A: 'a, B: 'a, F: 'a>(&'a self, f: F) -> impl Iterator<Item = B> + 'a
    where
        A: SMBiosStruct<'a>,
        F: FnMut(A) -> Option<B>,
    {
        self.table.filter_map(f)
    }

    /// Finds the structure matching the given handle
    pub fn find_by_handle(&'a self, handle: &Handle) -> Option<&UndefinedStruct> {
        self.table.find_by_handle(handle)
    }

    /// Finds all occurances of the structure
    pub fn collect<T>(&'a self) -> Vec<T>
    where
        T: SMBiosStruct<'a>,
    {
        self.table.collect()
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
