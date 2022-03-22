use serde::{ser::SerializeStruct, Serialize, Serializer};
#[cfg(not(feature = "no_std"))]
use std::{
    fs::{read, File},
    io::{prelude::*, Error, ErrorKind, SeekFrom},
    path::Path,
    ops::RangeBounds,
};
use core::{
    convert::TryFrom,
    convert::TryInto,
    fmt,
    num::Wrapping,
    any
};
use alloc::{vec::Vec, format};

/// SMBiosEntryPoint32 structure parse errors
pub enum SMBiosEntryPoint32Error {
    /// Slice is smaller than SMBiosEntryPoint32::MINIMUM_SIZE
    SliceTooSmall,
    /// _SM_ anchor not found
    SMNotFound,
    /// Entry Point Structure checksum verification failed
    EntryChecksumVerificationFailed,
    /// The Entry Point Length field specified a value which exceeded the bounds of the Entry Point Structure
    EntryPointLengthTooBig,
    /// _DMI_ anchor not found
    DMINotFound,
    /// Intermediate entry point structure checksum verification failed
    IntermediateChecksumVerificationFailed,
    /// Entry Point not found
    EntryPointNotFound,
}

#[cfg(not(feature = "no_std"))]
impl SMBiosEntryPoint32Error {
    fn into_io_error(&self) -> Error {
        Error::new(ErrorKind::InvalidData, self.to_string())
    }
}

impl fmt::Debug for SMBiosEntryPoint32Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SMBiosEntryPoint32Error")
            .field(&format!("{}", &self))
            .finish()
    }
}

impl fmt::Display for SMBiosEntryPoint32Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            SMBiosEntryPoint32Error::SliceTooSmall => "Slice is smaller than SMBiosEntryPoint32::MINIMUM_SIZE",
            SMBiosEntryPoint32Error::SMNotFound => "_SM_ anchor not found",
            SMBiosEntryPoint32Error::EntryChecksumVerificationFailed => "Entry Point Structure checksum verification failed",
            SMBiosEntryPoint32Error::EntryPointLengthTooBig => "The Entry Point Length field specified a value which exceeded the bounds of the Entry Point Structure",
            SMBiosEntryPoint32Error::DMINotFound => "_DMI_ anchor not found",
            SMBiosEntryPoint32Error::IntermediateChecksumVerificationFailed => "Intermediate entry point structure checksum verification failed",
            SMBiosEntryPoint32Error::EntryPointNotFound => "Entry Point not found"
        };
        f.write_str(message)
    }
}

#[cfg(not(feature = "no_std"))]
impl std::error::Error for SMBiosEntryPoint32Error {}

/// # SMBIOS 2.1 (32 bit) Entry Point structure
///
/// On non-UEFI systems, the 32-bit SMBIOS Entry Point structure, can be located by application software
/// by searching for the anchor-string on paragraph (16-byte) boundaries within the physical memory address
/// range 000F0000h to 000FFFFFh. This entry point encapsulates an intermediate anchor string that is used
/// by some existing DMI browsers.
///
/// On UEFI-based systems, the SMBIOS Entry Point structure can be located by looking in the EFI
/// Configuration Table for the SMBIOS GUID (SMBIOS_TABLE_GUID, {EB9D2D31-2D88-11D3-9A16-
/// 0090273FC14D}) and using the associated pointer. See section 4.6 of the UEFI Specification for details.
/// See section 2.3 of the UEFI Specification for how to report the containing memory type.
pub struct SMBiosEntryPoint32 {
    raw: Vec<u8>,
}

