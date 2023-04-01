use crate::core::{strings::*, UndefinedStruct};
use crate::SMBiosStruct;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use core::{fmt, any};
use core::ops::Deref;
use alloc::string::String;

/// #  BIOS Information (Type 0)
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.5.0 (DSP0134)
/// Document Date: 2021-09-15
pub struct SMBiosInformation<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosInformation<'a> {
    const STRUCT_TYPE: u8 = 0u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosInformation<'a> {
    /// BIOS vendor's name
    pub fn vendor(&self) -> SMBiosString {
        self.parts.get_field_string(0x4)
    }

    /// BIOS version
    ///
    /// This value is a free-form string that may contain
    /// Core and OEM version information.
    pub fn version(&self) -> SMBiosString {
        self.parts.get_field_string(0x5)
    }

    /// BIOS starting address segment
    ///
    /// Segment location of BIOS starting address
    /// (for example, 0E800h).
    ///
    /// When not applicable, such as on UEFI-based systems,
    /// this value is set to 0000h.
    ///
    /// NOTE: The size of the runtime BIOS image can
    /// be computed by subtracting the Starting
    /// Address Segment from 10000h and
    /// multiplying the result by 16.
    pub fn starting_address_segment(&self) -> Option<u16> {
        self.parts.get_field_word(0x6)
    }

    /// BIOS release date
    ///
    /// The date string, if supplied, is in either
    /// mm/dd/yy or mm/dd/yyyy format. If the year
    /// portion of the string is two digits, the year is
    /// assumed to be 19yy.
    ///
    /// NOTE: The mm/dd/yyyy format is required for
    /// SMBIOS version 2.3 and later.
    pub fn release_date(&self) -> SMBiosString {
        self.parts.get_field_string(0x8)
    }

    /// BIOS ROM size
    ///
    /// Size (n) where 64K * (n+1) is the size of the
    /// physical device containing the BIOS, in
    /// bytes.
    ///
    /// FFh - size is 16MB or greater, see Extended
    /// BIOS ROM Size for actual size
    pub fn rom_size(&self) -> Option<RomSize> {
        self.parts.get_field_byte(0x9).map(|raw| RomSize::from(raw))
    }

    /// BIOS characteristics
    ///
    /// Defines which functions the BIOS supports:
    /// PCI, PCMCIA, Flash, etc
    pub fn characteristics(&self) -> Option<BiosCharacteristics> {
        self.parts
            .get_field_dword(0xA)
            .map(|raw| BiosCharacteristics::from(raw))
    }

    /// BIOS vendor reserved characteristics
    pub fn bios_vendor_reserved_characteristics(&self) -> Option<u16> {
        self.parts.get_field_word(0xE)
    }

    /// System vendor reserved characteristics
    pub fn system_vendor_reserved_characteristics(&self) -> Option<u16> {
        self.parts.get_field_word(0x10)
    }

    /// Characteristics extension byte 0
    pub fn characteristics_extension0(&self) -> Option<BiosCharacteristicsExtension0> {
        self.parts
            .get_field_byte(0x12)
            .map(|raw| BiosCharacteristicsExtension0::from(raw))
    }

    /// Characteristics extension byte 1
    pub fn characteristics_extension1(&self) -> Option<BiosCharacteristicsExtension1> {
        self.parts
            .get_field_byte(0x13)
            .map(|raw| BiosCharacteristicsExtension1::from(raw))
    }

    /// System BIOS major release
    ///
    /// Identifies the major release of the System
    /// BIOS; for example, the value is 0Ah for
    /// revision 10.22 and 02h for revision 2.1.
    ///
    /// This field or the System BIOS Minor
    /// Release field or both are updated each time
    /// a System BIOS update for a given system is
    /// released.
    ///
    /// If the system does not support the use of
    /// this field, the value is 0FFh for both this field
    /// and the System BIOS Minor Release field.
    pub fn system_bios_major_release(&self) -> Option<u8> {
        self.parts.get_field_byte(0x14)
    }

    /// System BIOS minor release
    ///
    /// Identifies the minor release of the System
    /// BIOS; for example, the value is 16h for
    /// revision 10.22 and 01h for revision 2.1.
    pub fn system_bios_minor_release(&self) -> Option<u8> {
        self.parts.get_field_byte(0x15)
    }

    /// Embedded controller firmware major release
    ///
    /// Identifies the major release of the
    /// embedded controller firmware; for example,
    /// the value would be 0Ah for revision 10.22
    /// and 02h for revision 2.1.
    ///
    /// This field or the Embedded Controller
    /// Firmware Minor Release field or both are
    /// updated each time an embedded controller
    /// firmware update for a given system is
    /// released.
    ///
    /// If the system does not have field
    /// upgradeable embedded controller firmware,
    /// the value is 0FFh.
    pub fn e_c_firmware_major_release(&self) -> Option<u8> {
        self.parts.get_field_byte(0x16)
    }

    /// Embedded controller firmware minor release
    ///
    /// Identifies the minor release of the
    /// embedded controller firmware; for example,
    /// the value is 16h for revision 10.22 and 01h
    /// for revision 2.1.
    /// If the system does not have field
    /// upgradeable embedded controller firmware,
    /// the value is 0FFh.
    pub fn e_c_firmware_minor_release(&self) -> Option<u8> {
        self.parts.get_field_byte(0x17)
    }

    /// Extended BIOS ROM size
    ///
    /// Extended size of the physical device(s)
    /// containing the BIOS, rounded up if needed.
    ///
    /// Bits 15:14 Unit
    /// 00b - megabytes
    /// 01b - gigabytes
    /// 10b - reserved
    /// 11b - reserved
    /// Bits 13:0 Size
    ///
    /// Examples: a 16 MB device would be
    /// represented as 0010h. A 48 GB device set
    /// would be represented as
    /// 0100_0000_0011_0000b or 4030h.
    pub fn extended_rom_size(&self) -> Option<RomSize> {
        self.parts
            .get_field_word(0x18)
            .map(|raw| RomSize::from(raw))
    }
}

