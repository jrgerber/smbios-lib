use super::*;

/// # Baseboard (or Module) Information (Type 2)
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosBaseboardInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosBaseboardInformation<'a> {
    const STRUCT_TYPE: u8 = 2u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosBaseboardInformation<'a> {
    ///Baseboard manufacturer
    pub fn manufacturer(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    /// Baseboard product
    pub fn product(&self) -> Option<String> {
        self.parts.get_field_string(0x05)
    }

    /// Baseboard version
    pub fn version(&self) -> Option<String> {
        self.parts.get_field_string(0x06)
    }

    /// Baseboard serial number
    pub fn serial_number(&self) -> Option<String> {
        self.parts.get_field_string(0x07)
    }

    /// Baseboard asset tag
    pub fn asset_tag(&self) -> Option<String> {
        self.parts.get_field_string(0x08)
    }

    /// Collection of flags that identify features of this baseboard.
    pub fn feature_flags(&self) -> Option<u8> {
        self.parts.get_field_byte(0x09)
    }

    /// This baseboard's location within the chassis (chassis is referenced by ChassisHandle).
    pub fn location_in_chassis(&self) -> Option<String> {
        self.parts.get_field_string(0x0A)
    }

    /// Handle, or instance number, associated with the chassis in which this board resides.
    pub fn chassis_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0x0B)
    }

    /// Type of baseboard.
    pub fn board_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0D)
    }

    /// The count of ObjectHandles.
    pub fn number_of_contained_object_handles(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0E)
    }

    /// List of handles of other structures that are contained by this baseboard.
    pub fn contained_object_handle_iterator(&'a self) -> ObjectHandleIterator<'a> {
        ObjectHandleIterator::new(self)
    }
}

impl fmt::Debug for SMBiosBaseboardInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosBaseboardInformation>())
            .field("header", &self.parts.header)
            .field("manufacturer", &self.manufacturer())
            .field("product", &self.product())
            .field("version", &self.version())
            .field("serial_number", &self.serial_number())
            .field("asset_tag", &self.asset_tag())
            .field("feature_flags", &self.feature_flags())
            .field("location_in_chassis", &self.location_in_chassis())
            .field("chassis_handle", &self.chassis_handle())
            .field("board_type", &self.board_type())
            .field(
                "number_of_contained_object_handles",
                &self.number_of_contained_object_handles(),
            )
            .field(
                "contained_object_handle_iterator",
                &self.contained_object_handle_iterator(),
            )
            .finish()
    }
}

/// # Object Handle Iterator
///
/// Iterates over the object handles contained within the [SMBiosBaseboardInformation] structure
pub struct ObjectHandleIterator<'a> {
    data: &'a SMBiosBaseboardInformation<'a>,
    current_index: usize,
    current_entry: u8,
    number_of_contained_object_handles: u8,
}

impl<'a> ObjectHandleIterator<'a> {
    const OBJECT_HANDLES_OFFSET: usize = 0x0Fusize;
    const HANDLE_SIZE: usize = 2usize;

    /// Creates an instance of the object handle iterator.
    pub fn new(data: &'a SMBiosBaseboardInformation<'a>) -> Self {
        ObjectHandleIterator {
            data: data,
            current_index: Self::OBJECT_HANDLES_OFFSET,
            current_entry: 0,
            number_of_contained_object_handles: data
                .number_of_contained_object_handles()
                .unwrap_or(0),
        }
    }

    fn reset(&mut self) {
        self.current_index = Self::OBJECT_HANDLES_OFFSET;
        self.current_entry = 0;
    }
}

impl<'a> IntoIterator for &'a ObjectHandleIterator<'a> {
    type Item = Handle;
    type IntoIter = ObjectHandleIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ObjectHandleIterator {
            data: self.data,
            current_index: ObjectHandleIterator::OBJECT_HANDLES_OFFSET,
            current_entry: 0,
            number_of_contained_object_handles: self
                .data
                .number_of_contained_object_handles()
                .unwrap_or(0),
        }
    }
}

impl<'a> Iterator for ObjectHandleIterator<'a> {
    type Item = Handle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_entry == self.number_of_contained_object_handles {
            self.reset();
            return None;
        }