impl<'a> SMBiosEntryPoint32 {
    /// Minimum acceptable size of this structure
    ///
    /// TODO: Review DMTF SMBIOS document history and see
    /// if structure sizes smaller than 0x1F existed.  If
    /// so then change this structure design to return Option<>
    /// values and adjust this size accordingly.
    pub const MINIMUM_SIZE: usize = 0x1F;

    /// Anchor String "_SM_" (offset 0)
    pub const SM_ANCHOR: [u8; 4] = [b'_', b'S', b'M', b'_'];

    /// Anchor String "_DMI_" (offset 0x10)
    pub const DMI_ANCHOR: [u8; 5] = [b'_', b'D', b'M', b'I', b'_'];

    /// Entry Point Structure Checksum Offset
    pub const ENTRY_POINT_STRUCTURE_CHECKSUM_OFFSET: usize = 0x04;

    /// Entry Point Length Offset
    pub const ENTRY_POINT_LENGTH_OFFSET: usize = 0x05;

    /// SMBIOS Major Version Offset
    pub const MAJOR_VERSION_OFFSET: usize = 0x06;

    /// SMBIOS Minor Version Offset
    pub const MINOR_VERSION_OFFSET: usize = 0x07;

    /// Maximum Structure Size Offset
    pub const MAXIMUM_STRUCTURE_SIZE_OFFSET: usize = 0x08;

    /// Entry Point Revision Offset
    pub const ENTRY_POINT_REVISION_OFFSET: usize = 0x0A;

    /// Formatted Area Offset
    pub const FORMATTED_AREA_OFFSET: usize = 0x0B;

    /// Intermediate Anchor String Offset
    ///
    /// NOTE: This field is paragraph-aligned, to allow legacy DMI browsers to
    /// find this entry point within the SMBIOS Entry Point Structure.
    pub const INTERMEDIATE_ANCHOR_OFFSET: usize = 0x10;

    /// Intermediate Checksum Offset
    pub const INTERMEDIATE_CHECKSUM_OFFSET: usize = 0x15;

    /// Structure Table Length Offset
    pub const STRUCTURE_TABLE_LENGTH_OFFSET: usize = 0x16;

    /// Structure Table Address Offset
    pub const STRUCTURE_TABLE_ADDRESS_OFFSET: usize = 0x18;

    /// Number of SMBIOS Structures Offset
    pub const NUMBER_OF_SMBIOS_STRUCTURES_OFFSET: usize = 0x1C;

    /// SMBIOS BCD Revision Offset
    pub const BCD_REVISION_OFFSET: usize = 0x1E;

    /// Entry Point Structure Checksum
    ///
    /// Checksum of the Entry Point Structure (EPS)
    ///
    /// This value, when added to all other bytes in the EPS, results in
    /// the value 00h (using 8-bit addition calculations). Values in the
    /// EPS are summed starting at offset 00h, for `entry_point_length`
    /// bytes.
    pub fn entry_point_structure_checksum(&self) -> u8 {
        self.raw[Self::ENTRY_POINT_STRUCTURE_CHECKSUM_OFFSET]
    }

    /// Entry Point Length
    ///
    /// Length of the Entry Point Structure, starting with the Anchor String
    /// field, in bytes, currently 1Fh
    ///
    /// NOTE: This value was incorrectly stated in version 2.1 of this specification
    /// as 1Eh. Because of this, there might be version 2.1
    /// implementations that use either the 1Eh or the 1Fh value, but
    /// version 2.2 or later implementations must use the 1Fh value.
    pub fn entry_point_length(&self) -> u8 {
        self.raw[Self::ENTRY_POINT_LENGTH_OFFSET]
    }

    /// SMBIOS Major Version
    ///
    /// Major version of this specification implemented in the table
    /// structures (for example, the value is 0Ah (10) for revision 10.22 and
    /// 02h for revision 2.1)
    pub fn major_version(&self) -> u8 {
        self.raw[Self::MAJOR_VERSION_OFFSET]
    }

    /// SMBIOS Minor Version
    ///
    /// Minor version of this specification implemented in the table
    /// structures (for example, the value is 16h (22) for revision 10.22 and
    /// 01h for revision 2.1)
    pub fn minor_version(&self) -> u8 {
        self.raw[Self::MINOR_VERSION_OFFSET]
    }

    /// Maximum Structure Size
    ///
    /// Size of the largest SMBIOS structure, in bytes, and encompasses
    /// the structure’s formatted area and text strings
    pub fn maximum_structure_size(&self) -> u16 {
        u16::from_le_bytes(
            self.raw[Self::MAXIMUM_STRUCTURE_SIZE_OFFSET..Self::MAXIMUM_STRUCTURE_SIZE_OFFSET + 2]
                .try_into()
                .expect("u16 is 2 bytes"),
        )
    }

    /// Entry Point Revision
    ///
    /// EPS revision implemented in this structure and identifies the
    /// formatting of offsets 0Bh to 0Fh as follows:
    /// - 00h Entry Point is based on SMBIOS 2.1 definition; formatted area is reserved and set to all 00h.
    /// - 01h-FFh Reserved for assignment by this specification
    pub fn entry_point_revision(&self) -> u8 {
        self.raw[Self::ENTRY_POINT_REVISION_OFFSET]
    }

    /// Formatted Area
    ///
    /// Value present in the `entry_point_revision` field defines the
    /// interpretation to be placed upon these 5 bytes
    pub fn formatted_area(&self) -> [u8; 5] {
        self.raw[Self::FORMATTED_AREA_OFFSET..Self::FORMATTED_AREA_OFFSET + 5]
            .try_into()
            .expect("5 bytes")
    }

    /// Intermediate Anchor String
    ///
    /// _DMI_, specified as five ASCII characters (5F 44 4D 49 5F).
    pub fn intermediate_anchor(&self) -> [u8; 5] {
        self.raw[Self::INTERMEDIATE_ANCHOR_OFFSET..Self::INTERMEDIATE_ANCHOR_OFFSET + 5]
            .try_into()
            .expect("5 bytes")
    }

    /// Intermediate Checksum
    ///
    /// Checksum of Intermediate Entry Point Structure (IEPS).
    ///
    /// This value, when added to all other bytes in the IEPS, results in
    /// the value 00h (using 8-bit addition calculations). Values in the
    /// IEPS are summed starting at offset 10h, for 0Fh bytes.
    pub fn intermediate_checksum(&self) -> u8 {
        self.raw[Self::INTERMEDIATE_CHECKSUM_OFFSET]
    }

    /// Structure Table Length
    ///
    /// Total length of SMBIOS Structure Table, pointed to by the
    /// `structure_table_address`, in bytes
    pub fn structure_table_length(&self) -> u16 {
        u16::from_le_bytes(
            self.raw[Self::STRUCTURE_TABLE_LENGTH_OFFSET..Self::STRUCTURE_TABLE_LENGTH_OFFSET + 2]
                .try_into()
                .expect("u16 is 2 bytes"),
        )
    }

    /// Structure Table Address
    ///
    /// 32-bit physical starting address of the read-only SMBIOS
    /// Structure Table, which can start at any 32-bit address
    /// This area contains all of the SMBIOS structures fully packed
    /// together. These structures can then be parsed to produce exactly
    /// the same format as that returned from a Get SMBIOS Structure
    /// function call.
    pub fn structure_table_address(&self) -> u32 {
        u32::from_le_bytes(
            self.raw
                [Self::STRUCTURE_TABLE_ADDRESS_OFFSET..Self::STRUCTURE_TABLE_ADDRESS_OFFSET + 4]
                .try_into()
                .expect("u32 is 4 bytes"),
        )
    }

    /// Number of SMBIOS Structures
    ///
    /// Total number of structures present in the SMBIOS Structure Table
    /// This is the value returned as NumStructures from the Get
    /// SMBIOS Information function.
    pub fn number_of_smbios_structures(&self) -> u16 {
        u16::from_le_bytes(
            self.raw[Self::NUMBER_OF_SMBIOS_STRUCTURES_OFFSET
                ..Self::NUMBER_OF_SMBIOS_STRUCTURES_OFFSET + 2]
                .try_into()
                .expect("u16 is 2 bytes"),
        )
    }

    /// SMBIOS BCD Revision
    ///
    /// Indicates compliance with a revision of this specification
    /// It is a BCD value where the upper nibble indicates the major
    /// version and the lower nibble the minor version. For revision 2.1,
    /// the returned value is 21h. If the value is 00h, only the Major and
    /// Minor Versions in offsets 6 and 7 of the Entry Point Structure
    /// provide the version information.
    pub fn bcd_revision(&self) -> u8 {
        self.raw[Self::BCD_REVISION_OFFSET]
    }

    /// Load this structure from a file
    #[cfg(not(feature = "no_std"))]
    pub fn try_load_from_file(filename: &Path) -> Result<Self, Error> {
        read(filename)?.try_into().map_err(|e: SMBiosEntryPoint32Error| e.into_io_error())
    }

    /// Load this structure by scanning a file within the given offsets,
    /// looking for the [SMBiosEntryPoint32::SM_ANCHOR] string.
    #[cfg(not(feature = "no_std"))]
    pub fn try_scan_from_file<T: Iterator<Item = u64>>(
        file: &mut File,
        range: T,
    ) -> Result<Self, Error>
    where
        T: RangeBounds<u64>,
    {
        let mut anchor = [0; 4];
        for offset in range.step_by(0x10) {
            file.seek(SeekFrom::Start(offset))?;
            file.read_exact(&mut anchor)?;
            if anchor == Self::SM_ANCHOR {
                let mut length = [0; 2];
                file.read_exact(&mut length)?;
                let struct_length = length[1] as usize;
                let mut entry_point_buffer = Vec::with_capacity(struct_length);
                entry_point_buffer.resize(struct_length, 0);
                file.seek(SeekFrom::Start(offset))?;
                file.read_exact(&mut entry_point_buffer)?;
                let entry_point: Self = entry_point_buffer.try_into()
                    .map_err(|e: SMBiosEntryPoint32Error| e.into_io_error())?;
                return Ok(entry_point);
            }
        }
        Err(Error::new(ErrorKind::UnexpectedEof, "Not found"))
    }

    /// Load this structure by scanning given memory slice,
    /// looking for the [SMBiosEntryPoint32::SM_ANCHOR] string.
    #[cfg(feature = "no_std")]
    pub fn try_scan_from_raw(data: &[u8]) -> Result<Self, SMBiosEntryPoint32Error> {
        let range = 0..data.len();
        for offset in range.step_by(0x10) {
            let anchor = &data[offset..offset + 4];
            if anchor == Self::SM_ANCHOR {
                let length = &data[offset + 4..offset + 6];
                let struct_length = length[1] as usize;
                let entry_point_buffer = data[offset..offset + struct_length].to_vec();
                let entry_point: Self = entry_point_buffer.try_into()?;
                return Ok(entry_point);
            }
        }
        Err(SMBiosEntryPoint32Error::EntryPointNotFound)
    }
}