/// # BIOS ROM size
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum RomSize {
    /// Size of this rom in bytes
    Kilobytes(u16),
    /// Extended size of the physical device(s)
    /// containing the BIOS (in MB).
    Megabytes(u16),
    /// Extended size of the physical device(s)
    /// containing the BIOS (in GB).
    Gigabytes(u16),
    /// Extended size of the physical device(s)
    /// containing the BIOS in raw form.
    ///
    /// The standard currently only defines MB and GB
    /// as given in the high nibble (bits 15-14)
    Undefined(u16),
    /// The value present is 16MB or greater and must be found using `extended_rom_size`
    SeeExtendedRomSize,
}

impl From<u16> for RomSize {
    fn from(raw: u16) -> Self {
        // Bits 15:14 Unit
        // 00b - megabytes
        // 01b - gigabytes
        // 10b - reserved
        // 11b - reserved
        // Bits 13:0 Size
        let unit = raw & 0b11000000_00000000; // 15:14 mask
        let size = raw & 0b00111111_11111111; // 13:0 mask

        if unit == 0b00000000_00000000 {
            RomSize::Megabytes(size)
        } else if unit == 0b01000000_00000000 {
            RomSize::Gigabytes(size)
        } else {
            RomSize::Undefined(raw)
        }
    }
}

impl From<u8> for RomSize {
    fn from(raw: u8) -> Self {
        match raw {
            0xFF => RomSize::SeeExtendedRomSize,
            // Size (n) where 64K * (n+1) is the size of the
            // physical device containing the BIOS, in bytes.
            _ => RomSize::Kilobytes(64 * (raw as u16 + 1)),
        }
    }
}

impl fmt::Debug for SMBiosInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<SMBiosInformation<'_>>())
            .field("header", &self.parts.header)
            .field("vendor", &self.vendor())
            .field("version", &self.version())
            .field("starting_address_segment", &self.starting_address_segment())
            .field("release_date", &self.release_date())
            .field("rom_size", &self.rom_size())
            .field("characteristics", &self.characteristics())
            .field(
                "bios_vendor_reserved_characteristics",
                &self.bios_vendor_reserved_characteristics(),
            )
            .field(
                "system_vendor_reserved_characteristics",
                &self.system_vendor_reserved_characteristics(),
            )
            .field(
                "characteristics_extension0",
                &self.characteristics_extension0(),
            )
            .field(
                "characteristics_extension1",
                &self.characteristics_extension1(),
            )
            .field(
                "system_bios_major_release",
                &self.system_bios_major_release(),
            )
            .field(
                "system_bios_minor_release",
                &self.system_bios_minor_release(),
            )
            .field(
                "e_c_firmware_major_release",
                &self.e_c_firmware_major_release(),
            )
            .field(
                "e_c_firmware_minor_release",
                &self.e_c_firmware_minor_release(),
            )
            .field("extended_rom_size", &self.extended_rom_size())
            .finish()
    }
}

