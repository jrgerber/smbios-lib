use crate::{SMBiosStruct, UndefinedStruct};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use core::any;
use std::fmt;

// The BIS (Boot Integrity Services) Entry Point structure is not defined in the SMBIOS DMTF document.
// bisapi037.pdf, section 3.1.3
// typedef struct _BIS_ENTRY_POINT
// {
//  UINT8 smBiosType;
//  UINT8 length;
//  UINT16 structHandle;
//  UINT8 structChecksum;
//  UINT8 reserved1;
//  UINT16 reserved2;
//  pBisEntry16 bisEntry16;
//  pBisEntry32 bisEntry32;
//  UINT64 reserved3;
//  UINT32 reserved4;
//  UINT16 doubleNull;
// }
// BIS_ENTRY_POINT,
// *pBIS_ENTRY_POINT;

/// # Boot Integrity Services (BIS) (Type 31)
/// Structure is reserved for use by the Boot Integrity Services (BIS)
///
/// This class is compliant with:
/// Boot Integrity Services Application Programming Interface Version 1.0
/// with added corrigenda: bis037
/// Published August 31, 1999
///
/// Note:
/// The BIS (Boot Integrity Services) Entry Point structure is not defined in the SMBIOS DMTF document.
/// bisapi037.pdf, section 3.1.3
/// typedef struct _BIS_ENTRY_POINT
pub struct SMBiosBisEntryPoint<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosBisEntryPoint<'a> {
    const STRUCT_TYPE: u8 = 31u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosBisEntryPoint<'a> {
    /// Used to make the 8-bit checksum of this structure equal zero.
    pub fn checksum(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    // fn reserved_1(&self) -> Option<u8> {
    //     self.parts.get_field_byte(0x05)
    // }

    // fn reserved_2(&self) -> Option<u16> {
    //     self.parts.get_field_word(0x06)
    // }

    /// BIS entry point pointer for use by 16-bit real-mode callers. This is a segmented pointer(in segment:offset form).
    ///
    /// Real Mode: In Real Mode all addresses are composed of Segment:Offset pairs, with
    /// both Segment and Offset being 16 bits.These combine to form a 20-bit physical address
    /// that cannot access above approximately the 1 Megabyte boundary.This mode is
    /// referred to in this section as “16-bit real-mode” or “16-bit mode”. Callers in this mode
    /// may only invoke BIS through the bisEntry16 entry point. The bisEntry16 function
    /// returns with the processor in 16-bit real-mode.
    pub fn bis_entry_16(&self) -> Option<u32> {
        self.parts.get_field_dword(0x08)
    }

    /// BIS entry point pointer for use by 32-bit flat physical address mode callers. This is a 32-bit physical address.
    ///
    /// Flat Mode: A 32-bit (protected) IA-32 processor mode where CS:0, DS:0, and SS:0 all
    /// refer to physical location 0 and all have 4 GB of address space.This mode is referred to
    /// in this section as “32-bit flat-mode” or “32-bit mode”. Callers in this mode may only
    /// invoke BIS through the bisEntry32 entry point. The bisEntry32 function returns with the
    /// processor in 32-bit flat-mode.
    pub fn bis_entry_32(&self) -> Option<u32> {
        self.parts.get_field_dword(0x0C)
    }

    // fn reserved_3(&self) -> Option<u64> {
    //     self.parts.get_field_qword(0x10)
    // }

    // fn reserved_4(&self) -> Option<u32> {
    //     self.parts.get_field_dword(0x18)
    // }
}

impl fmt::Debug for SMBiosBisEntryPoint<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<SMBiosBisEntryPoint<'_>>())
            .field("header", &self.parts.header)
            .field("checksum", &self.checksum())
            .field("bis_entry_16", &self.bis_entry_16())
            .field("bis_entry_32", &self.bis_entry_32())
            .finish()
    }
}

impl Serialize for SMBiosBisEntryPoint<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosBisEntryPoint", 4)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field("checksum", &self.checksum())?;
        state.serialize_field("bis_entry_16", &self.bis_entry_16())?;
        state.serialize_field("bis_entry_32", &self.bis_entry_32())?;
        state.end()
    }
}