impl<'a> TryFrom<Vec<u8>> for SMBiosEntryPoint32 {
    type Error = SMBiosEntryPoint32Error;

    fn try_from(raw: Vec<u8>) -> Result<Self, Self::Error> {
        if raw.len() < Self::MINIMUM_SIZE {
            return Err(SMBiosEntryPoint32Error::SliceTooSmall);
        }

        if !raw
            .iter()
            .zip(Self::SM_ANCHOR.iter())
            .all(|pair| pair.0 == pair.1)
        {
            return Err(SMBiosEntryPoint32Error::SMNotFound);
        }

        // Verify the EPS checksum
        // The checksum is calculated for a length of `entry_point_length`
        let entry_point_length = raw[Self::ENTRY_POINT_LENGTH_OFFSET] as usize;
        match raw.get(0..entry_point_length) {
            Some(checked_bytes) => {
                if !verify_checksum(checked_bytes) {
                    return Err(SMBiosEntryPoint32Error::EntryChecksumVerificationFailed);
                }
            }
            None => return Err(SMBiosEntryPoint32Error::EntryPointLengthTooBig),
        }

        let intermediate_anchor: [u8; 5] = raw
            [Self::INTERMEDIATE_ANCHOR_OFFSET..Self::INTERMEDIATE_ANCHOR_OFFSET + 5]
            .try_into()
            .expect("5 bytes");

        if !intermediate_anchor
            .iter()
            .zip(Self::DMI_ANCHOR.iter())
            .all(|pair| pair.0 == pair.1)
        {
            return Err(SMBiosEntryPoint32Error::DMINotFound);
        }

        // Verify the IEPS checksum
        // The checksum is calculated for a length of 0x0F
        let intermediate_entry_point_structure: [u8; 0x0F] = raw
            [Self::INTERMEDIATE_ANCHOR_OFFSET..]
            .try_into()
            .expect("0x0F bytes");

        if !verify_checksum(&intermediate_entry_point_structure) {
            return Err(SMBiosEntryPoint32Error::IntermediateChecksumVerificationFailed);
        }

        Ok(SMBiosEntryPoint32 { raw })
    }
}

