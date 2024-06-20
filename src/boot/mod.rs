// #[repr(C)]
// pub struct Multiboot2Header {
//     /// Magic number that a Multiboot-2 compliant bootloader looks for to identify the header
//     /// Must be set to 0xE85250D6
//     magic: u32,
//     /// Architecture of the target machine??
//     architecture: u32,
//     /// Length of the header in bytes
//     header_length: u32,
//     /// -(magic + architecture + header_length) should be 0
//     checksum: u32,
//     /// Specifies the end of the header
//     end_tag: MultibootEndTag,
// }

// #[repr(C)]
// struct MultibootEndTag {
//     tag_type: u16,
//     flags: u16,
//     size: u32,
// }

// #[no_mangle]
// #[link_section = ".multiboot"]
// pub static MULTIBOOT2_HEADER: Multiboot2Header = {
//     Multiboot2Header {
//         magic: 0xE85250D6,
//         architecture: 0,
//         header_length: core::mem::size_of::<Multiboot2Header>() as u32,
//         checksum: -(0xE85250D6 + 0 + core::mem::size_of::<Multiboot2Header>() as isize) as u32,
//         end_tag: MultibootEndTag {
//             tag_type: 0,
//             flags: 0,
//             size: 8,
//         },
//     }
// };
