use crate::*;

/// # Baseboard (or Module) Information (Type 2)
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosBaseboardInformation<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosBaseboardInformation<'a> {
    const STRUCT_TYPE: u8 = 2u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
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
    pub fn feature_flags(&self) -> Option<BaseboardFeatures> {
        self.parts
            .get_field_byte(0x09)
            .and_then(|raw| Some(BaseboardFeatures::from(raw)))
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
    pub fn board_type(&self) -> Option<BoardTypeData> {
        self.parts
            .get_field_byte(0x0D)
            .and_then(|raw| Some(BoardTypeData::from(raw)))
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
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosBaseboardInformation<'_>>())
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

/// # Board Type Data
pub struct BoardTypeData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [BoardType] value
    pub value: BoardType,
}

impl fmt::Debug for BoardTypeData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<BoardTypeData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Deref for BoardTypeData {
    type Target = BoardType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<u8> for BoardTypeData {
    fn from(raw: u8) -> Self {
        BoardTypeData {
            value: match raw {
                0x01 => BoardType::Other,
                0x02 => BoardType::Unknown,
                0x03 => BoardType::ServerBlade,
                0x04 => BoardType::ConnectivitySwitch,
                0x05 => BoardType::SystemManagementModule,
                0x06 => BoardType::ProcessorModule,
                0x07 => BoardType::IOModule,
                0x08 => BoardType::MemoryModule,
                0x09 => BoardType::Daughterboard,
                0x0A => BoardType::Motherboard,
                0x0B => BoardType::ProcessorMemoryModule,
                0x0C => BoardType::ProcessorIOModule,
                0x0D => BoardType::InterconnectBoard,
                _ => BoardType::None,
            },
            raw,
        }
    }
}

/// # Board Type
#[derive(Debug, PartialEq, Eq)]
pub enum BoardType {
    /// Unknown
    Unknown,
    /// Other
    Other,
    /// Server Blade
    ServerBlade,
    /// Connectivity Switch
    ConnectivitySwitch,
    /// System Management Module
    SystemManagementModule,
    /// Processor Module
    ProcessorModule,
    /// I/O Module
    IOModule,
    /// Memory Module
    MemoryModule,
    /// Daughter board
    Daughterboard,
    /// Motherboard (includes processor, memory, and I/O)
    Motherboard,
    /// Processor/Memory Module
    ProcessorMemoryModule,
    /// Processor/IO Module
    ProcessorIOModule,
    /// Interconnect board
    InterconnectBoard,
    /// A value unknown to this standard, check the raw value
    None,
}

/// # Baseboard Features
#[derive(PartialEq, Eq)]
pub struct BaseboardFeatures {
    raw: u8,
}

impl Deref for BaseboardFeatures {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl From<u8> for BaseboardFeatures {
    fn from(raw: u8) -> Self {
        BaseboardFeatures { raw }
    }
}

impl BaseboardFeatures {
    /// Set if the board is a hosting board (for example, a motherboard).
    pub fn hosting_board(&self) -> bool {
        self.raw & 0x01 == 0x01
    }

    /// Set if the board requires at least one daughter board or auxiliary card to function properly.
    pub fn requires_daughterboard(&self) -> bool {
        self.raw & 0x02 == 0x02
    }

    /// Set if the board is removable; it is designed to be taken in and out of the chassis without impairing the function of the chassis.
    pub fn is_removable(&self) -> bool {
        self.raw & 0x04 == 0x04
    }

    /// Set if the board is replaceable; it is possible to replace (either as a field repair or as an upgrade) the board with a physically different board. The board is inherently removable.
    pub fn is_replaceable(&self) -> bool {
        self.raw & 0x08 == 0x08
    }

    /// Set if the board is hot swappable; it is possible to replace the board with a physically different but equivalent board while power is applied to the board. The board is inherently replaceable and removable.
    pub fn is_hot_swappable(&self) -> bool {
        self.raw & 0x10 == 0x10
    }
}

impl fmt::Debug for BaseboardFeatures {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<BaseboardFeatures>())
            .field("raw", &self.raw)
            .field("hosting_board", &self.hosting_board())
            .field("requires_daughterboard", &self.requires_daughterboard())
            .field("is_removable", &self.is_removable())
            .field("is_replaceable", &self.is_replaceable())
            .field("is_hot_swappable", &self.is_hot_swappable())
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
                self.current_index = self.current_index + Handle::SIZE;
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
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
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

        let parts = UndefinedStruct::new(&baseboard_information_bytes);
        let baseboard_information = SMBiosBaseboardInformation::new(&parts);

        // header tests
        assert_eq!(*baseboard_information.parts().header.handle(), 0x0010);
        assert_eq!(baseboard_information.parts().header.length(), 0x13);

        // basic field tests
        assert_eq!(
            baseboard_information.manufacturer().unwrap(),
            "Microsoft Corporation".to_string()
        );
        assert_eq!(
            baseboard_information.product().unwrap(),
            "Surface Laptop 3".to_string()
        );
        assert_eq!(baseboard_information.version().is_none(), true);
        assert_eq!(
            baseboard_information.serial_number().unwrap(),
            "B009250100J1939B".to_string()
        );
        assert_eq!(baseboard_information.asset_tag().is_none(), true);
        assert_eq!(
            baseboard_information.feature_flags().unwrap(),
            BaseboardFeatures::from(1)
        );
        assert_eq!(baseboard_information.location_in_chassis().is_none(), true);
        assert_eq!(*baseboard_information.chassis_handle().unwrap(), 0x0F);
        assert_eq!(
            *baseboard_information.board_type().unwrap(),
            BoardType::Motherboard
        );

        // contained object handle tests
        assert_eq!(
            baseboard_information
                .number_of_contained_object_handles()
                .unwrap(),
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