impl fmt::Debug for SMBiosEntryPoint32 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<SMBiosEntryPoint32>())
            .field(
                "entry_point_structure_checksum",
                &self.entry_point_structure_checksum(),
            )
            .field("entry_point_length", &self.entry_point_length())
            .field("major_version", &self.major_version())
            .field("minor_version", &self.minor_version())
            .field("maximum_structure_size", &self.maximum_structure_size())
            .field("entry_point_revision", &self.entry_point_revision())
            .field("formatted_area", &self.formatted_area())
            .field("intermediate_anchor", &self.intermediate_anchor())
            .field("intermediate_checksum", &self.intermediate_checksum())
            .field("structure_table_length", &self.structure_table_length())
            .field("structure_table_address", &self.structure_table_address())
            .field(
                "number_of_smbios_structures",
                &self.number_of_smbios_structures(),
            )
            .field("bcd_revision", &self.bcd_revision())
            .finish()
    }
}

impl Serialize for SMBiosEntryPoint32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosEntryPoint32", 13)?;
        state.serialize_field(
            "entry_point_structure_checksum",
            &self.entry_point_structure_checksum(),
        )?;
        state.serialize_field("entry_point_length", &self.entry_point_length())?;
        state.serialize_field("major_version", &self.major_version())?;
        state.serialize_field("minor_version", &self.minor_version())?;
        state.serialize_field("maximum_structure_size", &self.maximum_structure_size())?;
        state.serialize_field("entry_point_revision", &self.entry_point_revision())?;
        state.serialize_field("formatted_area", &self.formatted_area())?;
        state.serialize_field("intermediate_anchor", &self.intermediate_anchor())?;
        state.serialize_field("intermediate_checksum", &self.intermediate_checksum())?;
        state.serialize_field("structure_table_length", &self.structure_table_length())?;
        state.serialize_field("structure_table_address", &self.structure_table_address())?;
        state.serialize_field(
            "number_of_smbios_structures",
            &self.number_of_smbios_structures(),
        )?;
        state.serialize_field("bcd_revision", &self.bcd_revision())?;
        state.end()
    }
}