impl Serialize for SMBiosInformation<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosInformation", 16)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field("vendor", &self.vendor())?;
        state.serialize_field("version", &self.version())?;
        state.serialize_field("starting_address_segment", &self.starting_address_segment())?;
        state.serialize_field("release_date", &self.release_date())?;
        state.serialize_field("rom_size", &self.rom_size())?;
        state.serialize_field("characteristics", &self.characteristics())?;
        state.serialize_field(
            "bios_vendor_reserved_characteristics",
            &self.bios_vendor_reserved_characteristics(),
        )?;
        state.serialize_field(
            "system_vendor_reserved_characteristics",
            &self.system_vendor_reserved_characteristics(),
        )?;
        state.serialize_field(
            "characteristics_extension0",
            &self.characteristics_extension0(),
        )?;
        state.serialize_field(
            "characteristics_extension1",
            &self.characteristics_extension1(),
        )?;
        state.serialize_field(
            "system_bios_major_release",
            &self.system_bios_major_release(),
        )?;
        state.serialize_field(
            "system_bios_minor_release",
            &self.system_bios_minor_release(),
        )?;
        state.serialize_field(
            "e_c_firmware_major_release",
            &self.e_c_firmware_major_release(),
        )?;
        state.serialize_field(
            "e_c_firmware_minor_release",
            &self.e_c_firmware_minor_release(),
        )?;
        state.serialize_field("extended_rom_size", &self.extended_rom_size())?;
        state.end()
    }
}

/// # BIOS Characteristics
#[derive(PartialEq, Eq)]
pub struct BiosCharacteristics {
    /// Raw value
    pub raw: u32,
}

impl Deref for BiosCharacteristics {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl From<u32> for BiosCharacteristics {
    fn from(raw: u32) -> Self {
        BiosCharacteristics { raw }
    }
}

impl BiosCharacteristics {
    /// Unknown.
    pub fn unknown(&self) -> bool {
        self.raw & 0x00000004 == 0x00000004
    }

    /// BIOS Characteristics are not supported.
    pub fn bios_characteristics_not_supported(&self) -> bool {
        self.raw & 0x00000008 == 0x00000008
    }

    /// ISA is supported.
    pub fn isa_supported(&self) -> bool {
        self.raw & 0x00000010 == 0x00000010
    }

    /// MCA is supported.
    pub fn mca_supported(&self) -> bool {
        self.raw & 0x00000020 == 0x00000020
    }

    /// EISA is supported.
    pub fn eisa_supported(&self) -> bool {
        self.raw & 0x00000040 == 0x00000040
    }

    /// PCI is supported.
    pub fn pci_supported(&self) -> bool {
        self.raw & 0x00000080 == 0x00000080
    }

    /// PC card (PCMCIA) is supported.
    pub fn pcmcia_supported(&self) -> bool {
        self.raw & 0x00000100 == 0x00000100
    }

    /// Plug and Play is supported.
    pub fn plug_and_play_supported(&self) -> bool {
        self.raw & 0x00000200 == 0x00000200
    }

    /// APM is supported.
    pub fn apm_supported(&self) -> bool {
        self.raw & 0x00000400 == 0x00000400
    }

    /// BIOS is upgradeable (Flash).
    pub fn bios_upgradeable(&self) -> bool {
        self.raw & 0x00000800 == 0x00000800
    }

    /// BIOS shadowing is allowed.
    pub fn bios_shadowing_allowed(&self) -> bool {
        self.raw & 0x00001000 == 0x00001000
    }

    /// VL-VESA is supported.
    pub fn vlvesa_supported(&self) -> bool {
        self.raw & 0x00002000 == 0x00002000
    }

    /// ESCD support is available.
    pub fn escd_support_available(&self) -> bool {
        self.raw & 0x00004000 == 0x00004000
    }

    /// Boot from CD is supported.
    pub fn boot_from_cdsupported(&self) -> bool {
        self.raw & 0x00008000 == 0x00008000
    }

    /// Selectable boot is supported.
    pub fn selectable_boot_supported(&self) -> bool {
        self.raw & 0x00010000 == 0x00010000
    }