        match self.data.parts().get_field_handle(self.current_index) {
            Some(current_handle) => {
                self.current_index = self.current_index + Self::HANDLE_SIZE;
                self.current_entry = self.current_entry + 1;
                Some(current_handle)
            }
            None => {
                self.reset();
                None
            }
        }
    }
}

impl<'a> fmt::Debug for ObjectHandleIterator<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_list().entries(self.into_iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_baseboard_information() {
        let baseboard_information_bytes = vec![
            // struct_type(2), length(0x13), handle(0x10)
            0x02, 0x13, 0x10, 0x00,
            // manufacturer(1), product(2), version(0), serial_number(3), asset_tag(0), feature_flags(1),
            // location_in_chassis(0), chassis_handle(0x0F), board_type(0x0A), number_of_contained_object_handles(2)
            0x01, 0x02, 0x00, 0x03, 0x00, 0x01, 0x00, 0x0F, 0x00, 0x0A, 0x02,
            // handle[0] == 0x0005
            0x05, 0x00, // handle[1] == 0x1200
            0x00, 0x12, // manufacturer: "Microsoft Corporation" (1)
            0x4D, 0x69, 0x63, 0x72, 0x6F, 0x73, 0x6F, 0x66, 0x74, 0x20, 0x43, 0x6F, 0x72, 0x70,
            0x6F, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x00,
            // product: "Surface Laptop 3" (2)
            0x53, 0x75, 0x72, 0x66, 0x61, 0x63, 0x65, 0x20, 0x4C, 0x61, 0x70, 0x74, 0x6F, 0x70,
            0x20, 0x33, 0x00, // serial_number: "B009250100J1939B" (3)
            0x42, 0x30, 0x30, 0x39, 0x32, 0x35, 0x30, 0x31, 0x30, 0x30, 0x4A, 0x31, 0x39, 0x33,
            0x39, 0x42, 0x00, // end of structure
            0x00,
        ];

        let parts = SMBiosStructParts::new(baseboard_information_bytes.as_slice());
        let baseboard_information = SMBiosBaseboardInformation::new(&parts);

        // header tests
        assert_eq!(*baseboard_information.parts().header.handle(), 0x0010);
        assert_eq!(baseboard_information.parts().header.length(), 0x13);

        // basic field tests
        assert_eq!(
            baseboard_information
                .manufacturer()
                .expect("manufacturer field exists"),
            "Microsoft Corporation".to_string()
        );
        assert_eq!(
            baseboard_information
                .product()
                .expect("product field exists"),
            "Surface Laptop 3".to_string()
        );
        assert_eq!(baseboard_information.version().is_none(), true);
        assert_eq!(
            baseboard_information
                .serial_number()
                .expect("serial_number field exists"),
            "B009250100J1939B".to_string()
        );
        assert_eq!(baseboard_information.asset_tag().is_none(), true);
        assert_eq!(
            baseboard_information
                .feature_flags()
                .expect("feature_flags field exists"),
            1
        );
        assert_eq!(baseboard_information.location_in_chassis().is_none(), true);
        assert_eq!(
            *baseboard_information
                .chassis_handle()
                .expect("chassis_handle field exists"),
            0x0F
        );
        assert_eq!(
            baseboard_information
                .board_type()
                .expect("board_type field exists"),
            0x0A
        );

        // contained object handle tests
        assert_eq!(
            baseboard_information
                .number_of_contained_object_handles()
                .expect("2 object handles"),
            2
        );

        let mut iterator = baseboard_information.contained_object_handle_iterator();

        let first_entry = iterator.next().expect("has a first entry");
        assert_eq!(*first_entry, 0x0005);

        let second_entry = iterator.next().expect("has a second entry");
        assert_eq!(*second_entry, 0x1200);

        assert!(iterator.next().is_none());

        let mut counter = 0;

        for _entry in baseboard_information.contained_object_handle_iterator() {
            counter = counter + 1;
        }

        assert_eq!(counter, 2);

        // debug print test
        println!("baseboard_information: {:?}", baseboard_information);
    }
}