/// SMBiosEntryPoint64 structure parse errors
pub enum SMBiosEntryPoint64Error {
    /// Slice is smaller than SMBiosEntryPoint64::MINIMUM_SIZE
    SliceTooSmall,
    /// Entry Point Structure checksum verification failed
    SM3NotFound,
    /// The Entry Point Length field specified a value which exceeded the bounds of the Entry Point Structure
    ChecksumVerificationFailed,
    /// Expected _SM3_ identifier not found
    EntryPointLengthTooBig,
    /// Entry Point not found
    EntryPointNotFound
}

#[cfg(not(feature = "no_std"))]
impl SMBiosEntryPoint64Error {
    fn into_io_error(&self) -> Error {
        Error::new(ErrorKind::InvalidData, self.to_string())
    }
}

impl fmt::Debug for SMBiosEntryPoint64Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SMBiosEntryPoint64Error")
            .field(&format!("{}", &self))
            .finish()
    }
}

impl fmt::Display for SMBiosEntryPoint64Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            SMBiosEntryPoint64Error::SliceTooSmall => "Slice is smaller than SMBiosEntryPoint64::MINIMUM_SIZE",
            SMBiosEntryPoint64Error::ChecksumVerificationFailed => "Entry Point Structure checksum verification failed",
            SMBiosEntryPoint64Error::EntryPointLengthTooBig => "The Entry Point Length field specified a value which exceeded the bounds of the Entry Point Structure",
            SMBiosEntryPoint64Error::SM3NotFound => "Expected _SM3_ identifier not found",
            SMBiosEntryPoint64Error::EntryPointNotFound => "Entry Point not found"
        };
        f.write_str(message)
    }
}