    /// BIOS ROM is socketed (e.g. PLCC or SOP socket).
    pub fn bios_rom_socketed(&self) -> bool {
        self.raw & 0x00020000 == 0x00020000
    }

    /// Boot from PC card (PCMCIA) is supported.
    pub fn boot_from_pcmcia_supported(&self) -> bool {
        self.raw & 0x00040000 == 0x00040000
    }

    /// EDD specification is supported.
    pub fn edd_specification_supported(&self) -> bool {
        self.raw & 0x00080000 == 0x00080000
    }

    /// Int 13h — Japanese floppy for NEC 9800 1.2 MB (3.5”, 1K bytes/sector, 360 RPM) is supported.
    pub fn floppy_nec_japanese_supported(&self) -> bool {
        self.raw & 0x00100000 == 0x00100000
    }

    /// Int 13h — Japanese floppy for Toshiba 1.2 MB (3.5”, 360 RPM) is supported.
    pub fn floppy_toshiba_japanese_supported(&self) -> bool {
        self.raw & 0x00200000 == 0x00200000
    }

    /// Int 13h — 5.25” / 360 KB floppy services are supported.
    pub fn floppy_525_360_supported(&self) -> bool {
        self.raw & 0x00400000 == 0x00400000
    }

    /// Int 13h — 5.25” /1.2 MB floppy services are supported.
    pub fn floppy_525_12_supported(&self) -> bool {
        self.raw & 0x00800000 == 0x00800000
    }

    /// Int 13h — 3.5” / 720 KB floppy services are supported.
    pub fn floppy_35_720_supported(&self) -> bool {
        self.raw & 0x01000000 == 0x01000000
    }

    /// Int 13h — 3.5” / 2.88 MB floppy services are supported.
    pub fn floppy_35_288_supported(&self) -> bool {
        self.raw & 0x02000000 == 0x02000000
    }

    /// Int 5h, print screen Service is supported.
    pub fn print_screen_service_supported(&self) -> bool {
        self.raw & 0x04000000 == 0x04000000
    }

    /// Int 9h, 8042 keyboard services are supported.
    pub fn keyboard_8042services_supported(&self) -> bool {
        self.raw & 0x08000000 == 0x08000000
    }

    /// Int 14h, serial services are supported.
    pub fn serial_services_supported(&self) -> bool {
        self.raw & 0x10000000 == 0x10000000
    }

    /// Int 17h, printer services are supported.
    pub fn printer_services_supported(&self) -> bool {
        self.raw & 0x20000000 == 0x20000000
    }

    /// Int 10h, CGA/Mono Video Services are supported.
    pub fn cga_mono_video_services_supported(&self) -> bool {
        self.raw & 0x40000000 == 0x40000000
    }

    /// NEC PC-98.
    pub fn nec_pc_98supported(&self) -> bool {
        self.raw & 0x80000000 == 0x80000000
    }
}

impl fmt::Debug for BiosCharacteristics {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<BiosCharacteristics>())
            .field("raw", &self.raw)
            .field("unknown", &self.unknown())
            .field(
                "bios_characteristics_not_supported",
                &self.bios_characteristics_not_supported(),
            )
            .field("isa_supported", &self.isa_supported())
            .field("mca_supported", &self.mca_supported())
            .field("eisa_supported", &self.eisa_supported())
            .field("pci_supported", &self.pci_supported())
            .field("pcmcia_supported", &self.pcmcia_supported())
            .field("plug_and_play_supported", &self.plug_and_play_supported())
            .field("apm_supported", &self.apm_supported())
            .field("bios_upgradeable", &self.bios_upgradeable())
            .field("bios_shadowing_allowed", &self.bios_shadowing_allowed())
            .field("vlvesa_supported", &self.vlvesa_supported())
            .field("escd_support_available", &self.escd_support_available())
            .field("boot_from_cdsupported", &self.boot_from_cdsupported())
            .field(
                "selectable_boot_supported",
                &self.selectable_boot_supported(),
            )
            .field("bios_rom_socketed", &self.bios_rom_socketed())
            .field(
                "boot_from_pcmcia_supported",
                &self.boot_from_pcmcia_supported(),
            )
            .field(
                "edd_specification_supported",
                &self.edd_specification_supported(),
            )
            .field(
                "floppy_nec_japanese_supported",
                &self.floppy_nec_japanese_supported(),
            )
            .field(
                "floppy_toshiba_japanese_supported",
                &self.floppy_toshiba_japanese_supported(),
            )
            .field("floppy_525_360_supported", &self.floppy_525_360_supported())
            .field("floppy_525_12_supported", &self.floppy_525_12_supported())
            .field("floppy_35_720_supported", &self.floppy_35_720_supported())
            .field("floppy_35_288_supported", &self.floppy_35_288_supported())
            .field(
                "print_screen_service_supported",
                &self.print_screen_service_supported(),
            )
            .field(
                "keyboard_8042services_supported",
                &self.keyboard_8042services_supported(),
            )
            .field(
                "serial_services_supported",
                &self.serial_services_supported(),
            )
            .field(
                "printer_services_supported",
                &self.printer_services_supported(),
            )
            .field(
                "cga_mono_video_services_supported",
                &self.cga_mono_video_services_supported(),
            )
            .field("nec_pc_98supported", &self.nec_pc_98supported())
            .finish()
    }
}

