use super::*;

pub struct SMBiosManagementDeviceComponent<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosManagementDeviceComponent<'a> {
    const STRUCT_TYPE: u8 = 35u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosManagementDeviceComponent<'a> {
    fn description(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    fn management_device_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0x05)
    }

    fn component_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0x07)
    }

    fn threshold_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0x09)
    }
}

impl fmt::Debug for SMBiosManagementDeviceComponent<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosManagementDeviceComponent>())
        .field("header", &self.parts.header)
        .field("description", &self.description())
        .field("management_device_handle", &self.management_device_handle())
        .field("component_handle", &self.component_handle())
        .field("threshold_handle", &self.threshold_handle())
        .finish()
    }
}