#[cfg(not(feature = "no_std"))]
impl std::error::Error for SMBiosEntryPoint64Error {}

/// # SMBIOS 3.0 (64 bit) Entry Point structure
///
/// On non-UEFI systems, the 64-bit SMBIOS Entry Point structure can be located by application software by
/// searching for the anchor-string on paragraph (16-byte) boundaries within the physical memory address
/// range 000F0000h to 000FFFFFh.
///
/// On UEFI-based systems, the SMBIOS Entry Point structure can be located by looking in the EFI
/// Configuration Table for the SMBIOS 3.x GUID (SMBIOS3_TABLE_GUID, {F2FD1544-9794-4A2C-992E836 E5BBCF20E394}) and using the associated pointer. See section 4.6 of the UEFI Specification for details.
/// See section 2.3 of the UEFI Specification for how to report the containing memory type.
pub struct SMBiosEntryPoint64 {
    raw: Vec<u8>,
}

impl<'a> SMBiosEntryPoint64 {
    /// Minimum acceptable size of this structure
    ///
    /// TODO: Review DMTF SMBIOS document history and see
    /// if structure sizes smaller than 0x18 existed.  If
    /// so then change this structure design to return Option<>
    /// values and adjust this size accordingly.
    pub const MINIMUM_SIZE: usize = 0x18;

    /// Anchor String "_SM3_" (offset 0)
    pub const SM3_ANCHOR: [u8; 5] = [b'_', b'S', b'M', b'3', b'_'];

    /// Entry Point Structure Checksum Offset
    pub const ENTRY_POINT_STRUCTURE_CHECKSUM_OFFSET: usize = 0x05;

    /// Entry Point Length Offset
    pub const ENTRY_POINT_LENGTH_OFFSET: usize = 0x06;

    /// SMBIOS Major Version Offset
    pub const MAJOR_VERSION_OFFSET: usize = 0x07;

    /// SMBIOS Minor Version Offset
    pub const MINOR_VERSION_OFFSET: usize = 0x08;

    /// SMBIOS Docrev
    pub const DOCREV_OFFSET: usize = 0x09;

    /// Entry Point Revision Offset
    pub const ENTRY_POINT_REVISION_OFFSET: usize = 0x0A;

    /// Structure Table Maximum Size Offset
    pub const STRUCTURE_TABLE_MAXIMUM_SIZE_OFFSET: usize = 0x0C;

    /// Structure Table Address Offset
    pub const STRUCTURE_TABLE_ADDRESS_OFFSET: usize = 0x10;

    /// Entry Point Structure Checksum
    ///
    /// Checksum of the Entry Point Structure (EPS)
    ///
    /// This value, when added to all other bytes in the EPS, results in
    /// the value 00h (using 8-bit addition calculations). Values in the
    /// EPS are summed starting at offset 00h, for `entry_point_length`
    /// bytes.
    pub fn entry_point_structure_checksum(&self) -> u8 {
        self.raw[Self::ENTRY_POINT_STRUCTURE_CHECKSUM_OFFSET]
    }

    /// Entry Point Length
    ///
    /// Length of the Entry Point Structure, starting with the Anchor String
    /// field, in bytes, currently 18h
    pub fn entry_point_length(&self) -> u8 {
        self.raw[Self::ENTRY_POINT_LENGTH_OFFSET]
    }

    /// SMBIOS Major Version
    ///
    /// Major version of this specification implemented in the table
    /// structures (for example, the value is 0Ah (10) for revision 10.22 and
    /// 02h for revision 2.1)
    pub fn major_version(&self) -> u8 {
        self.raw[Self::MAJOR_VERSION_OFFSET]
    }

    /// SMBIOS Minor Version
    ///
    /// Minor version of this specification implemented in the table
    /// structures (for example, the value is 16h (22) for revision 10.22 and
    /// 01h for revision 2.1)
    pub fn minor_version(&self) -> u8 {
        self.raw[Self::MINOR_VERSION_OFFSET]
    }

    /// SMBIOS Docrev
    ///
    /// Identifies the docrev of this specification implemented in the table
    /// structures (for example, the value is 00h for revision 10.22.0 and
    /// 01h for revision 2.7.1).
    pub fn docrev(&self) -> u8 {
        self.raw[Self::DOCREV_OFFSET]
    }

    /// Entry Point Revision
    ///
    /// EPS revision implemented in this structure and identifies the
    /// formatting of offsets 0Bh and beyond as follows:
    /// - 00h Reserved for assignment by this specification
    /// - 01h Entry Point is based on SMBIOS 3.0 definition;
    /// - 02h-FFh Reserved for assignment by this specification; offsets 0Ch-17h are defined per revision 01h
    pub fn entry_point_revision(&self) -> u8 {
        self.raw[Self::ENTRY_POINT_REVISION_OFFSET]
    }

    /// Structure Table Maximum Size
    ///
    /// Maximum size of SMBIOS Structure Table, pointed to by the
    /// Structure Table Address, in bytes. The actual size is guaranteed
    /// to be less or equal to the maximum size.
    pub fn structure_table_maximum_size(&self) -> u32 {
        u32::from_le_bytes(
            self.raw[Self::STRUCTURE_TABLE_MAXIMUM_SIZE_OFFSET
                ..Self::STRUCTURE_TABLE_MAXIMUM_SIZE_OFFSET + 4]
                .try_into()
                .expect("u32 is 4 bytes"),
        )
    }

    /// Structure Table Address
    ///
    /// The 64-bit physical starting address of the read-only SMBIOS
    /// Structure Table, which can start at any 64-bit address. This area
    /// contains all of the SMBIOS structures fully packed together
    pub fn structure_table_address(&self) -> u64 {
        u64::from_le_bytes(
            self.raw
                [Self::STRUCTURE_TABLE_ADDRESS_OFFSET..Self::STRUCTURE_TABLE_ADDRESS_OFFSET + 8]
                .try_into()
                .expect("u64 is 8 bytes"),
        )
    }

    /// Load this structure from a file
    #[cfg(not(feature = "no_std"))]
    pub fn try_load_from_file(filename: &Path) -> Result<Self, Error> {
        read(filename)?.try_into().map_err(|e: SMBiosEntryPoint64Error| e.into_io_error())
    }

    /// Load this structure by scanning a file within the given offsets,
    /// looking for the [SMBiosEntryPoint64::SM3_ANCHOR] string.
    #[cfg(not(feature = "no_std"))]
    pub fn try_scan_from_file<T: Iterator<Item = u64>>(
        file: &mut File,
        range: T,
    ) -> Result<Self, Error>
    where
        T: RangeBounds<u64>,
    {
        let mut anchor = [0; 5];
        for offset in range.step_by(0x10) {
            file.seek(SeekFrom::Start(offset))?;
            file.read_exact(&mut anchor)?;
            if anchor == Self::SM3_ANCHOR {
                let mut length = [0; 2];
                file.read_exact(&mut length)?;
                let struct_length = length[1] as usize;
                let mut entry_point_buffer = Vec::with_capacity(struct_length);
                entry_point_buffer.resize(struct_length, 0);
                file.seek(SeekFrom::Start(offset))?;
                file.read_exact(&mut entry_point_buffer)?;
                let entry_point: Self = entry_point_buffer.try_into()
                    .map_err(|e: SMBiosEntryPoint64Error| e.into_io_error())?;
                return Ok(entry_point);
            }
        }
        Err(Error::new(ErrorKind::UnexpectedEof, "Not found"))
    }

    /// Load this structure by scanning given memory slice,
    /// looking for the [SMBiosEntryPoint64::SM3_ANCHOR] string.
    #[cfg(feature = "no_std")]
    pub fn try_scan_from_raw(data: &[u8]) -> Result<Self, SMBiosEntryPoint64Error> {
        let range = 0..data.len();
        for offset in range.step_by(0x10) {
            let anchor = &data[offset..offset + 5];
            if anchor == Self::SM3_ANCHOR {
                let length = &data[offset + 5..offset + 7];
                let struct_length = length[1] as usize;
                let entry_point_buffer = data[offset..offset + struct_length].to_vec();
                let entry_point: Self = entry_point_buffer.try_into()?;
                return Ok(entry_point);
            }
        }
        Err(SMBiosEntryPoint64Error::EntryPointNotFound)
    }
}