impl Serialize for BiosCharacteristics {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("BiosCharacteristics", 31)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("unknown", &self.unknown())?;
        state.serialize_field(
            "bios_characteristics_not_supported",
            &self.bios_characteristics_not_supported(),
        )?;
        state.serialize_field("isa_supported", &self.isa_supported())?;
        state.serialize_field("mca_supported", &self.mca_supported())?;
        state.serialize_field("eisa_supported", &self.eisa_supported())?;
        state.serialize_field("pci_supported", &self.pci_supported())?;
        state.serialize_field("pcmcia_supported", &self.pcmcia_supported())?;
        state.serialize_field("plug_and_play_supported", &self.plug_and_play_supported())?;
        state.serialize_field("apm_supported", &self.apm_supported())?;
        state.serialize_field("bios_upgradeable", &self.bios_upgradeable())?;
        state.serialize_field("bios_shadowing_allowed", &self.bios_shadowing_allowed())?;
        state.serialize_field("vlvesa_supported", &self.vlvesa_supported())?;
        state.serialize_field("escd_support_available", &self.escd_support_available())?;
        state.serialize_field("boot_from_cdsupported", &self.boot_from_cdsupported())?;
        state.serialize_field(
            "selectable_boot_supported",
            &self.selectable_boot_supported(),
        )?;
        state.serialize_field("bios_rom_socketed", &self.bios_rom_socketed())?;
        state.serialize_field(
            "boot_from_pcmcia_supported",
            &self.boot_from_pcmcia_supported(),
        )?;
        state.serialize_field(
            "edd_specification_supported",
            &self.edd_specification_supported(),
        )?;
        state.serialize_field(
            "floppy_nec_japanese_supported",
            &self.floppy_nec_japanese_supported(),
        )?;
        state.serialize_field(
            "floppy_toshiba_japanese_supported",
            &self.floppy_toshiba_japanese_supported(),
        )?;
        state.serialize_field("floppy_525_360_supported", &self.floppy_525_360_supported())?;
        state.serialize_field("floppy_525_12_supported", &self.floppy_525_12_supported())?;
        state.serialize_field("floppy_35_720_supported", &self.floppy_35_720_supported())?;
        state.serialize_field("floppy_35_288_supported", &self.floppy_35_288_supported())?;
        state.serialize_field(
            "print_screen_service_supported",
            &self.print_screen_service_supported(),
        )?;
        state.serialize_field(
            "keyboard_8042services_supported",
            &self.keyboard_8042services_supported(),
        )?;
        state.serialize_field(
            "serial_services_supported",
            &self.serial_services_supported(),
        )?;
        state.serialize_field(
            "printer_services_supported",
            &self.printer_services_supported(),
        )?;
        state.serialize_field(
            "cga_mono_video_services_supported",
            &self.cga_mono_video_services_supported(),
        )?;
        state.serialize_field("nec_pc_98supported", &self.nec_pc_98supported())?;
        state.end()
    }
}

/// # BIOS Characteristics Extension Byte 0
#[derive(PartialEq, Eq)]
pub struct BiosCharacteristicsExtension0 {
    /// Raw value
    pub raw: u8,
}

impl Deref for BiosCharacteristicsExtension0 {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl From<u8> for BiosCharacteristicsExtension0 {
    fn from(raw: u8) -> Self {
        BiosCharacteristicsExtension0 { raw }
    }
}

impl BiosCharacteristicsExtension0 {
    /// ACPI is supported.
    pub fn acpi_is_supported(&self) -> bool {
        self.raw & 0b0000_0001 == 0b0000_0001
    }

