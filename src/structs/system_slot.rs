use super::*;

/// # System Slots (Type 9)
///
/// The information in this structure defines the attributes of a system slot. One
/// structure is provided for each slot in the system.
pub struct SMBiosSystemSlot<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosSystemSlot<'a> {
    const STRUCT_TYPE: u8 = 9u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosSystemSlot<'a> {
    // TODO: this is completely incorrect.  The auto-port apparently failed.
    // There are a bunch of fields that go here.

    // fn data_bus_width(&self) -> Option<u8> {
    //     self.parts.get_field_byte(0x04)
    // }
}

impl fmt::Debug for SMBiosSystemSlot<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosSystemSlot>())
            .field("header", &self.parts.header)
            // .field("data_bus_width", &self.data_bus_width())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type9 = vec![
            0x09, 0x11, 0x1C, 0x00, 0x01, 0xA5, 0x0D, 0x04, 0x04, 0x00, 0x00, 0x0C, 0x01, 0x00,
            0x00, 0x00, 0x08, 0x4A, 0x36, 0x42, 0x32, 0x00, 0x00,
        ];

        let parts = SMBiosStructParts::new(struct_type9.as_slice());
        let test_struct = SMBiosSystemSlot::new(&parts);
    }
}