impl fmt::Debug for SMBiosEntryPoint64 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<SMBiosEntryPoint64>())
            .field(
                "entry_point_structure_checksum",
                &self.entry_point_structure_checksum(),
            )
            .field("entry_point_length", &self.entry_point_length())
            .field("major_version", &self.major_version())
            .field("minor_version", &self.minor_version())
            .field("docrev", &self.docrev())
            .field(
                "structure_table_maximum_size",
                &self.structure_table_maximum_size(),
            )
            .field("structure_table_address", &self.structure_table_address())
            .finish()
    }
}

impl Serialize for SMBiosEntryPoint64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosEntryPoint64", 7)?;
        state.serialize_field(
            "entry_point_structure_checksum",
            &self.entry_point_structure_checksum(),
        )?;
        state.serialize_field("entry_point_length", &self.entry_point_length())?;
        state.serialize_field("major_version", &self.major_version())?;
        state.serialize_field("minor_version", &self.minor_version())?;
        state.serialize_field("docrev", &self.docrev())?;
        state.serialize_field(
            "structure_table_maximum_size",
            &self.structure_table_maximum_size(),
        )?;
        state.serialize_field("structure_table_address", &self.structure_table_address())?;
        state.end()
    }
}

impl<'a> TryFrom<Vec<u8>> for SMBiosEntryPoint64 {
    type Error = SMBiosEntryPoint64Error;

    fn try_from(raw: Vec<u8>) -> Result<Self, Self::Error> {
        if raw.len() < Self::MINIMUM_SIZE {
            return Err(SMBiosEntryPoint64Error::SliceTooSmall);
        }

        if !raw
            .iter()
            .zip(Self::SM3_ANCHOR.iter())
            .all(|pair| pair.0 == pair.1)
        {
            return Err(SMBiosEntryPoint64Error::SM3NotFound);
        }

        // Verify the checksum
        // The checksum is calculated for a length of `entry_point_length`
        let entry_point_length = raw[Self::ENTRY_POINT_LENGTH_OFFSET] as usize;
        match raw.get(0..entry_point_length) {
            Some(checked_bytes) => {
                if !verify_checksum(checked_bytes) {
                    return Err(SMBiosEntryPoint64Error::ChecksumVerificationFailed);
                }
            }
            None => return Err(SMBiosEntryPoint64Error::EntryPointLengthTooBig),
        }

        Ok(SMBiosEntryPoint64 { raw })
    }
}

/// Verifies EPS and IEPS Checksums
///
/// The EPS and IEPS contain a checksum value.
///
/// The checksum value, when added to all other bytes in the EPS, results in
/// the value 00h (using 8-bit addition [Wrapping] calculations).
/// Values in the EPS are summed starting at offset 00h, for 'entry_point_length'
/// bytes.
fn verify_checksum(data: &[u8]) -> bool {
    let mut sum = Wrapping(0u8);

    data.iter().for_each(|b| sum += Wrapping(*b));

    sum == Wrapping(0)
}