    /// USB Legacy is supported.
    pub fn usb_legacy_is_supported(&self) -> bool {
        self.raw & 0b0000_0010 == 0b0000_0010
    }

    /// AGP is supported.
    pub fn agp_is_supported(&self) -> bool {
        self.raw & 0b0000_0100 == 0b0000_0100
    }

    /// I2O boot is supported.
    pub fn i2oboot_is_supported(&self) -> bool {
        self.raw & 0b0000_1000 == 0b0000_1000
    }

    /// LS-120 SuperDisk boot is supported.
    pub fn ls120super_disk_boot_is_supported(&self) -> bool {
        self.raw & 0b0001_0000 == 0b0001_0000
    }

    /// ATAPI ZIP drive boot is supported.
    pub fn atapi_zip_drive_boot_is_supported(&self) -> bool {
        self.raw & 0b0010_0000 == 0b0010_0000
    }

    /// 1394 boot is supported.
    pub fn boot_1394is_supported(&self) -> bool {
        self.raw & 0b0100_0000 == 0b0100_0000
    }

    /// Smart battery is supported.
    pub fn smart_battery_is_supported(&self) -> bool {
        self.raw & 0b1000_0000 == 0b1000_0000
    }
}

impl fmt::Debug for BiosCharacteristicsExtension0 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<BiosCharacteristicsExtension0>())
            .field("raw", &self.raw)
            .field("acpi_is_supported", &self.acpi_is_supported())
            .field("usb_legacy_is_supported", &self.usb_legacy_is_supported())
            .field("agp_is_supported", &self.agp_is_supported())
            .field("i2oboot_is_supported", &self.i2oboot_is_supported())
            .field(
                "ls120super_disk_boot_is_supported",
                &self.ls120super_disk_boot_is_supported(),
            )
            .field(
                "atapi_zip_drive_boot_is_supported",
                &self.atapi_zip_drive_boot_is_supported(),
            )
            .field("boot_1394is_supported", &self.boot_1394is_supported())
            .field(
                "smart_battery_is_supported",
                &self.smart_battery_is_supported(),
            )
            .finish()
    }
}

impl Serialize for BiosCharacteristicsExtension0 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("BiosCharacteristicsExtension0", 9)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("acpi_is_supported", &self.acpi_is_supported())?;
        state.serialize_field("usb_legacy_is_supported", &self.usb_legacy_is_supported())?;
        state.serialize_field("agp_is_supported", &self.agp_is_supported())?;
        state.serialize_field("i2oboot_is_supported", &self.i2oboot_is_supported())?;
        state.serialize_field(
            "ls120super_disk_boot_is_supported",
            &self.ls120super_disk_boot_is_supported(),
        )?;
        state.serialize_field(
            "atapi_zip_drive_boot_is_supported",
            &self.atapi_zip_drive_boot_is_supported(),
        )?;
        state.serialize_field("boot_1394is_supported", &self.boot_1394is_supported())?;
        state.serialize_field(
            "smart_battery_is_supported",
            &self.smart_battery_is_supported(),
        )?;
        state.end()
    }
}

/// # BIOS Characteristics Extension Byte 1
#[derive(PartialEq, Eq)]
pub struct BiosCharacteristicsExtension1 {
    /// Raw value
    pub raw: u8,
}

impl Deref for BiosCharacteristicsExtension1 {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl From<u8> for BiosCharacteristicsExtension1 {
    fn from(raw: u8) -> Self {
        BiosCharacteristicsExtension1 { raw }
    }
}

impl BiosCharacteristicsExtension1 {
    /// BIOS Boot Specification is supported.
    ///
    /// Available version 2.3.0 and later.
    pub fn bios_boot_specification_is_supported(&self) -> bool {
        self.raw & 0b0000_0001 == 0b0000_0001
    }

    /// Function key-initiated network service boot is supported. When function key-uninitiated
    /// network service boot is not supported, a network adapter option ROM may choose to offer
    /// this functionality on its own, thus offering this capability to legacy systems. When the
    /// function is supported, the network adapter option ROM shall not offer this capability.
    ///
    /// Available version 2.3.1 and later.
    pub fn fkey_initiated_network_boot_is_supported(&self) -> bool {
        self.raw & 0b0000_0010 == 0b0000_0010
    }

