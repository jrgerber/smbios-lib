use crate::fields::*;
use std::fmt;

pub mod base;
pub use base::*;

mod unknown;
pub use unknown::*;

mod bios_information;
pub use bios_information::*;

mod additional_information;
pub use additional_information::*;

mod baseboard_information;
pub use baseboard_information::*;

mod bios_language_information;
pub use bios_language_information::*;

mod bis_entry_point;
pub use bis_entry_point::*;

mod built_in_pointing_device;
pub use built_in_pointing_device::*;

mod cache_information;
pub use cache_information::*;

mod cooling_device;
pub use cooling_device::*;

mod electrical_current_probe;
pub use electrical_current_probe::*;

mod end_of_table;
pub use end_of_table::*;

mod group_associations;
pub use group_associations::*;

mod hardware_security;
pub use hardware_security::*;

mod inactive;
pub use inactive::*;

mod ipmi_device_information;
pub use ipmi_device_information::*;

mod management_controller_host_interface;
pub use management_controller_host_interface::*;

mod management_device;
pub use management_device::*;

mod management_device_component;
pub use management_device_component::*;

mod management_device_threshold_data;
pub use management_device_threshold_data::*;

mod memory_array_mapped_address;
pub use memory_array_mapped_address::*;

mod memory_channel;
pub use memory_channel::*;

mod memory_controller_information;
pub use memory_controller_information::*;

mod memory_device;
pub use memory_device::*;

mod memory_device_mapped_address;
pub use memory_device_mapped_address::*;

mod memory_error_information_32;
pub use memory_error_information_32::*;

mod memory_error_information_64;
pub use memory_error_information_64::*;

mod memory_module_information;
pub use memory_module_information::*;

mod oem_strings;
pub use oem_strings::*;

mod on_board_device_information;
pub use on_board_device_information::*;

mod onboard_devices_extended_information;
pub use onboard_devices_extended_information::*;

mod out_of_band_remote_access;
pub use out_of_band_remote_access::*;

mod physical_memory_array;
pub use physical_memory_array::*;

mod portable_battery;
pub use portable_battery::*;

mod port_connector_information;
pub use port_connector_information::*;

mod processor_additional_information;
pub use processor_additional_information::*;

mod processor_information;
pub use processor_information::*;

mod system_boot_information;
pub use system_boot_information::*;

mod system_chassis_information;
pub use system_chassis_information::*;

mod system_configuration_options;
pub use system_configuration_options::*;

mod system_event_log;
pub use system_event_log::*;

mod system_information;
pub use system_information::*;

mod system_power_controls;
pub use system_power_controls::*;

mod system_power_supply;
pub use system_power_supply::*;

mod system_reset;
pub use system_reset::*;

mod system_slot;
pub use system_slot::*;

mod temperature_probe;
pub use temperature_probe::*;

mod tpm_device;
pub use tpm_device::*;

mod voltage_probe;
pub use voltage_probe::*;
