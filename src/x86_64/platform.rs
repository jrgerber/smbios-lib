use crate::{SMBiosVersion, SMBiosData, SMBiosEntryPoint64, SMBiosEntryPoint32, SMBiosEntryPoint64Error, SMBiosEntryPoint32Error};
use x86_64::{VirtAddr, PhysAddr};
use core::{slice, fmt};
use alloc::format;

const BLOCK_START: PhysAddr = PhysAddr::new_truncate(0xF0000);
const BLOCK_SIZE: usize = 0xFFFFF - 0xF0000;

/// A trait for mapping/unmapping physical memory blocks
pub trait MemoryMapper {
    /// Map physical memory block to virtual memory space
    fn map_block(&mut self, addr: PhysAddr, size: usize) -> VirtAddr;

    /// Unmap physical memory block from virtual memory space
    #[allow(unused_variables)]
    fn unmap_block(&mut self, addr: PhysAddr, virt_addr: VirtAddr, size: usize) {}
}

/// SMBios table load errors
pub enum SMBiosLoadError {
    /// Entry Point Structure checksum verification failed
    EntryChecksumVerificationFailed,
    /// The Entry Point Length field specified a value which exceeded the bounds of the Entry Point Structure
    EntryPointLengthTooBig,
    /// Intermediate entry point structure checksum verification failed
    IntermediateChecksumVerificationFailed,
    /// Entry Point not found
    EntryPointNotFound,
}

impl fmt::Debug for SMBiosLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SMBiosLoadError")
            .field(&format!("{}", &self))
            .finish()
    }
}

impl fmt::Display for SMBiosLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            SMBiosLoadError::EntryChecksumVerificationFailed => "Entry Point Structure checksum verification failed",
            SMBiosLoadError::EntryPointLengthTooBig => "The Entry Point Length field specified a value which exceeded the bounds of the Entry Point Structure",
            SMBiosLoadError::IntermediateChecksumVerificationFailed => "Intermediate entry point structure checksum verification failed",
            SMBiosLoadError::EntryPointNotFound => "Entry Point not found"
        };
        f.write_str(message)
    }
}


/// Loads [SMBiosData] by scanning memory from 0xF0000 to 0xFFFFF
pub fn table_load_from_device<T: MemoryMapper>(mapper: &mut T) -> Result<SMBiosData, SMBiosLoadError> {
    let start = mapper.map_block(BLOCK_START, BLOCK_SIZE);

    let slice: &[u8] = unsafe { slice::from_raw_parts(start.as_ptr(), BLOCK_SIZE) };
    let version: SMBiosVersion;
    let data_start: PhysAddr;
    let data_length: usize;

    match SMBiosEntryPoint64::try_scan_from_raw(slice) {
        Ok(entry_point) => {
            version = SMBiosVersion {
                major: entry_point.major_version(),
                minor: entry_point.minor_version(),
                revision: entry_point.docrev(),
            };
            data_start = PhysAddr::new(entry_point.structure_table_address());
            data_length = entry_point.structure_table_maximum_size() as usize;
        }
        Err(e) => match e {
            SMBiosEntryPoint64Error::EntryPointLengthTooBig => return Err(SMBiosLoadError::EntryPointLengthTooBig),
            SMBiosEntryPoint64Error::ChecksumVerificationFailed => return Err(SMBiosLoadError::EntryChecksumVerificationFailed),
            _ => match SMBiosEntryPoint32::try_scan_from_raw(slice) {
                Ok(entry_point) => {
                    version = SMBiosVersion {
                        major: entry_point.major_version(),
                        minor: entry_point.minor_version(),
                        revision: 0,
                    };
                    data_start = PhysAddr::new(entry_point.structure_table_address() as u64);
                    data_length = entry_point.structure_table_length() as usize;
                },
                Err(e) => return Err(match e {
                    SMBiosEntryPoint32Error::EntryChecksumVerificationFailed => SMBiosLoadError::EntryChecksumVerificationFailed,
                    SMBiosEntryPoint32Error::IntermediateChecksumVerificationFailed => SMBiosLoadError::IntermediateChecksumVerificationFailed,
                    SMBiosEntryPoint32Error::EntryPointLengthTooBig => SMBiosLoadError::EntryPointLengthTooBig,
                    _ => SMBiosLoadError::EntryPointNotFound
                })
            }
        },
    }
    mapper.unmap_block(BLOCK_START, start, BLOCK_SIZE);

    let data_start_virt = mapper.map_block(data_start, data_length);
    let data_slice: &[u8] = unsafe { slice::from_raw_parts(data_start_virt.as_ptr(), data_length) };
    let data = SMBiosData::from_vec_and_version(data_slice.to_vec(), Some(version));
    mapper.unmap_block(data_start, data_start_virt, data_length);
    Ok(data)
}