    /// Enable targeted content distribution. The manufacturer has ensured that the SMBIOS data
    /// is useful in identifying the computer for targeted delivery of model-specific software and
    /// firmware content through third-party content distribution services.
    ///
    /// Available version 2.4 and later.
    pub fn targeted_content_distribution_is_supported(&self) -> bool {
        self.raw & 0b0000_0100 == 0b0000_0100
    }

    /// UEFI Specification is supported.
    ///
    /// Available version 2.7 and later.
    pub fn uefi_specification_is_supported(&self) -> bool {
        self.raw & 0b0000_1000 == 0b0000_1000
    }

    /// SMBIOS table describes a virtual machine. (If this bit is not set, no inference can be made
    /// about the virtuality of the system.)
    ///
    /// Available version 2.7 and later.
    pub fn smbios_table_describes_avirtual_machine(&self) -> bool {
        self.raw & 0b0001_0000 == 0b0001_0000
    }

    /// Manufacturing mode is supported. (Manufacturing mode is a special boot mode, not normally
    /// available to end users, that modifies BIOS features and settings for use while the computer is being
    /// manufactured and tested.)
    ///
    /// Available version 3.5 and later.
    pub fn manufacturing_mode_is_supported(&self) -> bool {
        self.raw & 0b0010_0000 == 0b0010_0000
    }

    /// Manufacturing mode is enabled.
    ///
    /// Available version 3.5 and later.
    pub fn manufacturing_mode_is_enabled(&self) -> bool {
        self.raw & 0b0100_0000 == 0b0100_0000
    }
}

impl fmt::Debug for BiosCharacteristicsExtension1 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<BiosCharacteristicsExtension1>())
            .field("raw", &self.raw)
            .field(
                "bios_boot_specification_is_supported",
                &self.bios_boot_specification_is_supported(),
            )
            .field(
                "fkey_initiated_network_boot_is_supported",
                &self.fkey_initiated_network_boot_is_supported(),
            )
            .field(
                "targeted_content_distribution_is_supported",
                &self.targeted_content_distribution_is_supported(),
            )
            .field(
                "uefi_specification_is_supported",
                &self.uefi_specification_is_supported(),
            )
            .field(
                "smbios_table_describes_avirtual_machine",
                &self.smbios_table_describes_avirtual_machine(),
            )
            .field(
                "manufacturing_mode_is_supported",
                &self.manufacturing_mode_is_supported(),
            )
            .field(
                "manufacturing_mode_is_enabled",
                &self.manufacturing_mode_is_enabled(),
            )
            .finish()
    }
}

