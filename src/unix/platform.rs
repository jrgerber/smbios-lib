
// Example of Linux structure:
/*
    /sys/firmware/dmi/tables$ sudo hexdump -C smbios_entry_point 
    00000000  5f 53 4d 33 5f 7e 18 03  03 00 01 00 83 04 00 00  |_SM3_~..........|
    00000010  00 20 b0 7b 00 00 00 00                           |. .{....|
    00000018

    Note: _SM3_ indicates the version of the entry point.  Offsets 0x7-0x9 are
    the BIOS version 0x03, 0x03, 0x00 (3.0.0)

    jeff@blacktop:/sys/firmware/dmi/tables$ sudo hexdump -C DMI
    00000000  00 1a 00 00 01 02 00 00  03 ff 80 18 19 0c 00 00  |................|
    00000010  00 00 03 0d ff ff ff ff  00 00 4d 69 63 72 6f 73  |..........Micros|
    00000020  6f 66 74 20 43 6f 72 70  6f 72 61 74 69 6f 6e 00  |oft Corporation.|
    00000030  39 2e 31 30 32 2e 31 34  30 00 31 31 2f 31 36 2f  |9.102.140.11/16/|
    00000040  32 30 32 30 00 00 01 1b  01 00 01 02 03 04 86 76  |2020...........v|
    00000050  fb 97 d5 7b 15 d0 b2 39  6b ba a4 df c0 45 02 05  |...{...9k....E..|
    00000060  06 4d 69 63 72 6f 73 6f  66 74 20 43 6f 72 70 6f  |.Microsoft Corpo|
    00000070  72 61 74 69 6f 6e 00 53  75 72 66 61 63 65 20 4c  |ration.Surface L|
    00000080  61 70 74 6f 70 20 33 00  31 32 34 49 3a 30 30 30  |aptop 3.124I:000|
    00000090  33 36 54 3a 30 30 30 4d  3a 30 33 30 30 30 30 30  |36T:000M:0300000|
    000000a0  44 3a 30 42 3a 30 37 46  3a 31 43 3a 30 35 50 3a  |D:0B:07F:1C:05P:|
    000000b0  34 38 53 3a 30 31 45 3a  30 59 3a 31 4b 3a 30 55  |48S:01E:0Y:1K:0U|
    000000c0  3a 30 38 00 30 30 31 39  35 33 33 30 32 30 35 37  |:08.001953302057|
    000000d0  00 53 75 72 66 61 63 65  5f 4c 61 70 74 6f 70 5f  |.Surface_Laptop_|
    000000e0  33 5f 31 38 37 32 00 53  75 72 66 61 63 65 00 00  |3_1872.Surface..|
*/

// Note: /sys/class/dmi/id contains some of the BIOS values, already parsed by the kernel.
// These are useful for cross checking against the results this library produces when reading 
// /sys/firmware/dmi/tables/DMI

/// Temporary placeholder for Unix functions
pub fn hello_world() {    
    println!("Hello world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        hello_world();
    }
}