impl Serialize for BiosCharacteristicsExtension1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("BiosCharacteristicsExtension1", 6)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field(
            "bios_boot_specification_is_supported",
            &self.bios_boot_specification_is_supported(),
        )?;
        state.serialize_field(
            "fkey_initiated_network_boot_is_supported",
            &self.fkey_initiated_network_boot_is_supported(),
        )?;
        state.serialize_field(
            "targeted_content_distribution_is_supported",
            &self.targeted_content_distribution_is_supported(),
        )?;
        state.serialize_field(
            "uefi_specification_is_supported",
            &self.uefi_specification_is_supported(),
        )?;
        state.serialize_field(
            "smbios_table_describes_avirtual_machine",
            &self.smbios_table_describes_avirtual_machine(),
        )?;
        state.serialize_field(
            "manufacturing_mode_is_supported",
            &self.manufacturing_mode_is_supported(),
        )?;
        state.serialize_field(
            "manufacturing_mode_is_enabled",
            &self.manufacturing_mode_is_enabled(),
        )?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        // BIOS Information structure is sensitive to BIOS specification versions
        // and prone to bugs.  Therefore, it is important to test different
        // structure versions.
        //
        // The length field specifies:
        // 12h + number of BIOS Characteristics
        // Extension Bytes. If no Extension Bytes are
        // used the Length is 12h.
        //
        // For version 2.1 and 2.2 implementations, the length is 13h
        // because one extension byte is defined.
        //
        // For version 2.3 and later implementations, the
        // length is at least 14h because two extension
        // bytes are defined.
        //
        // For version 2.4 to 3.0, implementations, the length
        // is at least 18h because bytes 14-17h are defined.
        //
        // For version 3.1 and later implementations, the
        // length is at least 1Ah because bytes 14-19h
        // are defined.

        // 2.4 to 3.0 BIOS Information structure.  Does not include _extended_rom_size()_
        // field or fields beyond.
        let struct_type0 = vec![
            0x00, 0x18, 0x00, 0x00, 0x01, 0x02, 0x00, 0xF0, 0x03, 0xFF, 0x80, 0x98, 0x8B, 0x3F,
            0x01, 0x00, 0x11, 0x00, 0x03, 0x0D, 0x00, 0x21, 0x11, 0x2D, 0x4C, 0x45, 0x4E, 0x4F,
            0x56, 0x4F, 0x00, 0x53, 0x30, 0x33, 0x4B, 0x54, 0x33, 0x33, 0x41, 0x00, 0x30, 0x38,
            0x2F, 0x30, 0x36, 0x2F, 0x32, 0x30, 0x31, 0x39, 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type0);
        let test_struct = SMBiosInformation::new(&parts);

        assert_eq!(test_struct.vendor().to_string(), "LENOVO".to_string());
        assert_eq!(test_struct.version().to_string(), "S03KT33A".to_string());
        assert_eq!(test_struct.starting_address_segment(), Some(61440));
        assert_eq!(
            test_struct.release_date().to_string(),
            "08/06/2019".to_string()
        );
        assert_eq!(test_struct.rom_size(), Some(RomSize::SeeExtendedRomSize));
        assert_eq!(
            test_struct.characteristics(),
            Some(BiosCharacteristics::from(1066113152))
        );
        assert_eq!(test_struct.bios_vendor_reserved_characteristics(), Some(1));
        assert_eq!(
            test_struct.system_vendor_reserved_characteristics(),
            Some(17)
        );
        assert_eq!(
            test_struct.characteristics_extension0(),
            Some(BiosCharacteristicsExtension0::from(3))
        );
        assert_eq!(
            test_struct.characteristics_extension1(),
            Some(BiosCharacteristicsExtension1::from(13))
        );
        assert_eq!(test_struct.system_bios_major_release(), Some(0));
        assert_eq!(test_struct.system_bios_minor_release(), Some(33));
        assert_eq!(test_struct.e_c_firmware_major_release(), Some(17));
        assert_eq!(test_struct.e_c_firmware_minor_release(), Some(45));

        // 2.4 to 3.0 BIOS Information does not include _extended_rom_size()_ or
        // fields beyond.
        assert!(test_struct.extended_rom_size().is_none());

        // 3.1 BIOS (includes _extended_rom_size_)
        let struct_type0 = vec![
            0x00, 0x1A, 0x00, 0x00, 0x01, 0x02, 0x00, 0xF0, 0x03, 0xFF, 0x80, 0x98, 0x8B, 0x3F,
            0x01, 0x00, 0x11, 0x00, 0x03, 0x0D, 0x00, 0x21, 0x11, 0x2D, 0x30, 0x40, 0x4C, 0x45,
            0x4E, 0x4F, 0x56, 0x4F, 0x00, 0x53, 0x30, 0x33, 0x4B, 0x54, 0x33, 0x33, 0x41, 0x00,
            0x30, 0x38, 0x2F, 0x30, 0x36, 0x2F, 0x32, 0x30, 0x31, 0x39, 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type0);
        let test_struct = SMBiosInformation::new(&parts);

        let extended_rom_size = test_struct.extended_rom_size().unwrap();
        assert_eq!(extended_rom_size, RomSize::from(0x4030u16));

        match extended_rom_size {
            RomSize::Gigabytes(size) => assert_eq!(size, 48),
            _ => panic!("incorrect unit"),
        }

        println!("{:?}", test_struct);
    }

    #[test]
    pub fn test_rom_size() {
        let struct_type0 = vec![
            0x00, 0x18, 0x00, 0x00, 0x01, 0x02, 0x00, 0xF0, 0x03, 0xFE, 0x80, 0x98, 0x8B, 0x3F,
            0x01, 0x00, 0x11, 0x00, 0x03, 0x0D, 0x00, 0x21, 0x11, 0x2D, 0x4C, 0x45, 0x4E, 0x4F,
            0x56, 0x4F, 0x00, 0x53, 0x30, 0x33, 0x4B, 0x54, 0x33, 0x33, 0x41, 0x00, 0x30, 0x38,
            0x2F, 0x30, 0x36, 0x2F, 0x32, 0x30, 0x31, 0x39, 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type0);
        let test_struct = SMBiosInformation::new(&parts);
        assert_eq!(test_struct.rom_size(), Some(RomSize::Kilobytes(16320)))
    }
}
