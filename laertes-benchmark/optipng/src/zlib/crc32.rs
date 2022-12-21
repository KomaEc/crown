
pub type ptrdiff_t = std::os::raw::c_long;
pub type size_t = std::os::raw::c_ulong;
pub type z_size_t = size_t;
pub type Byte = std::os::raw::c_uchar;
pub type uInt = std::os::raw::c_uint;
pub type uLong = std::os::raw::c_ulong;
pub type Bytef = Byte;
pub type z_crc_t = std::os::raw::c_uint;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type off_t = __off_t;
pub type off64_t = __off64_t;
static mut crc_table: [[z_crc_t; 256]; 8] =
    [[0 as std::os::raw::c_ulong as z_crc_t, 0x77073096 as std::os::raw::c_ulong as z_crc_t,
      0xee0e612c as std::os::raw::c_ulong as z_crc_t,
      0x990951ba as std::os::raw::c_ulong as z_crc_t,
      0x76dc419 as std::os::raw::c_ulong as z_crc_t,
      0x706af48f as std::os::raw::c_ulong as z_crc_t,
      0xe963a535 as std::os::raw::c_ulong as z_crc_t,
      0x9e6495a3 as std::os::raw::c_ulong as z_crc_t,
      0xedb8832 as std::os::raw::c_ulong as z_crc_t,
      0x79dcb8a4 as std::os::raw::c_ulong as z_crc_t,
      0xe0d5e91e as std::os::raw::c_ulong as z_crc_t,
      0x97d2d988 as std::os::raw::c_ulong as z_crc_t,
      0x9b64c2b as std::os::raw::c_ulong as z_crc_t,
      0x7eb17cbd as std::os::raw::c_ulong as z_crc_t,
      0xe7b82d07 as std::os::raw::c_ulong as z_crc_t,
      0x90bf1d91 as std::os::raw::c_ulong as z_crc_t,
      0x1db71064 as std::os::raw::c_ulong as z_crc_t,
      0x6ab020f2 as std::os::raw::c_ulong as z_crc_t,
      0xf3b97148 as std::os::raw::c_ulong as z_crc_t,
      0x84be41de as std::os::raw::c_ulong as z_crc_t,
      0x1adad47d as std::os::raw::c_ulong as z_crc_t,
      0x6ddde4eb as std::os::raw::c_ulong as z_crc_t,
      0xf4d4b551 as std::os::raw::c_ulong as z_crc_t,
      0x83d385c7 as std::os::raw::c_ulong as z_crc_t,
      0x136c9856 as std::os::raw::c_ulong as z_crc_t,
      0x646ba8c0 as std::os::raw::c_ulong as z_crc_t,
      0xfd62f97a as std::os::raw::c_ulong as z_crc_t,
      0x8a65c9ec as std::os::raw::c_ulong as z_crc_t,
      0x14015c4f as std::os::raw::c_ulong as z_crc_t,
      0x63066cd9 as std::os::raw::c_ulong as z_crc_t,
      0xfa0f3d63 as std::os::raw::c_ulong as z_crc_t,
      0x8d080df5 as std::os::raw::c_ulong as z_crc_t,
      0x3b6e20c8 as std::os::raw::c_ulong as z_crc_t,
      0x4c69105e as std::os::raw::c_ulong as z_crc_t,
      0xd56041e4 as std::os::raw::c_ulong as z_crc_t,
      0xa2677172 as std::os::raw::c_ulong as z_crc_t,
      0x3c03e4d1 as std::os::raw::c_ulong as z_crc_t,
      0x4b04d447 as std::os::raw::c_ulong as z_crc_t,
      0xd20d85fd as std::os::raw::c_ulong as z_crc_t,
      0xa50ab56b as std::os::raw::c_ulong as z_crc_t,
      0x35b5a8fa as std::os::raw::c_ulong as z_crc_t,
      0x42b2986c as std::os::raw::c_ulong as z_crc_t,
      0xdbbbc9d6 as std::os::raw::c_ulong as z_crc_t,
      0xacbcf940 as std::os::raw::c_ulong as z_crc_t,
      0x32d86ce3 as std::os::raw::c_ulong as z_crc_t,
      0x45df5c75 as std::os::raw::c_ulong as z_crc_t,
      0xdcd60dcf as std::os::raw::c_ulong as z_crc_t,
      0xabd13d59 as std::os::raw::c_ulong as z_crc_t,
      0x26d930ac as std::os::raw::c_ulong as z_crc_t,
      0x51de003a as std::os::raw::c_ulong as z_crc_t,
      0xc8d75180 as std::os::raw::c_ulong as z_crc_t,
      0xbfd06116 as std::os::raw::c_ulong as z_crc_t,
      0x21b4f4b5 as std::os::raw::c_ulong as z_crc_t,
      0x56b3c423 as std::os::raw::c_ulong as z_crc_t,
      0xcfba9599 as std::os::raw::c_ulong as z_crc_t,
      0xb8bda50f as std::os::raw::c_ulong as z_crc_t,
      0x2802b89e as std::os::raw::c_ulong as z_crc_t,
      0x5f058808 as std::os::raw::c_ulong as z_crc_t,
      0xc60cd9b2 as std::os::raw::c_ulong as z_crc_t,
      0xb10be924 as std::os::raw::c_ulong as z_crc_t,
      0x2f6f7c87 as std::os::raw::c_ulong as z_crc_t,
      0x58684c11 as std::os::raw::c_ulong as z_crc_t,
      0xc1611dab as std::os::raw::c_ulong as z_crc_t,
      0xb6662d3d as std::os::raw::c_ulong as z_crc_t,
      0x76dc4190 as std::os::raw::c_ulong as z_crc_t,
      0x1db7106 as std::os::raw::c_ulong as z_crc_t,
      0x98d220bc as std::os::raw::c_ulong as z_crc_t,
      0xefd5102a as std::os::raw::c_ulong as z_crc_t,
      0x71b18589 as std::os::raw::c_ulong as z_crc_t,
      0x6b6b51f as std::os::raw::c_ulong as z_crc_t,
      0x9fbfe4a5 as std::os::raw::c_ulong as z_crc_t,
      0xe8b8d433 as std::os::raw::c_ulong as z_crc_t,
      0x7807c9a2 as std::os::raw::c_ulong as z_crc_t,
      0xf00f934 as std::os::raw::c_ulong as z_crc_t,
      0x9609a88e as std::os::raw::c_ulong as z_crc_t,
      0xe10e9818 as std::os::raw::c_ulong as z_crc_t,
      0x7f6a0dbb as std::os::raw::c_ulong as z_crc_t,
      0x86d3d2d as std::os::raw::c_ulong as z_crc_t,
      0x91646c97 as std::os::raw::c_ulong as z_crc_t,
      0xe6635c01 as std::os::raw::c_ulong as z_crc_t,
      0x6b6b51f4 as std::os::raw::c_ulong as z_crc_t,
      0x1c6c6162 as std::os::raw::c_ulong as z_crc_t,
      0x856530d8 as std::os::raw::c_ulong as z_crc_t,
      0xf262004e as std::os::raw::c_ulong as z_crc_t,
      0x6c0695ed as std::os::raw::c_ulong as z_crc_t,
      0x1b01a57b as std::os::raw::c_ulong as z_crc_t,
      0x8208f4c1 as std::os::raw::c_ulong as z_crc_t,
      0xf50fc457 as std::os::raw::c_ulong as z_crc_t,
      0x65b0d9c6 as std::os::raw::c_ulong as z_crc_t,
      0x12b7e950 as std::os::raw::c_ulong as z_crc_t,
      0x8bbeb8ea as std::os::raw::c_ulong as z_crc_t,
      0xfcb9887c as std::os::raw::c_ulong as z_crc_t,
      0x62dd1ddf as std::os::raw::c_ulong as z_crc_t,
      0x15da2d49 as std::os::raw::c_ulong as z_crc_t,
      0x8cd37cf3 as std::os::raw::c_ulong as z_crc_t,
      0xfbd44c65 as std::os::raw::c_ulong as z_crc_t,
      0x4db26158 as std::os::raw::c_ulong as z_crc_t,
      0x3ab551ce as std::os::raw::c_ulong as z_crc_t,
      0xa3bc0074 as std::os::raw::c_ulong as z_crc_t,
      0xd4bb30e2 as std::os::raw::c_ulong as z_crc_t,
      0x4adfa541 as std::os::raw::c_ulong as z_crc_t,
      0x3dd895d7 as std::os::raw::c_ulong as z_crc_t,
      0xa4d1c46d as std::os::raw::c_ulong as z_crc_t,
      0xd3d6f4fb as std::os::raw::c_ulong as z_crc_t,
      0x4369e96a as std::os::raw::c_ulong as z_crc_t,
      0x346ed9fc as std::os::raw::c_ulong as z_crc_t,
      0xad678846 as std::os::raw::c_ulong as z_crc_t,
      0xda60b8d0 as std::os::raw::c_ulong as z_crc_t,
      0x44042d73 as std::os::raw::c_ulong as z_crc_t,
      0x33031de5 as std::os::raw::c_ulong as z_crc_t,
      0xaa0a4c5f as std::os::raw::c_ulong as z_crc_t,
      0xdd0d7cc9 as std::os::raw::c_ulong as z_crc_t,
      0x5005713c as std::os::raw::c_ulong as z_crc_t,
      0x270241aa as std::os::raw::c_ulong as z_crc_t,
      0xbe0b1010 as std::os::raw::c_ulong as z_crc_t,
      0xc90c2086 as std::os::raw::c_ulong as z_crc_t,
      0x5768b525 as std::os::raw::c_ulong as z_crc_t,
      0x206f85b3 as std::os::raw::c_ulong as z_crc_t,
      0xb966d409 as std::os::raw::c_ulong as z_crc_t,
      0xce61e49f as std::os::raw::c_ulong as z_crc_t,
      0x5edef90e as std::os::raw::c_ulong as z_crc_t,
      0x29d9c998 as std::os::raw::c_ulong as z_crc_t,
      0xb0d09822 as std::os::raw::c_ulong as z_crc_t,
      0xc7d7a8b4 as std::os::raw::c_ulong as z_crc_t,
      0x59b33d17 as std::os::raw::c_ulong as z_crc_t,
      0x2eb40d81 as std::os::raw::c_ulong as z_crc_t,
      0xb7bd5c3b as std::os::raw::c_ulong as z_crc_t,
      0xc0ba6cad as std::os::raw::c_ulong as z_crc_t,
      0xedb88320 as std::os::raw::c_ulong as z_crc_t,
      0x9abfb3b6 as std::os::raw::c_ulong as z_crc_t,
      0x3b6e20c as std::os::raw::c_ulong as z_crc_t,
      0x74b1d29a as std::os::raw::c_ulong as z_crc_t,
      0xead54739 as std::os::raw::c_ulong as z_crc_t,
      0x9dd277af as std::os::raw::c_ulong as z_crc_t,
      0x4db2615 as std::os::raw::c_ulong as z_crc_t,
      0x73dc1683 as std::os::raw::c_ulong as z_crc_t,
      0xe3630b12 as std::os::raw::c_ulong as z_crc_t,
      0x94643b84 as std::os::raw::c_ulong as z_crc_t,
      0xd6d6a3e as std::os::raw::c_ulong as z_crc_t,
      0x7a6a5aa8 as std::os::raw::c_ulong as z_crc_t,
      0xe40ecf0b as std::os::raw::c_ulong as z_crc_t,
      0x9309ff9d as std::os::raw::c_ulong as z_crc_t,
      0xa00ae27 as std::os::raw::c_ulong as z_crc_t,
      0x7d079eb1 as std::os::raw::c_ulong as z_crc_t,
      0xf00f9344 as std::os::raw::c_ulong as z_crc_t,
      0x8708a3d2 as std::os::raw::c_ulong as z_crc_t,
      0x1e01f268 as std::os::raw::c_ulong as z_crc_t,
      0x6906c2fe as std::os::raw::c_ulong as z_crc_t,
      0xf762575d as std::os::raw::c_ulong as z_crc_t,
      0x806567cb as std::os::raw::c_ulong as z_crc_t,
      0x196c3671 as std::os::raw::c_ulong as z_crc_t,
      0x6e6b06e7 as std::os::raw::c_ulong as z_crc_t,
      0xfed41b76 as std::os::raw::c_ulong as z_crc_t,
      0x89d32be0 as std::os::raw::c_ulong as z_crc_t,
      0x10da7a5a as std::os::raw::c_ulong as z_crc_t,
      0x67dd4acc as std::os::raw::c_ulong as z_crc_t,
      0xf9b9df6f as std::os::raw::c_ulong as z_crc_t,
      0x8ebeeff9 as std::os::raw::c_ulong as z_crc_t,
      0x17b7be43 as std::os::raw::c_ulong as z_crc_t,
      0x60b08ed5 as std::os::raw::c_ulong as z_crc_t,
      0xd6d6a3e8 as std::os::raw::c_ulong as z_crc_t,
      0xa1d1937e as std::os::raw::c_ulong as z_crc_t,
      0x38d8c2c4 as std::os::raw::c_ulong as z_crc_t,
      0x4fdff252 as std::os::raw::c_ulong as z_crc_t,
      0xd1bb67f1 as std::os::raw::c_ulong as z_crc_t,
      0xa6bc5767 as std::os::raw::c_ulong as z_crc_t,
      0x3fb506dd as std::os::raw::c_ulong as z_crc_t,
      0x48b2364b as std::os::raw::c_ulong as z_crc_t,
      0xd80d2bda as std::os::raw::c_ulong as z_crc_t,
      0xaf0a1b4c as std::os::raw::c_ulong as z_crc_t,
      0x36034af6 as std::os::raw::c_ulong as z_crc_t,
      0x41047a60 as std::os::raw::c_ulong as z_crc_t,
      0xdf60efc3 as std::os::raw::c_ulong as z_crc_t,
      0xa867df55 as std::os::raw::c_ulong as z_crc_t,
      0x316e8eef as std::os::raw::c_ulong as z_crc_t,
      0x4669be79 as std::os::raw::c_ulong as z_crc_t,
      0xcb61b38c as std::os::raw::c_ulong as z_crc_t,
      0xbc66831a as std::os::raw::c_ulong as z_crc_t,
      0x256fd2a0 as std::os::raw::c_ulong as z_crc_t,
      0x5268e236 as std::os::raw::c_ulong as z_crc_t,
      0xcc0c7795 as std::os::raw::c_ulong as z_crc_t,
      0xbb0b4703 as std::os::raw::c_ulong as z_crc_t,
      0x220216b9 as std::os::raw::c_ulong as z_crc_t,
      0x5505262f as std::os::raw::c_ulong as z_crc_t,
      0xc5ba3bbe as std::os::raw::c_ulong as z_crc_t,
      0xb2bd0b28 as std::os::raw::c_ulong as z_crc_t,
      0x2bb45a92 as std::os::raw::c_ulong as z_crc_t,
      0x5cb36a04 as std::os::raw::c_ulong as z_crc_t,
      0xc2d7ffa7 as std::os::raw::c_ulong as z_crc_t,
      0xb5d0cf31 as std::os::raw::c_ulong as z_crc_t,
      0x2cd99e8b as std::os::raw::c_ulong as z_crc_t,
      0x5bdeae1d as std::os::raw::c_ulong as z_crc_t,
      0x9b64c2b0 as std::os::raw::c_ulong as z_crc_t,
      0xec63f226 as std::os::raw::c_ulong as z_crc_t,
      0x756aa39c as std::os::raw::c_ulong as z_crc_t,
      0x26d930a as std::os::raw::c_ulong as z_crc_t,
      0x9c0906a9 as std::os::raw::c_ulong as z_crc_t,
      0xeb0e363f as std::os::raw::c_ulong as z_crc_t,
      0x72076785 as std::os::raw::c_ulong as z_crc_t,
      0x5005713 as std::os::raw::c_ulong as z_crc_t,
      0x95bf4a82 as std::os::raw::c_ulong as z_crc_t,
      0xe2b87a14 as std::os::raw::c_ulong as z_crc_t,
      0x7bb12bae as std::os::raw::c_ulong as z_crc_t,
      0xcb61b38 as std::os::raw::c_ulong as z_crc_t,
      0x92d28e9b as std::os::raw::c_ulong as z_crc_t,
      0xe5d5be0d as std::os::raw::c_ulong as z_crc_t,
      0x7cdcefb7 as std::os::raw::c_ulong as z_crc_t,
      0xbdbdf21 as std::os::raw::c_ulong as z_crc_t,
      0x86d3d2d4 as std::os::raw::c_ulong as z_crc_t,
      0xf1d4e242 as std::os::raw::c_ulong as z_crc_t,
      0x68ddb3f8 as std::os::raw::c_ulong as z_crc_t,
      0x1fda836e as std::os::raw::c_ulong as z_crc_t,
      0x81be16cd as std::os::raw::c_ulong as z_crc_t,
      0xf6b9265b as std::os::raw::c_ulong as z_crc_t,
      0x6fb077e1 as std::os::raw::c_ulong as z_crc_t,
      0x18b74777 as std::os::raw::c_ulong as z_crc_t,
      0x88085ae6 as std::os::raw::c_ulong as z_crc_t,
      0xff0f6a70 as std::os::raw::c_ulong as z_crc_t,
      0x66063bca as std::os::raw::c_ulong as z_crc_t,
      0x11010b5c as std::os::raw::c_ulong as z_crc_t,
      0x8f659eff as std::os::raw::c_ulong as z_crc_t,
      0xf862ae69 as std::os::raw::c_ulong as z_crc_t,
      0x616bffd3 as std::os::raw::c_ulong as z_crc_t,
      0x166ccf45 as std::os::raw::c_ulong as z_crc_t,
      0xa00ae278 as std::os::raw::c_ulong as z_crc_t,
      0xd70dd2ee as std::os::raw::c_ulong as z_crc_t,
      0x4e048354 as std::os::raw::c_ulong as z_crc_t,
      0x3903b3c2 as std::os::raw::c_ulong as z_crc_t,
      0xa7672661 as std::os::raw::c_ulong as z_crc_t,
      0xd06016f7 as std::os::raw::c_ulong as z_crc_t,
      0x4969474d as std::os::raw::c_ulong as z_crc_t,
      0x3e6e77db as std::os::raw::c_ulong as z_crc_t,
      0xaed16a4a as std::os::raw::c_ulong as z_crc_t,
      0xd9d65adc as std::os::raw::c_ulong as z_crc_t,
      0x40df0b66 as std::os::raw::c_ulong as z_crc_t,
      0x37d83bf0 as std::os::raw::c_ulong as z_crc_t,
      0xa9bcae53 as std::os::raw::c_ulong as z_crc_t,
      0xdebb9ec5 as std::os::raw::c_ulong as z_crc_t,
      0x47b2cf7f as std::os::raw::c_ulong as z_crc_t,
      0x30b5ffe9 as std::os::raw::c_ulong as z_crc_t,
      0xbdbdf21c as std::os::raw::c_ulong as z_crc_t,
      0xcabac28a as std::os::raw::c_ulong as z_crc_t,
      0x53b39330 as std::os::raw::c_ulong as z_crc_t,
      0x24b4a3a6 as std::os::raw::c_ulong as z_crc_t,
      0xbad03605 as std::os::raw::c_ulong as z_crc_t,
      0xcdd70693 as std::os::raw::c_ulong as z_crc_t,
      0x54de5729 as std::os::raw::c_ulong as z_crc_t,
      0x23d967bf as std::os::raw::c_ulong as z_crc_t,
      0xb3667a2e as std::os::raw::c_ulong as z_crc_t,
      0xc4614ab8 as std::os::raw::c_ulong as z_crc_t,
      0x5d681b02 as std::os::raw::c_ulong as z_crc_t,
      0x2a6f2b94 as std::os::raw::c_ulong as z_crc_t,
      0xb40bbe37 as std::os::raw::c_ulong as z_crc_t,
      0xc30c8ea1 as std::os::raw::c_ulong as z_crc_t,
      0x5a05df1b as std::os::raw::c_ulong as z_crc_t,
      0x2d02ef8d as std::os::raw::c_ulong as z_crc_t],
     [0 as std::os::raw::c_ulong as z_crc_t, 0x191b3141 as std::os::raw::c_ulong as z_crc_t,
      0x32366282 as std::os::raw::c_ulong as z_crc_t,
      0x2b2d53c3 as std::os::raw::c_ulong as z_crc_t,
      0x646cc504 as std::os::raw::c_ulong as z_crc_t,
      0x7d77f445 as std::os::raw::c_ulong as z_crc_t,
      0x565aa786 as std::os::raw::c_ulong as z_crc_t,
      0x4f4196c7 as std::os::raw::c_ulong as z_crc_t,
      0xc8d98a08 as std::os::raw::c_ulong as z_crc_t,
      0xd1c2bb49 as std::os::raw::c_ulong as z_crc_t,
      0xfaefe88a as std::os::raw::c_ulong as z_crc_t,
      0xe3f4d9cb as std::os::raw::c_ulong as z_crc_t,
      0xacb54f0c as std::os::raw::c_ulong as z_crc_t,
      0xb5ae7e4d as std::os::raw::c_ulong as z_crc_t,
      0x9e832d8e as std::os::raw::c_ulong as z_crc_t,
      0x87981ccf as std::os::raw::c_ulong as z_crc_t,
      0x4ac21251 as std::os::raw::c_ulong as z_crc_t,
      0x53d92310 as std::os::raw::c_ulong as z_crc_t,
      0x78f470d3 as std::os::raw::c_ulong as z_crc_t,
      0x61ef4192 as std::os::raw::c_ulong as z_crc_t,
      0x2eaed755 as std::os::raw::c_ulong as z_crc_t,
      0x37b5e614 as std::os::raw::c_ulong as z_crc_t,
      0x1c98b5d7 as std::os::raw::c_ulong as z_crc_t,
      0x5838496 as std::os::raw::c_ulong as z_crc_t,
      0x821b9859 as std::os::raw::c_ulong as z_crc_t,
      0x9b00a918 as std::os::raw::c_ulong as z_crc_t,
      0xb02dfadb as std::os::raw::c_ulong as z_crc_t,
      0xa936cb9a as std::os::raw::c_ulong as z_crc_t,
      0xe6775d5d as std::os::raw::c_ulong as z_crc_t,
      0xff6c6c1c as std::os::raw::c_ulong as z_crc_t,
      0xd4413fdf as std::os::raw::c_ulong as z_crc_t,
      0xcd5a0e9e as std::os::raw::c_ulong as z_crc_t,
      0x958424a2 as std::os::raw::c_ulong as z_crc_t,
      0x8c9f15e3 as std::os::raw::c_ulong as z_crc_t,
      0xa7b24620 as std::os::raw::c_ulong as z_crc_t,
      0xbea97761 as std::os::raw::c_ulong as z_crc_t,
      0xf1e8e1a6 as std::os::raw::c_ulong as z_crc_t,
      0xe8f3d0e7 as std::os::raw::c_ulong as z_crc_t,
      0xc3de8324 as std::os::raw::c_ulong as z_crc_t,
      0xdac5b265 as std::os::raw::c_ulong as z_crc_t,
      0x5d5daeaa as std::os::raw::c_ulong as z_crc_t,
      0x44469feb as std::os::raw::c_ulong as z_crc_t,
      0x6f6bcc28 as std::os::raw::c_ulong as z_crc_t,
      0x7670fd69 as std::os::raw::c_ulong as z_crc_t,
      0x39316bae as std::os::raw::c_ulong as z_crc_t,
      0x202a5aef as std::os::raw::c_ulong as z_crc_t,
      0xb07092c as std::os::raw::c_ulong as z_crc_t,
      0x121c386d as std::os::raw::c_ulong as z_crc_t,
      0xdf4636f3 as std::os::raw::c_ulong as z_crc_t,
      0xc65d07b2 as std::os::raw::c_ulong as z_crc_t,
      0xed705471 as std::os::raw::c_ulong as z_crc_t,
      0xf46b6530 as std::os::raw::c_ulong as z_crc_t,
      0xbb2af3f7 as std::os::raw::c_ulong as z_crc_t,
      0xa231c2b6 as std::os::raw::c_ulong as z_crc_t,
      0x891c9175 as std::os::raw::c_ulong as z_crc_t,
      0x9007a034 as std::os::raw::c_ulong as z_crc_t,
      0x179fbcfb as std::os::raw::c_ulong as z_crc_t,
      0xe848dba as std::os::raw::c_ulong as z_crc_t,
      0x25a9de79 as std::os::raw::c_ulong as z_crc_t,
      0x3cb2ef38 as std::os::raw::c_ulong as z_crc_t,
      0x73f379ff as std::os::raw::c_ulong as z_crc_t,
      0x6ae848be as std::os::raw::c_ulong as z_crc_t,
      0x41c51b7d as std::os::raw::c_ulong as z_crc_t,
      0x58de2a3c as std::os::raw::c_ulong as z_crc_t,
      0xf0794f05 as std::os::raw::c_ulong as z_crc_t,
      0xe9627e44 as std::os::raw::c_ulong as z_crc_t,
      0xc24f2d87 as std::os::raw::c_ulong as z_crc_t,
      0xdb541cc6 as std::os::raw::c_ulong as z_crc_t,
      0x94158a01 as std::os::raw::c_ulong as z_crc_t,
      0x8d0ebb40 as std::os::raw::c_ulong as z_crc_t,
      0xa623e883 as std::os::raw::c_ulong as z_crc_t,
      0xbf38d9c2 as std::os::raw::c_ulong as z_crc_t,
      0x38a0c50d as std::os::raw::c_ulong as z_crc_t,
      0x21bbf44c as std::os::raw::c_ulong as z_crc_t,
      0xa96a78f as std::os::raw::c_ulong as z_crc_t,
      0x138d96ce as std::os::raw::c_ulong as z_crc_t,
      0x5ccc0009 as std::os::raw::c_ulong as z_crc_t,
      0x45d73148 as std::os::raw::c_ulong as z_crc_t,
      0x6efa628b as std::os::raw::c_ulong as z_crc_t,
      0x77e153ca as std::os::raw::c_ulong as z_crc_t,
      0xbabb5d54 as std::os::raw::c_ulong as z_crc_t,
      0xa3a06c15 as std::os::raw::c_ulong as z_crc_t,
      0x888d3fd6 as std::os::raw::c_ulong as z_crc_t,
      0x91960e97 as std::os::raw::c_ulong as z_crc_t,
      0xded79850 as std::os::raw::c_ulong as z_crc_t,
      0xc7cca911 as std::os::raw::c_ulong as z_crc_t,
      0xece1fad2 as std::os::raw::c_ulong as z_crc_t,
      0xf5facb93 as std::os::raw::c_ulong as z_crc_t,
      0x7262d75c as std::os::raw::c_ulong as z_crc_t,
      0x6b79e61d as std::os::raw::c_ulong as z_crc_t,
      0x4054b5de as std::os::raw::c_ulong as z_crc_t,
      0x594f849f as std::os::raw::c_ulong as z_crc_t,
      0x160e1258 as std::os::raw::c_ulong as z_crc_t,
      0xf152319 as std::os::raw::c_ulong as z_crc_t,
      0x243870da as std::os::raw::c_ulong as z_crc_t,
      0x3d23419b as std::os::raw::c_ulong as z_crc_t,
      0x65fd6ba7 as std::os::raw::c_ulong as z_crc_t,
      0x7ce65ae6 as std::os::raw::c_ulong as z_crc_t,
      0x57cb0925 as std::os::raw::c_ulong as z_crc_t,
      0x4ed03864 as std::os::raw::c_ulong as z_crc_t,
      0x191aea3 as std::os::raw::c_ulong as z_crc_t,
      0x188a9fe2 as std::os::raw::c_ulong as z_crc_t,
      0x33a7cc21 as std::os::raw::c_ulong as z_crc_t,
      0x2abcfd60 as std::os::raw::c_ulong as z_crc_t,
      0xad24e1af as std::os::raw::c_ulong as z_crc_t,
      0xb43fd0ee as std::os::raw::c_ulong as z_crc_t,
      0x9f12832d as std::os::raw::c_ulong as z_crc_t,
      0x8609b26c as std::os::raw::c_ulong as z_crc_t,
      0xc94824ab as std::os::raw::c_ulong as z_crc_t,
      0xd05315ea as std::os::raw::c_ulong as z_crc_t,
      0xfb7e4629 as std::os::raw::c_ulong as z_crc_t,
      0xe2657768 as std::os::raw::c_ulong as z_crc_t,
      0x2f3f79f6 as std::os::raw::c_ulong as z_crc_t,
      0x362448b7 as std::os::raw::c_ulong as z_crc_t,
      0x1d091b74 as std::os::raw::c_ulong as z_crc_t,
      0x4122a35 as std::os::raw::c_ulong as z_crc_t,
      0x4b53bcf2 as std::os::raw::c_ulong as z_crc_t,
      0x52488db3 as std::os::raw::c_ulong as z_crc_t,
      0x7965de70 as std::os::raw::c_ulong as z_crc_t,
      0x607eef31 as std::os::raw::c_ulong as z_crc_t,
      0xe7e6f3fe as std::os::raw::c_ulong as z_crc_t,
      0xfefdc2bf as std::os::raw::c_ulong as z_crc_t,
      0xd5d0917c as std::os::raw::c_ulong as z_crc_t,
      0xcccba03d as std::os::raw::c_ulong as z_crc_t,
      0x838a36fa as std::os::raw::c_ulong as z_crc_t,
      0x9a9107bb as std::os::raw::c_ulong as z_crc_t,
      0xb1bc5478 as std::os::raw::c_ulong as z_crc_t,
      0xa8a76539 as std::os::raw::c_ulong as z_crc_t,
      0x3b83984b as std::os::raw::c_ulong as z_crc_t,
      0x2298a90a as std::os::raw::c_ulong as z_crc_t,
      0x9b5fac9 as std::os::raw::c_ulong as z_crc_t,
      0x10aecb88 as std::os::raw::c_ulong as z_crc_t,
      0x5fef5d4f as std::os::raw::c_ulong as z_crc_t,
      0x46f46c0e as std::os::raw::c_ulong as z_crc_t,
      0x6dd93fcd as std::os::raw::c_ulong as z_crc_t,
      0x74c20e8c as std::os::raw::c_ulong as z_crc_t,
      0xf35a1243 as std::os::raw::c_ulong as z_crc_t,
      0xea412302 as std::os::raw::c_ulong as z_crc_t,
      0xc16c70c1 as std::os::raw::c_ulong as z_crc_t,
      0xd8774180 as std::os::raw::c_ulong as z_crc_t,
      0x9736d747 as std::os::raw::c_ulong as z_crc_t,
      0x8e2de606 as std::os::raw::c_ulong as z_crc_t,
      0xa500b5c5 as std::os::raw::c_ulong as z_crc_t,
      0xbc1b8484 as std::os::raw::c_ulong as z_crc_t,
      0x71418a1a as std::os::raw::c_ulong as z_crc_t,
      0x685abb5b as std::os::raw::c_ulong as z_crc_t,
      0x4377e898 as std::os::raw::c_ulong as z_crc_t,
      0x5a6cd9d9 as std::os::raw::c_ulong as z_crc_t,
      0x152d4f1e as std::os::raw::c_ulong as z_crc_t,
      0xc367e5f as std::os::raw::c_ulong as z_crc_t,
      0x271b2d9c as std::os::raw::c_ulong as z_crc_t,
      0x3e001cdd as std::os::raw::c_ulong as z_crc_t,
      0xb9980012 as std::os::raw::c_ulong as z_crc_t,
      0xa0833153 as std::os::raw::c_ulong as z_crc_t,
      0x8bae6290 as std::os::raw::c_ulong as z_crc_t,
      0x92b553d1 as std::os::raw::c_ulong as z_crc_t,
      0xddf4c516 as std::os::raw::c_ulong as z_crc_t,
      0xc4eff457 as std::os::raw::c_ulong as z_crc_t,
      0xefc2a794 as std::os::raw::c_ulong as z_crc_t,
      0xf6d996d5 as std::os::raw::c_ulong as z_crc_t,
      0xae07bce9 as std::os::raw::c_ulong as z_crc_t,
      0xb71c8da8 as std::os::raw::c_ulong as z_crc_t,
      0x9c31de6b as std::os::raw::c_ulong as z_crc_t,
      0x852aef2a as std::os::raw::c_ulong as z_crc_t,
      0xca6b79ed as std::os::raw::c_ulong as z_crc_t,
      0xd37048ac as std::os::raw::c_ulong as z_crc_t,
      0xf85d1b6f as std::os::raw::c_ulong as z_crc_t,
      0xe1462a2e as std::os::raw::c_ulong as z_crc_t,
      0x66de36e1 as std::os::raw::c_ulong as z_crc_t,
      0x7fc507a0 as std::os::raw::c_ulong as z_crc_t,
      0x54e85463 as std::os::raw::c_ulong as z_crc_t,
      0x4df36522 as std::os::raw::c_ulong as z_crc_t,
      0x2b2f3e5 as std::os::raw::c_ulong as z_crc_t,
      0x1ba9c2a4 as std::os::raw::c_ulong as z_crc_t,
      0x30849167 as std::os::raw::c_ulong as z_crc_t,
      0x299fa026 as std::os::raw::c_ulong as z_crc_t,
      0xe4c5aeb8 as std::os::raw::c_ulong as z_crc_t,
      0xfdde9ff9 as std::os::raw::c_ulong as z_crc_t,
      0xd6f3cc3a as std::os::raw::c_ulong as z_crc_t,
      0xcfe8fd7b as std::os::raw::c_ulong as z_crc_t,
      0x80a96bbc as std::os::raw::c_ulong as z_crc_t,
      0x99b25afd as std::os::raw::c_ulong as z_crc_t,
      0xb29f093e as std::os::raw::c_ulong as z_crc_t,
      0xab84387f as std::os::raw::c_ulong as z_crc_t,
      0x2c1c24b0 as std::os::raw::c_ulong as z_crc_t,
      0x350715f1 as std::os::raw::c_ulong as z_crc_t,
      0x1e2a4632 as std::os::raw::c_ulong as z_crc_t,
      0x7317773 as std::os::raw::c_ulong as z_crc_t,
      0x4870e1b4 as std::os::raw::c_ulong as z_crc_t,
      0x516bd0f5 as std::os::raw::c_ulong as z_crc_t,
      0x7a468336 as std::os::raw::c_ulong as z_crc_t,
      0x635db277 as std::os::raw::c_ulong as z_crc_t,
      0xcbfad74e as std::os::raw::c_ulong as z_crc_t,
      0xd2e1e60f as std::os::raw::c_ulong as z_crc_t,
      0xf9ccb5cc as std::os::raw::c_ulong as z_crc_t,
      0xe0d7848d as std::os::raw::c_ulong as z_crc_t,
      0xaf96124a as std::os::raw::c_ulong as z_crc_t,
      0xb68d230b as std::os::raw::c_ulong as z_crc_t,
      0x9da070c8 as std::os::raw::c_ulong as z_crc_t,
      0x84bb4189 as std::os::raw::c_ulong as z_crc_t,
      0x3235d46 as std::os::raw::c_ulong as z_crc_t,
      0x1a386c07 as std::os::raw::c_ulong as z_crc_t,
      0x31153fc4 as std::os::raw::c_ulong as z_crc_t,
      0x280e0e85 as std::os::raw::c_ulong as z_crc_t,
      0x674f9842 as std::os::raw::c_ulong as z_crc_t,
      0x7e54a903 as std::os::raw::c_ulong as z_crc_t,
      0x5579fac0 as std::os::raw::c_ulong as z_crc_t,
      0x4c62cb81 as std::os::raw::c_ulong as z_crc_t,
      0x8138c51f as std::os::raw::c_ulong as z_crc_t,
      0x9823f45e as std::os::raw::c_ulong as z_crc_t,
      0xb30ea79d as std::os::raw::c_ulong as z_crc_t,
      0xaa1596dc as std::os::raw::c_ulong as z_crc_t,
      0xe554001b as std::os::raw::c_ulong as z_crc_t,
      0xfc4f315a as std::os::raw::c_ulong as z_crc_t,
      0xd7626299 as std::os::raw::c_ulong as z_crc_t,
      0xce7953d8 as std::os::raw::c_ulong as z_crc_t,
      0x49e14f17 as std::os::raw::c_ulong as z_crc_t,
      0x50fa7e56 as std::os::raw::c_ulong as z_crc_t,
      0x7bd72d95 as std::os::raw::c_ulong as z_crc_t,
      0x62cc1cd4 as std::os::raw::c_ulong as z_crc_t,
      0x2d8d8a13 as std::os::raw::c_ulong as z_crc_t,
      0x3496bb52 as std::os::raw::c_ulong as z_crc_t,
      0x1fbbe891 as std::os::raw::c_ulong as z_crc_t,
      0x6a0d9d0 as std::os::raw::c_ulong as z_crc_t,
      0x5e7ef3ec as std::os::raw::c_ulong as z_crc_t,
      0x4765c2ad as std::os::raw::c_ulong as z_crc_t,
      0x6c48916e as std::os::raw::c_ulong as z_crc_t,
      0x7553a02f as std::os::raw::c_ulong as z_crc_t,
      0x3a1236e8 as std::os::raw::c_ulong as z_crc_t,
      0x230907a9 as std::os::raw::c_ulong as z_crc_t,
      0x824546a as std::os::raw::c_ulong as z_crc_t,
      0x113f652b as std::os::raw::c_ulong as z_crc_t,
      0x96a779e4 as std::os::raw::c_ulong as z_crc_t,
      0x8fbc48a5 as std::os::raw::c_ulong as z_crc_t,
      0xa4911b66 as std::os::raw::c_ulong as z_crc_t,
      0xbd8a2a27 as std::os::raw::c_ulong as z_crc_t,
      0xf2cbbce0 as std::os::raw::c_ulong as z_crc_t,
      0xebd08da1 as std::os::raw::c_ulong as z_crc_t,
      0xc0fdde62 as std::os::raw::c_ulong as z_crc_t,
      0xd9e6ef23 as std::os::raw::c_ulong as z_crc_t,
      0x14bce1bd as std::os::raw::c_ulong as z_crc_t,
      0xda7d0fc as std::os::raw::c_ulong as z_crc_t,
      0x268a833f as std::os::raw::c_ulong as z_crc_t,
      0x3f91b27e as std::os::raw::c_ulong as z_crc_t,
      0x70d024b9 as std::os::raw::c_ulong as z_crc_t,
      0x69cb15f8 as std::os::raw::c_ulong as z_crc_t,
      0x42e6463b as std::os::raw::c_ulong as z_crc_t,
      0x5bfd777a as std::os::raw::c_ulong as z_crc_t,
      0xdc656bb5 as std::os::raw::c_ulong as z_crc_t,
      0xc57e5af4 as std::os::raw::c_ulong as z_crc_t,
      0xee530937 as std::os::raw::c_ulong as z_crc_t,
      0xf7483876 as std::os::raw::c_ulong as z_crc_t,
      0xb809aeb1 as std::os::raw::c_ulong as z_crc_t,
      0xa1129ff0 as std::os::raw::c_ulong as z_crc_t,
      0x8a3fcc33 as std::os::raw::c_ulong as z_crc_t,
      0x9324fd72 as std::os::raw::c_ulong as z_crc_t],
     [0 as std::os::raw::c_ulong as z_crc_t, 0x1c26a37 as std::os::raw::c_ulong as z_crc_t,
      0x384d46e as std::os::raw::c_ulong as z_crc_t,
      0x246be59 as std::os::raw::c_ulong as z_crc_t,
      0x709a8dc as std::os::raw::c_ulong as z_crc_t,
      0x6cbc2eb as std::os::raw::c_ulong as z_crc_t,
      0x48d7cb2 as std::os::raw::c_ulong as z_crc_t,
      0x54f1685 as std::os::raw::c_ulong as z_crc_t,
      0xe1351b8 as std::os::raw::c_ulong as z_crc_t,
      0xfd13b8f as std::os::raw::c_ulong as z_crc_t,
      0xd9785d6 as std::os::raw::c_ulong as z_crc_t,
      0xc55efe1 as std::os::raw::c_ulong as z_crc_t,
      0x91af964 as std::os::raw::c_ulong as z_crc_t,
      0x8d89353 as std::os::raw::c_ulong as z_crc_t,
      0xa9e2d0a as std::os::raw::c_ulong as z_crc_t,
      0xb5c473d as std::os::raw::c_ulong as z_crc_t,
      0x1c26a370 as std::os::raw::c_ulong as z_crc_t,
      0x1de4c947 as std::os::raw::c_ulong as z_crc_t,
      0x1fa2771e as std::os::raw::c_ulong as z_crc_t,
      0x1e601d29 as std::os::raw::c_ulong as z_crc_t,
      0x1b2f0bac as std::os::raw::c_ulong as z_crc_t,
      0x1aed619b as std::os::raw::c_ulong as z_crc_t,
      0x18abdfc2 as std::os::raw::c_ulong as z_crc_t,
      0x1969b5f5 as std::os::raw::c_ulong as z_crc_t,
      0x1235f2c8 as std::os::raw::c_ulong as z_crc_t,
      0x13f798ff as std::os::raw::c_ulong as z_crc_t,
      0x11b126a6 as std::os::raw::c_ulong as z_crc_t,
      0x10734c91 as std::os::raw::c_ulong as z_crc_t,
      0x153c5a14 as std::os::raw::c_ulong as z_crc_t,
      0x14fe3023 as std::os::raw::c_ulong as z_crc_t,
      0x16b88e7a as std::os::raw::c_ulong as z_crc_t,
      0x177ae44d as std::os::raw::c_ulong as z_crc_t,
      0x384d46e0 as std::os::raw::c_ulong as z_crc_t,
      0x398f2cd7 as std::os::raw::c_ulong as z_crc_t,
      0x3bc9928e as std::os::raw::c_ulong as z_crc_t,
      0x3a0bf8b9 as std::os::raw::c_ulong as z_crc_t,
      0x3f44ee3c as std::os::raw::c_ulong as z_crc_t,
      0x3e86840b as std::os::raw::c_ulong as z_crc_t,
      0x3cc03a52 as std::os::raw::c_ulong as z_crc_t,
      0x3d025065 as std::os::raw::c_ulong as z_crc_t,
      0x365e1758 as std::os::raw::c_ulong as z_crc_t,
      0x379c7d6f as std::os::raw::c_ulong as z_crc_t,
      0x35dac336 as std::os::raw::c_ulong as z_crc_t,
      0x3418a901 as std::os::raw::c_ulong as z_crc_t,
      0x3157bf84 as std::os::raw::c_ulong as z_crc_t,
      0x3095d5b3 as std::os::raw::c_ulong as z_crc_t,
      0x32d36bea as std::os::raw::c_ulong as z_crc_t,
      0x331101dd as std::os::raw::c_ulong as z_crc_t,
      0x246be590 as std::os::raw::c_ulong as z_crc_t,
      0x25a98fa7 as std::os::raw::c_ulong as z_crc_t,
      0x27ef31fe as std::os::raw::c_ulong as z_crc_t,
      0x262d5bc9 as std::os::raw::c_ulong as z_crc_t,
      0x23624d4c as std::os::raw::c_ulong as z_crc_t,
      0x22a0277b as std::os::raw::c_ulong as z_crc_t,
      0x20e69922 as std::os::raw::c_ulong as z_crc_t,
      0x2124f315 as std::os::raw::c_ulong as z_crc_t,
      0x2a78b428 as std::os::raw::c_ulong as z_crc_t,
      0x2bbade1f as std::os::raw::c_ulong as z_crc_t,
      0x29fc6046 as std::os::raw::c_ulong as z_crc_t,
      0x283e0a71 as std::os::raw::c_ulong as z_crc_t,
      0x2d711cf4 as std::os::raw::c_ulong as z_crc_t,
      0x2cb376c3 as std::os::raw::c_ulong as z_crc_t,
      0x2ef5c89a as std::os::raw::c_ulong as z_crc_t,
      0x2f37a2ad as std::os::raw::c_ulong as z_crc_t,
      0x709a8dc0 as std::os::raw::c_ulong as z_crc_t,
      0x7158e7f7 as std::os::raw::c_ulong as z_crc_t,
      0x731e59ae as std::os::raw::c_ulong as z_crc_t,
      0x72dc3399 as std::os::raw::c_ulong as z_crc_t,
      0x7793251c as std::os::raw::c_ulong as z_crc_t,
      0x76514f2b as std::os::raw::c_ulong as z_crc_t,
      0x7417f172 as std::os::raw::c_ulong as z_crc_t,
      0x75d59b45 as std::os::raw::c_ulong as z_crc_t,
      0x7e89dc78 as std::os::raw::c_ulong as z_crc_t,
      0x7f4bb64f as std::os::raw::c_ulong as z_crc_t,
      0x7d0d0816 as std::os::raw::c_ulong as z_crc_t,
      0x7ccf6221 as std::os::raw::c_ulong as z_crc_t,
      0x798074a4 as std::os::raw::c_ulong as z_crc_t,
      0x78421e93 as std::os::raw::c_ulong as z_crc_t,
      0x7a04a0ca as std::os::raw::c_ulong as z_crc_t,
      0x7bc6cafd as std::os::raw::c_ulong as z_crc_t,
      0x6cbc2eb0 as std::os::raw::c_ulong as z_crc_t,
      0x6d7e4487 as std::os::raw::c_ulong as z_crc_t,
      0x6f38fade as std::os::raw::c_ulong as z_crc_t,
      0x6efa90e9 as std::os::raw::c_ulong as z_crc_t,
      0x6bb5866c as std::os::raw::c_ulong as z_crc_t,
      0x6a77ec5b as std::os::raw::c_ulong as z_crc_t,
      0x68315202 as std::os::raw::c_ulong as z_crc_t,
      0x69f33835 as std::os::raw::c_ulong as z_crc_t,
      0x62af7f08 as std::os::raw::c_ulong as z_crc_t,
      0x636d153f as std::os::raw::c_ulong as z_crc_t,
      0x612bab66 as std::os::raw::c_ulong as z_crc_t,
      0x60e9c151 as std::os::raw::c_ulong as z_crc_t,
      0x65a6d7d4 as std::os::raw::c_ulong as z_crc_t,
      0x6464bde3 as std::os::raw::c_ulong as z_crc_t,
      0x662203ba as std::os::raw::c_ulong as z_crc_t,
      0x67e0698d as std::os::raw::c_ulong as z_crc_t,
      0x48d7cb20 as std::os::raw::c_ulong as z_crc_t,
      0x4915a117 as std::os::raw::c_ulong as z_crc_t,
      0x4b531f4e as std::os::raw::c_ulong as z_crc_t,
      0x4a917579 as std::os::raw::c_ulong as z_crc_t,
      0x4fde63fc as std::os::raw::c_ulong as z_crc_t,
      0x4e1c09cb as std::os::raw::c_ulong as z_crc_t,
      0x4c5ab792 as std::os::raw::c_ulong as z_crc_t,
      0x4d98dda5 as std::os::raw::c_ulong as z_crc_t,
      0x46c49a98 as std::os::raw::c_ulong as z_crc_t,
      0x4706f0af as std::os::raw::c_ulong as z_crc_t,
      0x45404ef6 as std::os::raw::c_ulong as z_crc_t,
      0x448224c1 as std::os::raw::c_ulong as z_crc_t,
      0x41cd3244 as std::os::raw::c_ulong as z_crc_t,
      0x400f5873 as std::os::raw::c_ulong as z_crc_t,
      0x4249e62a as std::os::raw::c_ulong as z_crc_t,
      0x438b8c1d as std::os::raw::c_ulong as z_crc_t,
      0x54f16850 as std::os::raw::c_ulong as z_crc_t,
      0x55330267 as std::os::raw::c_ulong as z_crc_t,
      0x5775bc3e as std::os::raw::c_ulong as z_crc_t,
      0x56b7d609 as std::os::raw::c_ulong as z_crc_t,
      0x53f8c08c as std::os::raw::c_ulong as z_crc_t,
      0x523aaabb as std::os::raw::c_ulong as z_crc_t,
      0x507c14e2 as std::os::raw::c_ulong as z_crc_t,
      0x51be7ed5 as std::os::raw::c_ulong as z_crc_t,
      0x5ae239e8 as std::os::raw::c_ulong as z_crc_t,
      0x5b2053df as std::os::raw::c_ulong as z_crc_t,
      0x5966ed86 as std::os::raw::c_ulong as z_crc_t,
      0x58a487b1 as std::os::raw::c_ulong as z_crc_t,
      0x5deb9134 as std::os::raw::c_ulong as z_crc_t,
      0x5c29fb03 as std::os::raw::c_ulong as z_crc_t,
      0x5e6f455a as std::os::raw::c_ulong as z_crc_t,
      0x5fad2f6d as std::os::raw::c_ulong as z_crc_t,
      0xe1351b80 as std::os::raw::c_ulong as z_crc_t,
      0xe0f771b7 as std::os::raw::c_ulong as z_crc_t,
      0xe2b1cfee as std::os::raw::c_ulong as z_crc_t,
      0xe373a5d9 as std::os::raw::c_ulong as z_crc_t,
      0xe63cb35c as std::os::raw::c_ulong as z_crc_t,
      0xe7fed96b as std::os::raw::c_ulong as z_crc_t,
      0xe5b86732 as std::os::raw::c_ulong as z_crc_t,
      0xe47a0d05 as std::os::raw::c_ulong as z_crc_t,
      0xef264a38 as std::os::raw::c_ulong as z_crc_t,
      0xeee4200f as std::os::raw::c_ulong as z_crc_t,
      0xeca29e56 as std::os::raw::c_ulong as z_crc_t,
      0xed60f461 as std::os::raw::c_ulong as z_crc_t,
      0xe82fe2e4 as std::os::raw::c_ulong as z_crc_t,
      0xe9ed88d3 as std::os::raw::c_ulong as z_crc_t,
      0xebab368a as std::os::raw::c_ulong as z_crc_t,
      0xea695cbd as std::os::raw::c_ulong as z_crc_t,
      0xfd13b8f0 as std::os::raw::c_ulong as z_crc_t,
      0xfcd1d2c7 as std::os::raw::c_ulong as z_crc_t,
      0xfe976c9e as std::os::raw::c_ulong as z_crc_t,
      0xff5506a9 as std::os::raw::c_ulong as z_crc_t,
      0xfa1a102c as std::os::raw::c_ulong as z_crc_t,
      0xfbd87a1b as std::os::raw::c_ulong as z_crc_t,
      0xf99ec442 as std::os::raw::c_ulong as z_crc_t,
      0xf85cae75 as std::os::raw::c_ulong as z_crc_t,
      0xf300e948 as std::os::raw::c_ulong as z_crc_t,
      0xf2c2837f as std::os::raw::c_ulong as z_crc_t,
      0xf0843d26 as std::os::raw::c_ulong as z_crc_t,
      0xf1465711 as std::os::raw::c_ulong as z_crc_t,
      0xf4094194 as std::os::raw::c_ulong as z_crc_t,
      0xf5cb2ba3 as std::os::raw::c_ulong as z_crc_t,
      0xf78d95fa as std::os::raw::c_ulong as z_crc_t,
      0xf64fffcd as std::os::raw::c_ulong as z_crc_t,
      0xd9785d60 as std::os::raw::c_ulong as z_crc_t,
      0xd8ba3757 as std::os::raw::c_ulong as z_crc_t,
      0xdafc890e as std::os::raw::c_ulong as z_crc_t,
      0xdb3ee339 as std::os::raw::c_ulong as z_crc_t,
      0xde71f5bc as std::os::raw::c_ulong as z_crc_t,
      0xdfb39f8b as std::os::raw::c_ulong as z_crc_t,
      0xddf521d2 as std::os::raw::c_ulong as z_crc_t,
      0xdc374be5 as std::os::raw::c_ulong as z_crc_t,
      0xd76b0cd8 as std::os::raw::c_ulong as z_crc_t,
      0xd6a966ef as std::os::raw::c_ulong as z_crc_t,
      0xd4efd8b6 as std::os::raw::c_ulong as z_crc_t,
      0xd52db281 as std::os::raw::c_ulong as z_crc_t,
      0xd062a404 as std::os::raw::c_ulong as z_crc_t,
      0xd1a0ce33 as std::os::raw::c_ulong as z_crc_t,
      0xd3e6706a as std::os::raw::c_ulong as z_crc_t,
      0xd2241a5d as std::os::raw::c_ulong as z_crc_t,
      0xc55efe10 as std::os::raw::c_ulong as z_crc_t,
      0xc49c9427 as std::os::raw::c_ulong as z_crc_t,
      0xc6da2a7e as std::os::raw::c_ulong as z_crc_t,
      0xc7184049 as std::os::raw::c_ulong as z_crc_t,
      0xc25756cc as std::os::raw::c_ulong as z_crc_t,
      0xc3953cfb as std::os::raw::c_ulong as z_crc_t,
      0xc1d382a2 as std::os::raw::c_ulong as z_crc_t,
      0xc011e895 as std::os::raw::c_ulong as z_crc_t,
      0xcb4dafa8 as std::os::raw::c_ulong as z_crc_t,
      0xca8fc59f as std::os::raw::c_ulong as z_crc_t,
      0xc8c97bc6 as std::os::raw::c_ulong as z_crc_t,
      0xc90b11f1 as std::os::raw::c_ulong as z_crc_t,
      0xcc440774 as std::os::raw::c_ulong as z_crc_t,
      0xcd866d43 as std::os::raw::c_ulong as z_crc_t,
      0xcfc0d31a as std::os::raw::c_ulong as z_crc_t,
      0xce02b92d as std::os::raw::c_ulong as z_crc_t,
      0x91af9640 as std::os::raw::c_ulong as z_crc_t,
      0x906dfc77 as std::os::raw::c_ulong as z_crc_t,
      0x922b422e as std::os::raw::c_ulong as z_crc_t,
      0x93e92819 as std::os::raw::c_ulong as z_crc_t,
      0x96a63e9c as std::os::raw::c_ulong as z_crc_t,
      0x976454ab as std::os::raw::c_ulong as z_crc_t,
      0x9522eaf2 as std::os::raw::c_ulong as z_crc_t,
      0x94e080c5 as std::os::raw::c_ulong as z_crc_t,
      0x9fbcc7f8 as std::os::raw::c_ulong as z_crc_t,
      0x9e7eadcf as std::os::raw::c_ulong as z_crc_t,
      0x9c381396 as std::os::raw::c_ulong as z_crc_t,
      0x9dfa79a1 as std::os::raw::c_ulong as z_crc_t,
      0x98b56f24 as std::os::raw::c_ulong as z_crc_t,
      0x99770513 as std::os::raw::c_ulong as z_crc_t,
      0x9b31bb4a as std::os::raw::c_ulong as z_crc_t,
      0x9af3d17d as std::os::raw::c_ulong as z_crc_t,
      0x8d893530 as std::os::raw::c_ulong as z_crc_t,
      0x8c4b5f07 as std::os::raw::c_ulong as z_crc_t,
      0x8e0de15e as std::os::raw::c_ulong as z_crc_t,
      0x8fcf8b69 as std::os::raw::c_ulong as z_crc_t,
      0x8a809dec as std::os::raw::c_ulong as z_crc_t,
      0x8b42f7db as std::os::raw::c_ulong as z_crc_t,
      0x89044982 as std::os::raw::c_ulong as z_crc_t,
      0x88c623b5 as std::os::raw::c_ulong as z_crc_t,
      0x839a6488 as std::os::raw::c_ulong as z_crc_t,
      0x82580ebf as std::os::raw::c_ulong as z_crc_t,
      0x801eb0e6 as std::os::raw::c_ulong as z_crc_t,
      0x81dcdad1 as std::os::raw::c_ulong as z_crc_t,
      0x8493cc54 as std::os::raw::c_ulong as z_crc_t,
      0x8551a663 as std::os::raw::c_ulong as z_crc_t,
      0x8717183a as std::os::raw::c_ulong as z_crc_t,
      0x86d5720d as std::os::raw::c_ulong as z_crc_t,
      0xa9e2d0a0 as std::os::raw::c_ulong as z_crc_t,
      0xa820ba97 as std::os::raw::c_ulong as z_crc_t,
      0xaa6604ce as std::os::raw::c_ulong as z_crc_t,
      0xaba46ef9 as std::os::raw::c_ulong as z_crc_t,
      0xaeeb787c as std::os::raw::c_ulong as z_crc_t,
      0xaf29124b as std::os::raw::c_ulong as z_crc_t,
      0xad6fac12 as std::os::raw::c_ulong as z_crc_t,
      0xacadc625 as std::os::raw::c_ulong as z_crc_t,
      0xa7f18118 as std::os::raw::c_ulong as z_crc_t,
      0xa633eb2f as std::os::raw::c_ulong as z_crc_t,
      0xa4755576 as std::os::raw::c_ulong as z_crc_t,
      0xa5b73f41 as std::os::raw::c_ulong as z_crc_t,
      0xa0f829c4 as std::os::raw::c_ulong as z_crc_t,
      0xa13a43f3 as std::os::raw::c_ulong as z_crc_t,
      0xa37cfdaa as std::os::raw::c_ulong as z_crc_t,
      0xa2be979d as std::os::raw::c_ulong as z_crc_t,
      0xb5c473d0 as std::os::raw::c_ulong as z_crc_t,
      0xb40619e7 as std::os::raw::c_ulong as z_crc_t,
      0xb640a7be as std::os::raw::c_ulong as z_crc_t,
      0xb782cd89 as std::os::raw::c_ulong as z_crc_t,
      0xb2cddb0c as std::os::raw::c_ulong as z_crc_t,
      0xb30fb13b as std::os::raw::c_ulong as z_crc_t,
      0xb1490f62 as std::os::raw::c_ulong as z_crc_t,
      0xb08b6555 as std::os::raw::c_ulong as z_crc_t,
      0xbbd72268 as std::os::raw::c_ulong as z_crc_t,
      0xba15485f as std::os::raw::c_ulong as z_crc_t,
      0xb853f606 as std::os::raw::c_ulong as z_crc_t,
      0xb9919c31 as std::os::raw::c_ulong as z_crc_t,
      0xbcde8ab4 as std::os::raw::c_ulong as z_crc_t,
      0xbd1ce083 as std::os::raw::c_ulong as z_crc_t,
      0xbf5a5eda as std::os::raw::c_ulong as z_crc_t,
      0xbe9834ed as std::os::raw::c_ulong as z_crc_t],
     [0 as std::os::raw::c_ulong as z_crc_t, 0xb8bc6765 as std::os::raw::c_ulong as z_crc_t,
      0xaa09c88b as std::os::raw::c_ulong as z_crc_t,
      0x12b5afee as std::os::raw::c_ulong as z_crc_t,
      0x8f629757 as std::os::raw::c_ulong as z_crc_t,
      0x37def032 as std::os::raw::c_ulong as z_crc_t,
      0x256b5fdc as std::os::raw::c_ulong as z_crc_t,
      0x9dd738b9 as std::os::raw::c_ulong as z_crc_t,
      0xc5b428ef as std::os::raw::c_ulong as z_crc_t,
      0x7d084f8a as std::os::raw::c_ulong as z_crc_t,
      0x6fbde064 as std::os::raw::c_ulong as z_crc_t,
      0xd7018701 as std::os::raw::c_ulong as z_crc_t,
      0x4ad6bfb8 as std::os::raw::c_ulong as z_crc_t,
      0xf26ad8dd as std::os::raw::c_ulong as z_crc_t,
      0xe0df7733 as std::os::raw::c_ulong as z_crc_t,
      0x58631056 as std::os::raw::c_ulong as z_crc_t,
      0x5019579f as std::os::raw::c_ulong as z_crc_t,
      0xe8a530fa as std::os::raw::c_ulong as z_crc_t,
      0xfa109f14 as std::os::raw::c_ulong as z_crc_t,
      0x42acf871 as std::os::raw::c_ulong as z_crc_t,
      0xdf7bc0c8 as std::os::raw::c_ulong as z_crc_t,
      0x67c7a7ad as std::os::raw::c_ulong as z_crc_t,
      0x75720843 as std::os::raw::c_ulong as z_crc_t,
      0xcdce6f26 as std::os::raw::c_ulong as z_crc_t,
      0x95ad7f70 as std::os::raw::c_ulong as z_crc_t,
      0x2d111815 as std::os::raw::c_ulong as z_crc_t,
      0x3fa4b7fb as std::os::raw::c_ulong as z_crc_t,
      0x8718d09e as std::os::raw::c_ulong as z_crc_t,
      0x1acfe827 as std::os::raw::c_ulong as z_crc_t,
      0xa2738f42 as std::os::raw::c_ulong as z_crc_t,
      0xb0c620ac as std::os::raw::c_ulong as z_crc_t,
      0x87a47c9 as std::os::raw::c_ulong as z_crc_t,
      0xa032af3e as std::os::raw::c_ulong as z_crc_t,
      0x188ec85b as std::os::raw::c_ulong as z_crc_t,
      0xa3b67b5 as std::os::raw::c_ulong as z_crc_t,
      0xb28700d0 as std::os::raw::c_ulong as z_crc_t,
      0x2f503869 as std::os::raw::c_ulong as z_crc_t,
      0x97ec5f0c as std::os::raw::c_ulong as z_crc_t,
      0x8559f0e2 as std::os::raw::c_ulong as z_crc_t,
      0x3de59787 as std::os::raw::c_ulong as z_crc_t,
      0x658687d1 as std::os::raw::c_ulong as z_crc_t,
      0xdd3ae0b4 as std::os::raw::c_ulong as z_crc_t,
      0xcf8f4f5a as std::os::raw::c_ulong as z_crc_t,
      0x7733283f as std::os::raw::c_ulong as z_crc_t,
      0xeae41086 as std::os::raw::c_ulong as z_crc_t,
      0x525877e3 as std::os::raw::c_ulong as z_crc_t,
      0x40edd80d as std::os::raw::c_ulong as z_crc_t,
      0xf851bf68 as std::os::raw::c_ulong as z_crc_t,
      0xf02bf8a1 as std::os::raw::c_ulong as z_crc_t,
      0x48979fc4 as std::os::raw::c_ulong as z_crc_t,
      0x5a22302a as std::os::raw::c_ulong as z_crc_t,
      0xe29e574f as std::os::raw::c_ulong as z_crc_t,
      0x7f496ff6 as std::os::raw::c_ulong as z_crc_t,
      0xc7f50893 as std::os::raw::c_ulong as z_crc_t,
      0xd540a77d as std::os::raw::c_ulong as z_crc_t,
      0x6dfcc018 as std::os::raw::c_ulong as z_crc_t,
      0x359fd04e as std::os::raw::c_ulong as z_crc_t,
      0x8d23b72b as std::os::raw::c_ulong as z_crc_t,
      0x9f9618c5 as std::os::raw::c_ulong as z_crc_t,
      0x272a7fa0 as std::os::raw::c_ulong as z_crc_t,
      0xbafd4719 as std::os::raw::c_ulong as z_crc_t,
      0x241207c as std::os::raw::c_ulong as z_crc_t,
      0x10f48f92 as std::os::raw::c_ulong as z_crc_t,
      0xa848e8f7 as std::os::raw::c_ulong as z_crc_t,
      0x9b14583d as std::os::raw::c_ulong as z_crc_t,
      0x23a83f58 as std::os::raw::c_ulong as z_crc_t,
      0x311d90b6 as std::os::raw::c_ulong as z_crc_t,
      0x89a1f7d3 as std::os::raw::c_ulong as z_crc_t,
      0x1476cf6a as std::os::raw::c_ulong as z_crc_t,
      0xaccaa80f as std::os::raw::c_ulong as z_crc_t,
      0xbe7f07e1 as std::os::raw::c_ulong as z_crc_t,
      0x6c36084 as std::os::raw::c_ulong as z_crc_t,
      0x5ea070d2 as std::os::raw::c_ulong as z_crc_t,
      0xe61c17b7 as std::os::raw::c_ulong as z_crc_t,
      0xf4a9b859 as std::os::raw::c_ulong as z_crc_t,
      0x4c15df3c as std::os::raw::c_ulong as z_crc_t,
      0xd1c2e785 as std::os::raw::c_ulong as z_crc_t,
      0x697e80e0 as std::os::raw::c_ulong as z_crc_t,
      0x7bcb2f0e as std::os::raw::c_ulong as z_crc_t,
      0xc377486b as std::os::raw::c_ulong as z_crc_t,
      0xcb0d0fa2 as std::os::raw::c_ulong as z_crc_t,
      0x73b168c7 as std::os::raw::c_ulong as z_crc_t,
      0x6104c729 as std::os::raw::c_ulong as z_crc_t,
      0xd9b8a04c as std::os::raw::c_ulong as z_crc_t,
      0x446f98f5 as std::os::raw::c_ulong as z_crc_t,
      0xfcd3ff90 as std::os::raw::c_ulong as z_crc_t,
      0xee66507e as std::os::raw::c_ulong as z_crc_t,
      0x56da371b as std::os::raw::c_ulong as z_crc_t,
      0xeb9274d as std::os::raw::c_ulong as z_crc_t,
      0xb6054028 as std::os::raw::c_ulong as z_crc_t,
      0xa4b0efc6 as std::os::raw::c_ulong as z_crc_t,
      0x1c0c88a3 as std::os::raw::c_ulong as z_crc_t,
      0x81dbb01a as std::os::raw::c_ulong as z_crc_t,
      0x3967d77f as std::os::raw::c_ulong as z_crc_t,
      0x2bd27891 as std::os::raw::c_ulong as z_crc_t,
      0x936e1ff4 as std::os::raw::c_ulong as z_crc_t,
      0x3b26f703 as std::os::raw::c_ulong as z_crc_t,
      0x839a9066 as std::os::raw::c_ulong as z_crc_t,
      0x912f3f88 as std::os::raw::c_ulong as z_crc_t,
      0x299358ed as std::os::raw::c_ulong as z_crc_t,
      0xb4446054 as std::os::raw::c_ulong as z_crc_t,
      0xcf80731 as std::os::raw::c_ulong as z_crc_t,
      0x1e4da8df as std::os::raw::c_ulong as z_crc_t,
      0xa6f1cfba as std::os::raw::c_ulong as z_crc_t,
      0xfe92dfec as std::os::raw::c_ulong as z_crc_t,
      0x462eb889 as std::os::raw::c_ulong as z_crc_t,
      0x549b1767 as std::os::raw::c_ulong as z_crc_t,
      0xec277002 as std::os::raw::c_ulong as z_crc_t,
      0x71f048bb as std::os::raw::c_ulong as z_crc_t,
      0xc94c2fde as std::os::raw::c_ulong as z_crc_t,
      0xdbf98030 as std::os::raw::c_ulong as z_crc_t,
      0x6345e755 as std::os::raw::c_ulong as z_crc_t,
      0x6b3fa09c as std::os::raw::c_ulong as z_crc_t,
      0xd383c7f9 as std::os::raw::c_ulong as z_crc_t,
      0xc1366817 as std::os::raw::c_ulong as z_crc_t,
      0x798a0f72 as std::os::raw::c_ulong as z_crc_t,
      0xe45d37cb as std::os::raw::c_ulong as z_crc_t,
      0x5ce150ae as std::os::raw::c_ulong as z_crc_t,
      0x4e54ff40 as std::os::raw::c_ulong as z_crc_t,
      0xf6e89825 as std::os::raw::c_ulong as z_crc_t,
      0xae8b8873 as std::os::raw::c_ulong as z_crc_t,
      0x1637ef16 as std::os::raw::c_ulong as z_crc_t,
      0x48240f8 as std::os::raw::c_ulong as z_crc_t,
      0xbc3e279d as std::os::raw::c_ulong as z_crc_t,
      0x21e91f24 as std::os::raw::c_ulong as z_crc_t,
      0x99557841 as std::os::raw::c_ulong as z_crc_t,
      0x8be0d7af as std::os::raw::c_ulong as z_crc_t,
      0x335cb0ca as std::os::raw::c_ulong as z_crc_t,
      0xed59b63b as std::os::raw::c_ulong as z_crc_t,
      0x55e5d15e as std::os::raw::c_ulong as z_crc_t,
      0x47507eb0 as std::os::raw::c_ulong as z_crc_t,
      0xffec19d5 as std::os::raw::c_ulong as z_crc_t,
      0x623b216c as std::os::raw::c_ulong as z_crc_t,
      0xda874609 as std::os::raw::c_ulong as z_crc_t,
      0xc832e9e7 as std::os::raw::c_ulong as z_crc_t,
      0x708e8e82 as std::os::raw::c_ulong as z_crc_t,
      0x28ed9ed4 as std::os::raw::c_ulong as z_crc_t,
      0x9051f9b1 as std::os::raw::c_ulong as z_crc_t,
      0x82e4565f as std::os::raw::c_ulong as z_crc_t,
      0x3a58313a as std::os::raw::c_ulong as z_crc_t,
      0xa78f0983 as std::os::raw::c_ulong as z_crc_t,
      0x1f336ee6 as std::os::raw::c_ulong as z_crc_t,
      0xd86c108 as std::os::raw::c_ulong as z_crc_t,
      0xb53aa66d as std::os::raw::c_ulong as z_crc_t,
      0xbd40e1a4 as std::os::raw::c_ulong as z_crc_t,
      0x5fc86c1 as std::os::raw::c_ulong as z_crc_t,
      0x1749292f as std::os::raw::c_ulong as z_crc_t,
      0xaff54e4a as std::os::raw::c_ulong as z_crc_t,
      0x322276f3 as std::os::raw::c_ulong as z_crc_t,
      0x8a9e1196 as std::os::raw::c_ulong as z_crc_t,
      0x982bbe78 as std::os::raw::c_ulong as z_crc_t,
      0x2097d91d as std::os::raw::c_ulong as z_crc_t,
      0x78f4c94b as std::os::raw::c_ulong as z_crc_t,
      0xc048ae2e as std::os::raw::c_ulong as z_crc_t,
      0xd2fd01c0 as std::os::raw::c_ulong as z_crc_t,
      0x6a4166a5 as std::os::raw::c_ulong as z_crc_t,
      0xf7965e1c as std::os::raw::c_ulong as z_crc_t,
      0x4f2a3979 as std::os::raw::c_ulong as z_crc_t,
      0x5d9f9697 as std::os::raw::c_ulong as z_crc_t,
      0xe523f1f2 as std::os::raw::c_ulong as z_crc_t,
      0x4d6b1905 as std::os::raw::c_ulong as z_crc_t,
      0xf5d77e60 as std::os::raw::c_ulong as z_crc_t,
      0xe762d18e as std::os::raw::c_ulong as z_crc_t,
      0x5fdeb6eb as std::os::raw::c_ulong as z_crc_t,
      0xc2098e52 as std::os::raw::c_ulong as z_crc_t,
      0x7ab5e937 as std::os::raw::c_ulong as z_crc_t,
      0x680046d9 as std::os::raw::c_ulong as z_crc_t,
      0xd0bc21bc as std::os::raw::c_ulong as z_crc_t,
      0x88df31ea as std::os::raw::c_ulong as z_crc_t,
      0x3063568f as std::os::raw::c_ulong as z_crc_t,
      0x22d6f961 as std::os::raw::c_ulong as z_crc_t,
      0x9a6a9e04 as std::os::raw::c_ulong as z_crc_t,
      0x7bda6bd as std::os::raw::c_ulong as z_crc_t,
      0xbf01c1d8 as std::os::raw::c_ulong as z_crc_t,
      0xadb46e36 as std::os::raw::c_ulong as z_crc_t,
      0x15080953 as std::os::raw::c_ulong as z_crc_t,
      0x1d724e9a as std::os::raw::c_ulong as z_crc_t,
      0xa5ce29ff as std::os::raw::c_ulong as z_crc_t,
      0xb77b8611 as std::os::raw::c_ulong as z_crc_t,
      0xfc7e174 as std::os::raw::c_ulong as z_crc_t,
      0x9210d9cd as std::os::raw::c_ulong as z_crc_t,
      0x2aacbea8 as std::os::raw::c_ulong as z_crc_t,
      0x38191146 as std::os::raw::c_ulong as z_crc_t,
      0x80a57623 as std::os::raw::c_ulong as z_crc_t,
      0xd8c66675 as std::os::raw::c_ulong as z_crc_t,
      0x607a0110 as std::os::raw::c_ulong as z_crc_t,
      0x72cfaefe as std::os::raw::c_ulong as z_crc_t,
      0xca73c99b as std::os::raw::c_ulong as z_crc_t,
      0x57a4f122 as std::os::raw::c_ulong as z_crc_t,
      0xef189647 as std::os::raw::c_ulong as z_crc_t,
      0xfdad39a9 as std::os::raw::c_ulong as z_crc_t,
      0x45115ecc as std::os::raw::c_ulong as z_crc_t,
      0x764dee06 as std::os::raw::c_ulong as z_crc_t,
      0xcef18963 as std::os::raw::c_ulong as z_crc_t,
      0xdc44268d as std::os::raw::c_ulong as z_crc_t,
      0x64f841e8 as std::os::raw::c_ulong as z_crc_t,
      0xf92f7951 as std::os::raw::c_ulong as z_crc_t,
      0x41931e34 as std::os::raw::c_ulong as z_crc_t,
      0x5326b1da as std::os::raw::c_ulong as z_crc_t,
      0xeb9ad6bf as std::os::raw::c_ulong as z_crc_t,
      0xb3f9c6e9 as std::os::raw::c_ulong as z_crc_t,
      0xb45a18c as std::os::raw::c_ulong as z_crc_t,
      0x19f00e62 as std::os::raw::c_ulong as z_crc_t,
      0xa14c6907 as std::os::raw::c_ulong as z_crc_t,
      0x3c9b51be as std::os::raw::c_ulong as z_crc_t,
      0x842736db as std::os::raw::c_ulong as z_crc_t,
      0x96929935 as std::os::raw::c_ulong as z_crc_t,
      0x2e2efe50 as std::os::raw::c_ulong as z_crc_t,
      0x2654b999 as std::os::raw::c_ulong as z_crc_t,
      0x9ee8defc as std::os::raw::c_ulong as z_crc_t,
      0x8c5d7112 as std::os::raw::c_ulong as z_crc_t,
      0x34e11677 as std::os::raw::c_ulong as z_crc_t,
      0xa9362ece as std::os::raw::c_ulong as z_crc_t,
      0x118a49ab as std::os::raw::c_ulong as z_crc_t,
      0x33fe645 as std::os::raw::c_ulong as z_crc_t,
      0xbb838120 as std::os::raw::c_ulong as z_crc_t,
      0xe3e09176 as std::os::raw::c_ulong as z_crc_t,
      0x5b5cf613 as std::os::raw::c_ulong as z_crc_t,
      0x49e959fd as std::os::raw::c_ulong as z_crc_t,
      0xf1553e98 as std::os::raw::c_ulong as z_crc_t,
      0x6c820621 as std::os::raw::c_ulong as z_crc_t,
      0xd43e6144 as std::os::raw::c_ulong as z_crc_t,
      0xc68bceaa as std::os::raw::c_ulong as z_crc_t,
      0x7e37a9cf as std::os::raw::c_ulong as z_crc_t,
      0xd67f4138 as std::os::raw::c_ulong as z_crc_t,
      0x6ec3265d as std::os::raw::c_ulong as z_crc_t,
      0x7c7689b3 as std::os::raw::c_ulong as z_crc_t,
      0xc4caeed6 as std::os::raw::c_ulong as z_crc_t,
      0x591dd66f as std::os::raw::c_ulong as z_crc_t,
      0xe1a1b10a as std::os::raw::c_ulong as z_crc_t,
      0xf3141ee4 as std::os::raw::c_ulong as z_crc_t,
      0x4ba87981 as std::os::raw::c_ulong as z_crc_t,
      0x13cb69d7 as std::os::raw::c_ulong as z_crc_t,
      0xab770eb2 as std::os::raw::c_ulong as z_crc_t,
      0xb9c2a15c as std::os::raw::c_ulong as z_crc_t,
      0x17ec639 as std::os::raw::c_ulong as z_crc_t,
      0x9ca9fe80 as std::os::raw::c_ulong as z_crc_t,
      0x241599e5 as std::os::raw::c_ulong as z_crc_t,
      0x36a0360b as std::os::raw::c_ulong as z_crc_t,
      0x8e1c516e as std::os::raw::c_ulong as z_crc_t,
      0x866616a7 as std::os::raw::c_ulong as z_crc_t,
      0x3eda71c2 as std::os::raw::c_ulong as z_crc_t,
      0x2c6fde2c as std::os::raw::c_ulong as z_crc_t,
      0x94d3b949 as std::os::raw::c_ulong as z_crc_t,
      0x90481f0 as std::os::raw::c_ulong as z_crc_t,
      0xb1b8e695 as std::os::raw::c_ulong as z_crc_t,
      0xa30d497b as std::os::raw::c_ulong as z_crc_t,
      0x1bb12e1e as std::os::raw::c_ulong as z_crc_t,
      0x43d23e48 as std::os::raw::c_ulong as z_crc_t,
      0xfb6e592d as std::os::raw::c_ulong as z_crc_t,
      0xe9dbf6c3 as std::os::raw::c_ulong as z_crc_t,
      0x516791a6 as std::os::raw::c_ulong as z_crc_t,
      0xccb0a91f as std::os::raw::c_ulong as z_crc_t,
      0x740cce7a as std::os::raw::c_ulong as z_crc_t,
      0x66b96194 as std::os::raw::c_ulong as z_crc_t,
      0xde0506f1 as std::os::raw::c_ulong as z_crc_t],
     [0 as std::os::raw::c_ulong as z_crc_t, 0x96300777 as std::os::raw::c_ulong as z_crc_t,
      0x2c610eee as std::os::raw::c_ulong as z_crc_t,
      0xba510999 as std::os::raw::c_ulong as z_crc_t,
      0x19c46d07 as std::os::raw::c_ulong as z_crc_t,
      0x8ff46a70 as std::os::raw::c_ulong as z_crc_t,
      0x35a563e9 as std::os::raw::c_ulong as z_crc_t,
      0xa395649e as std::os::raw::c_ulong as z_crc_t,
      0x3288db0e as std::os::raw::c_ulong as z_crc_t,
      0xa4b8dc79 as std::os::raw::c_ulong as z_crc_t,
      0x1ee9d5e0 as std::os::raw::c_ulong as z_crc_t,
      0x88d9d297 as std::os::raw::c_ulong as z_crc_t,
      0x2b4cb609 as std::os::raw::c_ulong as z_crc_t,
      0xbd7cb17e as std::os::raw::c_ulong as z_crc_t,
      0x72db8e7 as std::os::raw::c_ulong as z_crc_t,
      0x911dbf90 as std::os::raw::c_ulong as z_crc_t,
      0x6410b71d as std::os::raw::c_ulong as z_crc_t,
      0xf220b06a as std::os::raw::c_ulong as z_crc_t,
      0x4871b9f3 as std::os::raw::c_ulong as z_crc_t,
      0xde41be84 as std::os::raw::c_ulong as z_crc_t,
      0x7dd4da1a as std::os::raw::c_ulong as z_crc_t,
      0xebe4dd6d as std::os::raw::c_ulong as z_crc_t,
      0x51b5d4f4 as std::os::raw::c_ulong as z_crc_t,
      0xc785d383 as std::os::raw::c_ulong as z_crc_t,
      0x56986c13 as std::os::raw::c_ulong as z_crc_t,
      0xc0a86b64 as std::os::raw::c_ulong as z_crc_t,
      0x7af962fd as std::os::raw::c_ulong as z_crc_t,
      0xecc9658a as std::os::raw::c_ulong as z_crc_t,
      0x4f5c0114 as std::os::raw::c_ulong as z_crc_t,
      0xd96c0663 as std::os::raw::c_ulong as z_crc_t,
      0x633d0ffa as std::os::raw::c_ulong as z_crc_t,
      0xf50d088d as std::os::raw::c_ulong as z_crc_t,
      0xc8206e3b as std::os::raw::c_ulong as z_crc_t,
      0x5e10694c as std::os::raw::c_ulong as z_crc_t,
      0xe44160d5 as std::os::raw::c_ulong as z_crc_t,
      0x727167a2 as std::os::raw::c_ulong as z_crc_t,
      0xd1e4033c as std::os::raw::c_ulong as z_crc_t,
      0x47d4044b as std::os::raw::c_ulong as z_crc_t,
      0xfd850dd2 as std::os::raw::c_ulong as z_crc_t,
      0x6bb50aa5 as std::os::raw::c_ulong as z_crc_t,
      0xfaa8b535 as std::os::raw::c_ulong as z_crc_t,
      0x6c98b242 as std::os::raw::c_ulong as z_crc_t,
      0xd6c9bbdb as std::os::raw::c_ulong as z_crc_t,
      0x40f9bcac as std::os::raw::c_ulong as z_crc_t,
      0xe36cd832 as std::os::raw::c_ulong as z_crc_t,
      0x755cdf45 as std::os::raw::c_ulong as z_crc_t,
      0xcf0dd6dc as std::os::raw::c_ulong as z_crc_t,
      0x593dd1ab as std::os::raw::c_ulong as z_crc_t,
      0xac30d926 as std::os::raw::c_ulong as z_crc_t,
      0x3a00de51 as std::os::raw::c_ulong as z_crc_t,
      0x8051d7c8 as std::os::raw::c_ulong as z_crc_t,
      0x1661d0bf as std::os::raw::c_ulong as z_crc_t,
      0xb5f4b421 as std::os::raw::c_ulong as z_crc_t,
      0x23c4b356 as std::os::raw::c_ulong as z_crc_t,
      0x9995bacf as std::os::raw::c_ulong as z_crc_t,
      0xfa5bdb8 as std::os::raw::c_ulong as z_crc_t,
      0x9eb80228 as std::os::raw::c_ulong as z_crc_t,
      0x888055f as std::os::raw::c_ulong as z_crc_t,
      0xb2d90cc6 as std::os::raw::c_ulong as z_crc_t,
      0x24e90bb1 as std::os::raw::c_ulong as z_crc_t,
      0x877c6f2f as std::os::raw::c_ulong as z_crc_t,
      0x114c6858 as std::os::raw::c_ulong as z_crc_t,
      0xab1d61c1 as std::os::raw::c_ulong as z_crc_t,
      0x3d2d66b6 as std::os::raw::c_ulong as z_crc_t,
      0x9041dc76 as std::os::raw::c_ulong as z_crc_t,
      0x671db01 as std::os::raw::c_ulong as z_crc_t,
      0xbc20d298 as std::os::raw::c_ulong as z_crc_t,
      0x2a10d5ef as std::os::raw::c_ulong as z_crc_t,
      0x8985b171 as std::os::raw::c_ulong as z_crc_t,
      0x1fb5b606 as std::os::raw::c_ulong as z_crc_t,
      0xa5e4bf9f as std::os::raw::c_ulong as z_crc_t,
      0x33d4b8e8 as std::os::raw::c_ulong as z_crc_t,
      0xa2c90778 as std::os::raw::c_ulong as z_crc_t,
      0x34f9000f as std::os::raw::c_ulong as z_crc_t,
      0x8ea80996 as std::os::raw::c_ulong as z_crc_t,
      0x18980ee1 as std::os::raw::c_ulong as z_crc_t,
      0xbb0d6a7f as std::os::raw::c_ulong as z_crc_t,
      0x2d3d6d08 as std::os::raw::c_ulong as z_crc_t,
      0x976c6491 as std::os::raw::c_ulong as z_crc_t,
      0x15c63e6 as std::os::raw::c_ulong as z_crc_t,
      0xf4516b6b as std::os::raw::c_ulong as z_crc_t,
      0x62616c1c as std::os::raw::c_ulong as z_crc_t,
      0xd8306585 as std::os::raw::c_ulong as z_crc_t,
      0x4e0062f2 as std::os::raw::c_ulong as z_crc_t,
      0xed95066c as std::os::raw::c_ulong as z_crc_t,
      0x7ba5011b as std::os::raw::c_ulong as z_crc_t,
      0xc1f40882 as std::os::raw::c_ulong as z_crc_t,
      0x57c40ff5 as std::os::raw::c_ulong as z_crc_t,
      0xc6d9b065 as std::os::raw::c_ulong as z_crc_t,
      0x50e9b712 as std::os::raw::c_ulong as z_crc_t,
      0xeab8be8b as std::os::raw::c_ulong as z_crc_t,
      0x7c88b9fc as std::os::raw::c_ulong as z_crc_t,
      0xdf1ddd62 as std::os::raw::c_ulong as z_crc_t,
      0x492dda15 as std::os::raw::c_ulong as z_crc_t,
      0xf37cd38c as std::os::raw::c_ulong as z_crc_t,
      0x654cd4fb as std::os::raw::c_ulong as z_crc_t,
      0x5861b24d as std::os::raw::c_ulong as z_crc_t,
      0xce51b53a as std::os::raw::c_ulong as z_crc_t,
      0x7400bca3 as std::os::raw::c_ulong as z_crc_t,
      0xe230bbd4 as std::os::raw::c_ulong as z_crc_t,
      0x41a5df4a as std::os::raw::c_ulong as z_crc_t,
      0xd795d83d as std::os::raw::c_ulong as z_crc_t,
      0x6dc4d1a4 as std::os::raw::c_ulong as z_crc_t,
      0xfbf4d6d3 as std::os::raw::c_ulong as z_crc_t,
      0x6ae96943 as std::os::raw::c_ulong as z_crc_t,
      0xfcd96e34 as std::os::raw::c_ulong as z_crc_t,
      0x468867ad as std::os::raw::c_ulong as z_crc_t,
      0xd0b860da as std::os::raw::c_ulong as z_crc_t,
      0x732d0444 as std::os::raw::c_ulong as z_crc_t,
      0xe51d0333 as std::os::raw::c_ulong as z_crc_t,
      0x5f4c0aaa as std::os::raw::c_ulong as z_crc_t,
      0xc97c0ddd as std::os::raw::c_ulong as z_crc_t,
      0x3c710550 as std::os::raw::c_ulong as z_crc_t,
      0xaa410227 as std::os::raw::c_ulong as z_crc_t,
      0x10100bbe as std::os::raw::c_ulong as z_crc_t,
      0x86200cc9 as std::os::raw::c_ulong as z_crc_t,
      0x25b56857 as std::os::raw::c_ulong as z_crc_t,
      0xb3856f20 as std::os::raw::c_ulong as z_crc_t,
      0x9d466b9 as std::os::raw::c_ulong as z_crc_t,
      0x9fe461ce as std::os::raw::c_ulong as z_crc_t,
      0xef9de5e as std::os::raw::c_ulong as z_crc_t,
      0x98c9d929 as std::os::raw::c_ulong as z_crc_t,
      0x2298d0b0 as std::os::raw::c_ulong as z_crc_t,
      0xb4a8d7c7 as std::os::raw::c_ulong as z_crc_t,
      0x173db359 as std::os::raw::c_ulong as z_crc_t,
      0x810db42e as std::os::raw::c_ulong as z_crc_t,
      0x3b5cbdb7 as std::os::raw::c_ulong as z_crc_t,
      0xad6cbac0 as std::os::raw::c_ulong as z_crc_t,
      0x2083b8ed as std::os::raw::c_ulong as z_crc_t,
      0xb6b3bf9a as std::os::raw::c_ulong as z_crc_t,
      0xce2b603 as std::os::raw::c_ulong as z_crc_t,
      0x9ad2b174 as std::os::raw::c_ulong as z_crc_t,
      0x3947d5ea as std::os::raw::c_ulong as z_crc_t,
      0xaf77d29d as std::os::raw::c_ulong as z_crc_t,
      0x1526db04 as std::os::raw::c_ulong as z_crc_t,
      0x8316dc73 as std::os::raw::c_ulong as z_crc_t,
      0x120b63e3 as std::os::raw::c_ulong as z_crc_t,
      0x843b6494 as std::os::raw::c_ulong as z_crc_t,
      0x3e6a6d0d as std::os::raw::c_ulong as z_crc_t,
      0xa85a6a7a as std::os::raw::c_ulong as z_crc_t,
      0xbcf0ee4 as std::os::raw::c_ulong as z_crc_t,
      0x9dff0993 as std::os::raw::c_ulong as z_crc_t,
      0x27ae000a as std::os::raw::c_ulong as z_crc_t,
      0xb19e077d as std::os::raw::c_ulong as z_crc_t,
      0x44930ff0 as std::os::raw::c_ulong as z_crc_t,
      0xd2a30887 as std::os::raw::c_ulong as z_crc_t,
      0x68f2011e as std::os::raw::c_ulong as z_crc_t,
      0xfec20669 as std::os::raw::c_ulong as z_crc_t,
      0x5d5762f7 as std::os::raw::c_ulong as z_crc_t,
      0xcb676580 as std::os::raw::c_ulong as z_crc_t,
      0x71366c19 as std::os::raw::c_ulong as z_crc_t,
      0xe7066b6e as std::os::raw::c_ulong as z_crc_t,
      0x761bd4fe as std::os::raw::c_ulong as z_crc_t,
      0xe02bd389 as std::os::raw::c_ulong as z_crc_t,
      0x5a7ada10 as std::os::raw::c_ulong as z_crc_t,
      0xcc4add67 as std::os::raw::c_ulong as z_crc_t,
      0x6fdfb9f9 as std::os::raw::c_ulong as z_crc_t,
      0xf9efbe8e as std::os::raw::c_ulong as z_crc_t,
      0x43beb717 as std::os::raw::c_ulong as z_crc_t,
      0xd58eb060 as std::os::raw::c_ulong as z_crc_t,
      0xe8a3d6d6 as std::os::raw::c_ulong as z_crc_t,
      0x7e93d1a1 as std::os::raw::c_ulong as z_crc_t,
      0xc4c2d838 as std::os::raw::c_ulong as z_crc_t,
      0x52f2df4f as std::os::raw::c_ulong as z_crc_t,
      0xf167bbd1 as std::os::raw::c_ulong as z_crc_t,
      0x6757bca6 as std::os::raw::c_ulong as z_crc_t,
      0xdd06b53f as std::os::raw::c_ulong as z_crc_t,
      0x4b36b248 as std::os::raw::c_ulong as z_crc_t,
      0xda2b0dd8 as std::os::raw::c_ulong as z_crc_t,
      0x4c1b0aaf as std::os::raw::c_ulong as z_crc_t,
      0xf64a0336 as std::os::raw::c_ulong as z_crc_t,
      0x607a0441 as std::os::raw::c_ulong as z_crc_t,
      0xc3ef60df as std::os::raw::c_ulong as z_crc_t,
      0x55df67a8 as std::os::raw::c_ulong as z_crc_t,
      0xef8e6e31 as std::os::raw::c_ulong as z_crc_t,
      0x79be6946 as std::os::raw::c_ulong as z_crc_t,
      0x8cb361cb as std::os::raw::c_ulong as z_crc_t,
      0x1a8366bc as std::os::raw::c_ulong as z_crc_t,
      0xa0d26f25 as std::os::raw::c_ulong as z_crc_t,
      0x36e26852 as std::os::raw::c_ulong as z_crc_t,
      0x95770ccc as std::os::raw::c_ulong as z_crc_t,
      0x3470bbb as std::os::raw::c_ulong as z_crc_t,
      0xb9160222 as std::os::raw::c_ulong as z_crc_t,
      0x2f260555 as std::os::raw::c_ulong as z_crc_t,
      0xbe3bbac5 as std::os::raw::c_ulong as z_crc_t,
      0x280bbdb2 as std::os::raw::c_ulong as z_crc_t,
      0x925ab42b as std::os::raw::c_ulong as z_crc_t,
      0x46ab35c as std::os::raw::c_ulong as z_crc_t,
      0xa7ffd7c2 as std::os::raw::c_ulong as z_crc_t,
      0x31cfd0b5 as std::os::raw::c_ulong as z_crc_t,
      0x8b9ed92c as std::os::raw::c_ulong as z_crc_t,
      0x1daede5b as std::os::raw::c_ulong as z_crc_t,
      0xb0c2649b as std::os::raw::c_ulong as z_crc_t,
      0x26f263ec as std::os::raw::c_ulong as z_crc_t,
      0x9ca36a75 as std::os::raw::c_ulong as z_crc_t,
      0xa936d02 as std::os::raw::c_ulong as z_crc_t,
      0xa906099c as std::os::raw::c_ulong as z_crc_t,
      0x3f360eeb as std::os::raw::c_ulong as z_crc_t,
      0x85670772 as std::os::raw::c_ulong as z_crc_t,
      0x13570005 as std::os::raw::c_ulong as z_crc_t,
      0x824abf95 as std::os::raw::c_ulong as z_crc_t,
      0x147ab8e2 as std::os::raw::c_ulong as z_crc_t,
      0xae2bb17b as std::os::raw::c_ulong as z_crc_t,
      0x381bb60c as std::os::raw::c_ulong as z_crc_t,
      0x9b8ed292 as std::os::raw::c_ulong as z_crc_t,
      0xdbed5e5 as std::os::raw::c_ulong as z_crc_t,
      0xb7efdc7c as std::os::raw::c_ulong as z_crc_t,
      0x21dfdb0b as std::os::raw::c_ulong as z_crc_t,
      0xd4d2d386 as std::os::raw::c_ulong as z_crc_t,
      0x42e2d4f1 as std::os::raw::c_ulong as z_crc_t,
      0xf8b3dd68 as std::os::raw::c_ulong as z_crc_t,
      0x6e83da1f as std::os::raw::c_ulong as z_crc_t,
      0xcd16be81 as std::os::raw::c_ulong as z_crc_t,
      0x5b26b9f6 as std::os::raw::c_ulong as z_crc_t,
      0xe177b06f as std::os::raw::c_ulong as z_crc_t,
      0x7747b718 as std::os::raw::c_ulong as z_crc_t,
      0xe65a0888 as std::os::raw::c_ulong as z_crc_t,
      0x706a0fff as std::os::raw::c_ulong as z_crc_t,
      0xca3b0666 as std::os::raw::c_ulong as z_crc_t,
      0x5c0b0111 as std::os::raw::c_ulong as z_crc_t,
      0xff9e658f as std::os::raw::c_ulong as z_crc_t,
      0x69ae62f8 as std::os::raw::c_ulong as z_crc_t,
      0xd3ff6b61 as std::os::raw::c_ulong as z_crc_t,
      0x45cf6c16 as std::os::raw::c_ulong as z_crc_t,
      0x78e20aa0 as std::os::raw::c_ulong as z_crc_t,
      0xeed20dd7 as std::os::raw::c_ulong as z_crc_t,
      0x5483044e as std::os::raw::c_ulong as z_crc_t,
      0xc2b30339 as std::os::raw::c_ulong as z_crc_t,
      0x612667a7 as std::os::raw::c_ulong as z_crc_t,
      0xf71660d0 as std::os::raw::c_ulong as z_crc_t,
      0x4d476949 as std::os::raw::c_ulong as z_crc_t,
      0xdb776e3e as std::os::raw::c_ulong as z_crc_t,
      0x4a6ad1ae as std::os::raw::c_ulong as z_crc_t,
      0xdc5ad6d9 as std::os::raw::c_ulong as z_crc_t,
      0x660bdf40 as std::os::raw::c_ulong as z_crc_t,
      0xf03bd837 as std::os::raw::c_ulong as z_crc_t,
      0x53aebca9 as std::os::raw::c_ulong as z_crc_t,
      0xc59ebbde as std::os::raw::c_ulong as z_crc_t,
      0x7fcfb247 as std::os::raw::c_ulong as z_crc_t,
      0xe9ffb530 as std::os::raw::c_ulong as z_crc_t,
      0x1cf2bdbd as std::os::raw::c_ulong as z_crc_t,
      0x8ac2baca as std::os::raw::c_ulong as z_crc_t,
      0x3093b353 as std::os::raw::c_ulong as z_crc_t,
      0xa6a3b424 as std::os::raw::c_ulong as z_crc_t,
      0x536d0ba as std::os::raw::c_ulong as z_crc_t,
      0x9306d7cd as std::os::raw::c_ulong as z_crc_t,
      0x2957de54 as std::os::raw::c_ulong as z_crc_t,
      0xbf67d923 as std::os::raw::c_ulong as z_crc_t,
      0x2e7a66b3 as std::os::raw::c_ulong as z_crc_t,
      0xb84a61c4 as std::os::raw::c_ulong as z_crc_t,
      0x21b685d as std::os::raw::c_ulong as z_crc_t,
      0x942b6f2a as std::os::raw::c_ulong as z_crc_t,
      0x37be0bb4 as std::os::raw::c_ulong as z_crc_t,
      0xa18e0cc3 as std::os::raw::c_ulong as z_crc_t,
      0x1bdf055a as std::os::raw::c_ulong as z_crc_t,
      0x8def022d as std::os::raw::c_ulong as z_crc_t],
     [0 as std::os::raw::c_ulong as z_crc_t, 0x41311b19 as std::os::raw::c_ulong as z_crc_t,
      0x82623632 as std::os::raw::c_ulong as z_crc_t,
      0xc3532d2b as std::os::raw::c_ulong as z_crc_t,
      0x4c56c64 as std::os::raw::c_ulong as z_crc_t,
      0x45f4777d as std::os::raw::c_ulong as z_crc_t,
      0x86a75a56 as std::os::raw::c_ulong as z_crc_t,
      0xc796414f as std::os::raw::c_ulong as z_crc_t,
      0x88ad9c8 as std::os::raw::c_ulong as z_crc_t,
      0x49bbc2d1 as std::os::raw::c_ulong as z_crc_t,
      0x8ae8effa as std::os::raw::c_ulong as z_crc_t,
      0xcbd9f4e3 as std::os::raw::c_ulong as z_crc_t,
      0xc4fb5ac as std::os::raw::c_ulong as z_crc_t,
      0x4d7eaeb5 as std::os::raw::c_ulong as z_crc_t,
      0x8e2d839e as std::os::raw::c_ulong as z_crc_t,
      0xcf1c9887 as std::os::raw::c_ulong as z_crc_t,
      0x5112c24a as std::os::raw::c_ulong as z_crc_t,
      0x1023d953 as std::os::raw::c_ulong as z_crc_t,
      0xd370f478 as std::os::raw::c_ulong as z_crc_t,
      0x9241ef61 as std::os::raw::c_ulong as z_crc_t,
      0x55d7ae2e as std::os::raw::c_ulong as z_crc_t,
      0x14e6b537 as std::os::raw::c_ulong as z_crc_t,
      0xd7b5981c as std::os::raw::c_ulong as z_crc_t,
      0x96848305 as std::os::raw::c_ulong as z_crc_t,
      0x59981b82 as std::os::raw::c_ulong as z_crc_t,
      0x18a9009b as std::os::raw::c_ulong as z_crc_t,
      0xdbfa2db0 as std::os::raw::c_ulong as z_crc_t,
      0x9acb36a9 as std::os::raw::c_ulong as z_crc_t,
      0x5d5d77e6 as std::os::raw::c_ulong as z_crc_t,
      0x1c6c6cff as std::os::raw::c_ulong as z_crc_t,
      0xdf3f41d4 as std::os::raw::c_ulong as z_crc_t,
      0x9e0e5acd as std::os::raw::c_ulong as z_crc_t,
      0xa2248495 as std::os::raw::c_ulong as z_crc_t,
      0xe3159f8c as std::os::raw::c_ulong as z_crc_t,
      0x2046b2a7 as std::os::raw::c_ulong as z_crc_t,
      0x6177a9be as std::os::raw::c_ulong as z_crc_t,
      0xa6e1e8f1 as std::os::raw::c_ulong as z_crc_t,
      0xe7d0f3e8 as std::os::raw::c_ulong as z_crc_t,
      0x2483dec3 as std::os::raw::c_ulong as z_crc_t,
      0x65b2c5da as std::os::raw::c_ulong as z_crc_t,
      0xaaae5d5d as std::os::raw::c_ulong as z_crc_t,
      0xeb9f4644 as std::os::raw::c_ulong as z_crc_t,
      0x28cc6b6f as std::os::raw::c_ulong as z_crc_t,
      0x69fd7076 as std::os::raw::c_ulong as z_crc_t,
      0xae6b3139 as std::os::raw::c_ulong as z_crc_t,
      0xef5a2a20 as std::os::raw::c_ulong as z_crc_t,
      0x2c09070b as std::os::raw::c_ulong as z_crc_t,
      0x6d381c12 as std::os::raw::c_ulong as z_crc_t,
      0xf33646df as std::os::raw::c_ulong as z_crc_t,
      0xb2075dc6 as std::os::raw::c_ulong as z_crc_t,
      0x715470ed as std::os::raw::c_ulong as z_crc_t,
      0x30656bf4 as std::os::raw::c_ulong as z_crc_t,
      0xf7f32abb as std::os::raw::c_ulong as z_crc_t,
      0xb6c231a2 as std::os::raw::c_ulong as z_crc_t,
      0x75911c89 as std::os::raw::c_ulong as z_crc_t,
      0x34a00790 as std::os::raw::c_ulong as z_crc_t,
      0xfbbc9f17 as std::os::raw::c_ulong as z_crc_t,
      0xba8d840e as std::os::raw::c_ulong as z_crc_t,
      0x79dea925 as std::os::raw::c_ulong as z_crc_t,
      0x38efb23c as std::os::raw::c_ulong as z_crc_t,
      0xff79f373 as std::os::raw::c_ulong as z_crc_t,
      0xbe48e86a as std::os::raw::c_ulong as z_crc_t,
      0x7d1bc541 as std::os::raw::c_ulong as z_crc_t,
      0x3c2ade58 as std::os::raw::c_ulong as z_crc_t,
      0x54f79f0 as std::os::raw::c_ulong as z_crc_t,
      0x447e62e9 as std::os::raw::c_ulong as z_crc_t,
      0x872d4fc2 as std::os::raw::c_ulong as z_crc_t,
      0xc61c54db as std::os::raw::c_ulong as z_crc_t,
      0x18a1594 as std::os::raw::c_ulong as z_crc_t,
      0x40bb0e8d as std::os::raw::c_ulong as z_crc_t,
      0x83e823a6 as std::os::raw::c_ulong as z_crc_t,
      0xc2d938bf as std::os::raw::c_ulong as z_crc_t,
      0xdc5a038 as std::os::raw::c_ulong as z_crc_t,
      0x4cf4bb21 as std::os::raw::c_ulong as z_crc_t,
      0x8fa7960a as std::os::raw::c_ulong as z_crc_t,
      0xce968d13 as std::os::raw::c_ulong as z_crc_t,
      0x900cc5c as std::os::raw::c_ulong as z_crc_t,
      0x4831d745 as std::os::raw::c_ulong as z_crc_t,
      0x8b62fa6e as std::os::raw::c_ulong as z_crc_t,
      0xca53e177 as std::os::raw::c_ulong as z_crc_t,
      0x545dbbba as std::os::raw::c_ulong as z_crc_t,
      0x156ca0a3 as std::os::raw::c_ulong as z_crc_t,
      0xd63f8d88 as std::os::raw::c_ulong as z_crc_t,
      0x970e9691 as std::os::raw::c_ulong as z_crc_t,
      0x5098d7de as std::os::raw::c_ulong as z_crc_t,
      0x11a9ccc7 as std::os::raw::c_ulong as z_crc_t,
      0xd2fae1ec as std::os::raw::c_ulong as z_crc_t,
      0x93cbfaf5 as std::os::raw::c_ulong as z_crc_t,
      0x5cd76272 as std::os::raw::c_ulong as z_crc_t,
      0x1de6796b as std::os::raw::c_ulong as z_crc_t,
      0xdeb55440 as std::os::raw::c_ulong as z_crc_t,
      0x9f844f59 as std::os::raw::c_ulong as z_crc_t,
      0x58120e16 as std::os::raw::c_ulong as z_crc_t,
      0x1923150f as std::os::raw::c_ulong as z_crc_t,
      0xda703824 as std::os::raw::c_ulong as z_crc_t,
      0x9b41233d as std::os::raw::c_ulong as z_crc_t,
      0xa76bfd65 as std::os::raw::c_ulong as z_crc_t,
      0xe65ae67c as std::os::raw::c_ulong as z_crc_t,
      0x2509cb57 as std::os::raw::c_ulong as z_crc_t,
      0x6438d04e as std::os::raw::c_ulong as z_crc_t,
      0xa3ae9101 as std::os::raw::c_ulong as z_crc_t,
      0xe29f8a18 as std::os::raw::c_ulong as z_crc_t,
      0x21cca733 as std::os::raw::c_ulong as z_crc_t,
      0x60fdbc2a as std::os::raw::c_ulong as z_crc_t,
      0xafe124ad as std::os::raw::c_ulong as z_crc_t,
      0xeed03fb4 as std::os::raw::c_ulong as z_crc_t,
      0x2d83129f as std::os::raw::c_ulong as z_crc_t,
      0x6cb20986 as std::os::raw::c_ulong as z_crc_t,
      0xab2448c9 as std::os::raw::c_ulong as z_crc_t,
      0xea1553d0 as std::os::raw::c_ulong as z_crc_t,
      0x29467efb as std::os::raw::c_ulong as z_crc_t,
      0x687765e2 as std::os::raw::c_ulong as z_crc_t,
      0xf6793f2f as std::os::raw::c_ulong as z_crc_t,
      0xb7482436 as std::os::raw::c_ulong as z_crc_t,
      0x741b091d as std::os::raw::c_ulong as z_crc_t,
      0x352a1204 as std::os::raw::c_ulong as z_crc_t,
      0xf2bc534b as std::os::raw::c_ulong as z_crc_t,
      0xb38d4852 as std::os::raw::c_ulong as z_crc_t,
      0x70de6579 as std::os::raw::c_ulong as z_crc_t,
      0x31ef7e60 as std::os::raw::c_ulong as z_crc_t,
      0xfef3e6e7 as std::os::raw::c_ulong as z_crc_t,
      0xbfc2fdfe as std::os::raw::c_ulong as z_crc_t,
      0x7c91d0d5 as std::os::raw::c_ulong as z_crc_t,
      0x3da0cbcc as std::os::raw::c_ulong as z_crc_t,
      0xfa368a83 as std::os::raw::c_ulong as z_crc_t,
      0xbb07919a as std::os::raw::c_ulong as z_crc_t,
      0x7854bcb1 as std::os::raw::c_ulong as z_crc_t,
      0x3965a7a8 as std::os::raw::c_ulong as z_crc_t,
      0x4b98833b as std::os::raw::c_ulong as z_crc_t,
      0xaa99822 as std::os::raw::c_ulong as z_crc_t,
      0xc9fab509 as std::os::raw::c_ulong as z_crc_t,
      0x88cbae10 as std::os::raw::c_ulong as z_crc_t,
      0x4f5def5f as std::os::raw::c_ulong as z_crc_t,
      0xe6cf446 as std::os::raw::c_ulong as z_crc_t,
      0xcd3fd96d as std::os::raw::c_ulong as z_crc_t,
      0x8c0ec274 as std::os::raw::c_ulong as z_crc_t,
      0x43125af3 as std::os::raw::c_ulong as z_crc_t,
      0x22341ea as std::os::raw::c_ulong as z_crc_t,
      0xc1706cc1 as std::os::raw::c_ulong as z_crc_t,
      0x804177d8 as std::os::raw::c_ulong as z_crc_t,
      0x47d73697 as std::os::raw::c_ulong as z_crc_t,
      0x6e62d8e as std::os::raw::c_ulong as z_crc_t,
      0xc5b500a5 as std::os::raw::c_ulong as z_crc_t,
      0x84841bbc as std::os::raw::c_ulong as z_crc_t,
      0x1a8a4171 as std::os::raw::c_ulong as z_crc_t,
      0x5bbb5a68 as std::os::raw::c_ulong as z_crc_t,
      0x98e87743 as std::os::raw::c_ulong as z_crc_t,
      0xd9d96c5a as std::os::raw::c_ulong as z_crc_t,
      0x1e4f2d15 as std::os::raw::c_ulong as z_crc_t,
      0x5f7e360c as std::os::raw::c_ulong as z_crc_t,
      0x9c2d1b27 as std::os::raw::c_ulong as z_crc_t,
      0xdd1c003e as std::os::raw::c_ulong as z_crc_t,
      0x120098b9 as std::os::raw::c_ulong as z_crc_t,
      0x533183a0 as std::os::raw::c_ulong as z_crc_t,
      0x9062ae8b as std::os::raw::c_ulong as z_crc_t,
      0xd153b592 as std::os::raw::c_ulong as z_crc_t,
      0x16c5f4dd as std::os::raw::c_ulong as z_crc_t,
      0x57f4efc4 as std::os::raw::c_ulong as z_crc_t,
      0x94a7c2ef as std::os::raw::c_ulong as z_crc_t,
      0xd596d9f6 as std::os::raw::c_ulong as z_crc_t,
      0xe9bc07ae as std::os::raw::c_ulong as z_crc_t,
      0xa88d1cb7 as std::os::raw::c_ulong as z_crc_t,
      0x6bde319c as std::os::raw::c_ulong as z_crc_t,
      0x2aef2a85 as std::os::raw::c_ulong as z_crc_t,
      0xed796bca as std::os::raw::c_ulong as z_crc_t,
      0xac4870d3 as std::os::raw::c_ulong as z_crc_t,
      0x6f1b5df8 as std::os::raw::c_ulong as z_crc_t,
      0x2e2a46e1 as std::os::raw::c_ulong as z_crc_t,
      0xe136de66 as std::os::raw::c_ulong as z_crc_t,
      0xa007c57f as std::os::raw::c_ulong as z_crc_t,
      0x6354e854 as std::os::raw::c_ulong as z_crc_t,
      0x2265f34d as std::os::raw::c_ulong as z_crc_t,
      0xe5f3b202 as std::os::raw::c_ulong as z_crc_t,
      0xa4c2a91b as std::os::raw::c_ulong as z_crc_t,
      0x67918430 as std::os::raw::c_ulong as z_crc_t,
      0x26a09f29 as std::os::raw::c_ulong as z_crc_t,
      0xb8aec5e4 as std::os::raw::c_ulong as z_crc_t,
      0xf99fdefd as std::os::raw::c_ulong as z_crc_t,
      0x3accf3d6 as std::os::raw::c_ulong as z_crc_t,
      0x7bfde8cf as std::os::raw::c_ulong as z_crc_t,
      0xbc6ba980 as std::os::raw::c_ulong as z_crc_t,
      0xfd5ab299 as std::os::raw::c_ulong as z_crc_t,
      0x3e099fb2 as std::os::raw::c_ulong as z_crc_t,
      0x7f3884ab as std::os::raw::c_ulong as z_crc_t,
      0xb0241c2c as std::os::raw::c_ulong as z_crc_t,
      0xf1150735 as std::os::raw::c_ulong as z_crc_t,
      0x32462a1e as std::os::raw::c_ulong as z_crc_t,
      0x73773107 as std::os::raw::c_ulong as z_crc_t,
      0xb4e17048 as std::os::raw::c_ulong as z_crc_t,
      0xf5d06b51 as std::os::raw::c_ulong as z_crc_t,
      0x3683467a as std::os::raw::c_ulong as z_crc_t,
      0x77b25d63 as std::os::raw::c_ulong as z_crc_t,
      0x4ed7facb as std::os::raw::c_ulong as z_crc_t,
      0xfe6e1d2 as std::os::raw::c_ulong as z_crc_t,
      0xccb5ccf9 as std::os::raw::c_ulong as z_crc_t,
      0x8d84d7e0 as std::os::raw::c_ulong as z_crc_t,
      0x4a1296af as std::os::raw::c_ulong as z_crc_t,
      0xb238db6 as std::os::raw::c_ulong as z_crc_t,
      0xc870a09d as std::os::raw::c_ulong as z_crc_t,
      0x8941bb84 as std::os::raw::c_ulong as z_crc_t,
      0x465d2303 as std::os::raw::c_ulong as z_crc_t,
      0x76c381a as std::os::raw::c_ulong as z_crc_t,
      0xc43f1531 as std::os::raw::c_ulong as z_crc_t,
      0x850e0e28 as std::os::raw::c_ulong as z_crc_t,
      0x42984f67 as std::os::raw::c_ulong as z_crc_t,
      0x3a9547e as std::os::raw::c_ulong as z_crc_t,
      0xc0fa7955 as std::os::raw::c_ulong as z_crc_t,
      0x81cb624c as std::os::raw::c_ulong as z_crc_t,
      0x1fc53881 as std::os::raw::c_ulong as z_crc_t,
      0x5ef42398 as std::os::raw::c_ulong as z_crc_t,
      0x9da70eb3 as std::os::raw::c_ulong as z_crc_t,
      0xdc9615aa as std::os::raw::c_ulong as z_crc_t,
      0x1b0054e5 as std::os::raw::c_ulong as z_crc_t,
      0x5a314ffc as std::os::raw::c_ulong as z_crc_t,
      0x996262d7 as std::os::raw::c_ulong as z_crc_t,
      0xd85379ce as std::os::raw::c_ulong as z_crc_t,
      0x174fe149 as std::os::raw::c_ulong as z_crc_t,
      0x567efa50 as std::os::raw::c_ulong as z_crc_t,
      0x952dd77b as std::os::raw::c_ulong as z_crc_t,
      0xd41ccc62 as std::os::raw::c_ulong as z_crc_t,
      0x138a8d2d as std::os::raw::c_ulong as z_crc_t,
      0x52bb9634 as std::os::raw::c_ulong as z_crc_t,
      0x91e8bb1f as std::os::raw::c_ulong as z_crc_t,
      0xd0d9a006 as std::os::raw::c_ulong as z_crc_t,
      0xecf37e5e as std::os::raw::c_ulong as z_crc_t,
      0xadc26547 as std::os::raw::c_ulong as z_crc_t,
      0x6e91486c as std::os::raw::c_ulong as z_crc_t,
      0x2fa05375 as std::os::raw::c_ulong as z_crc_t,
      0xe836123a as std::os::raw::c_ulong as z_crc_t,
      0xa9070923 as std::os::raw::c_ulong as z_crc_t,
      0x6a542408 as std::os::raw::c_ulong as z_crc_t,
      0x2b653f11 as std::os::raw::c_ulong as z_crc_t,
      0xe479a796 as std::os::raw::c_ulong as z_crc_t,
      0xa548bc8f as std::os::raw::c_ulong as z_crc_t,
      0x661b91a4 as std::os::raw::c_ulong as z_crc_t,
      0x272a8abd as std::os::raw::c_ulong as z_crc_t,
      0xe0bccbf2 as std::os::raw::c_ulong as z_crc_t,
      0xa18dd0eb as std::os::raw::c_ulong as z_crc_t,
      0x62defdc0 as std::os::raw::c_ulong as z_crc_t,
      0x23efe6d9 as std::os::raw::c_ulong as z_crc_t,
      0xbde1bc14 as std::os::raw::c_ulong as z_crc_t,
      0xfcd0a70d as std::os::raw::c_ulong as z_crc_t,
      0x3f838a26 as std::os::raw::c_ulong as z_crc_t,
      0x7eb2913f as std::os::raw::c_ulong as z_crc_t,
      0xb924d070 as std::os::raw::c_ulong as z_crc_t,
      0xf815cb69 as std::os::raw::c_ulong as z_crc_t,
      0x3b46e642 as std::os::raw::c_ulong as z_crc_t,
      0x7a77fd5b as std::os::raw::c_ulong as z_crc_t,
      0xb56b65dc as std::os::raw::c_ulong as z_crc_t,
      0xf45a7ec5 as std::os::raw::c_ulong as z_crc_t,
      0x370953ee as std::os::raw::c_ulong as z_crc_t,
      0x763848f7 as std::os::raw::c_ulong as z_crc_t,
      0xb1ae09b8 as std::os::raw::c_ulong as z_crc_t,
      0xf09f12a1 as std::os::raw::c_ulong as z_crc_t,
      0x33cc3f8a as std::os::raw::c_ulong as z_crc_t,
      0x72fd2493 as std::os::raw::c_ulong as z_crc_t],
     [0 as std::os::raw::c_ulong as z_crc_t, 0x376ac201 as std::os::raw::c_ulong as z_crc_t,
      0x6ed48403 as std::os::raw::c_ulong as z_crc_t,
      0x59be4602 as std::os::raw::c_ulong as z_crc_t,
      0xdca80907 as std::os::raw::c_ulong as z_crc_t,
      0xebc2cb06 as std::os::raw::c_ulong as z_crc_t,
      0xb27c8d04 as std::os::raw::c_ulong as z_crc_t,
      0x85164f05 as std::os::raw::c_ulong as z_crc_t,
      0xb851130e as std::os::raw::c_ulong as z_crc_t,
      0x8f3bd10f as std::os::raw::c_ulong as z_crc_t,
      0xd685970d as std::os::raw::c_ulong as z_crc_t,
      0xe1ef550c as std::os::raw::c_ulong as z_crc_t,
      0x64f91a09 as std::os::raw::c_ulong as z_crc_t,
      0x5393d808 as std::os::raw::c_ulong as z_crc_t,
      0xa2d9e0a as std::os::raw::c_ulong as z_crc_t,
      0x3d475c0b as std::os::raw::c_ulong as z_crc_t,
      0x70a3261c as std::os::raw::c_ulong as z_crc_t,
      0x47c9e41d as std::os::raw::c_ulong as z_crc_t,
      0x1e77a21f as std::os::raw::c_ulong as z_crc_t,
      0x291d601e as std::os::raw::c_ulong as z_crc_t,
      0xac0b2f1b as std::os::raw::c_ulong as z_crc_t,
      0x9b61ed1a as std::os::raw::c_ulong as z_crc_t,
      0xc2dfab18 as std::os::raw::c_ulong as z_crc_t,
      0xf5b56919 as std::os::raw::c_ulong as z_crc_t,
      0xc8f23512 as std::os::raw::c_ulong as z_crc_t,
      0xff98f713 as std::os::raw::c_ulong as z_crc_t,
      0xa626b111 as std::os::raw::c_ulong as z_crc_t,
      0x914c7310 as std::os::raw::c_ulong as z_crc_t,
      0x145a3c15 as std::os::raw::c_ulong as z_crc_t,
      0x2330fe14 as std::os::raw::c_ulong as z_crc_t,
      0x7a8eb816 as std::os::raw::c_ulong as z_crc_t,
      0x4de47a17 as std::os::raw::c_ulong as z_crc_t,
      0xe0464d38 as std::os::raw::c_ulong as z_crc_t,
      0xd72c8f39 as std::os::raw::c_ulong as z_crc_t,
      0x8e92c93b as std::os::raw::c_ulong as z_crc_t,
      0xb9f80b3a as std::os::raw::c_ulong as z_crc_t,
      0x3cee443f as std::os::raw::c_ulong as z_crc_t,
      0xb84863e as std::os::raw::c_ulong as z_crc_t,
      0x523ac03c as std::os::raw::c_ulong as z_crc_t,
      0x6550023d as std::os::raw::c_ulong as z_crc_t,
      0x58175e36 as std::os::raw::c_ulong as z_crc_t,
      0x6f7d9c37 as std::os::raw::c_ulong as z_crc_t,
      0x36c3da35 as std::os::raw::c_ulong as z_crc_t,
      0x1a91834 as std::os::raw::c_ulong as z_crc_t,
      0x84bf5731 as std::os::raw::c_ulong as z_crc_t,
      0xb3d59530 as std::os::raw::c_ulong as z_crc_t,
      0xea6bd332 as std::os::raw::c_ulong as z_crc_t,
      0xdd011133 as std::os::raw::c_ulong as z_crc_t,
      0x90e56b24 as std::os::raw::c_ulong as z_crc_t,
      0xa78fa925 as std::os::raw::c_ulong as z_crc_t,
      0xfe31ef27 as std::os::raw::c_ulong as z_crc_t,
      0xc95b2d26 as std::os::raw::c_ulong as z_crc_t,
      0x4c4d6223 as std::os::raw::c_ulong as z_crc_t,
      0x7b27a022 as std::os::raw::c_ulong as z_crc_t,
      0x2299e620 as std::os::raw::c_ulong as z_crc_t,
      0x15f32421 as std::os::raw::c_ulong as z_crc_t,
      0x28b4782a as std::os::raw::c_ulong as z_crc_t,
      0x1fdeba2b as std::os::raw::c_ulong as z_crc_t,
      0x4660fc29 as std::os::raw::c_ulong as z_crc_t,
      0x710a3e28 as std::os::raw::c_ulong as z_crc_t,
      0xf41c712d as std::os::raw::c_ulong as z_crc_t,
      0xc376b32c as std::os::raw::c_ulong as z_crc_t,
      0x9ac8f52e as std::os::raw::c_ulong as z_crc_t,
      0xada2372f as std::os::raw::c_ulong as z_crc_t,
      0xc08d9a70 as std::os::raw::c_ulong as z_crc_t,
      0xf7e75871 as std::os::raw::c_ulong as z_crc_t,
      0xae591e73 as std::os::raw::c_ulong as z_crc_t,
      0x9933dc72 as std::os::raw::c_ulong as z_crc_t,
      0x1c259377 as std::os::raw::c_ulong as z_crc_t,
      0x2b4f5176 as std::os::raw::c_ulong as z_crc_t,
      0x72f11774 as std::os::raw::c_ulong as z_crc_t,
      0x459bd575 as std::os::raw::c_ulong as z_crc_t,
      0x78dc897e as std::os::raw::c_ulong as z_crc_t,
      0x4fb64b7f as std::os::raw::c_ulong as z_crc_t,
      0x16080d7d as std::os::raw::c_ulong as z_crc_t,
      0x2162cf7c as std::os::raw::c_ulong as z_crc_t,
      0xa4748079 as std::os::raw::c_ulong as z_crc_t,
      0x931e4278 as std::os::raw::c_ulong as z_crc_t,
      0xcaa0047a as std::os::raw::c_ulong as z_crc_t,
      0xfdcac67b as std::os::raw::c_ulong as z_crc_t,
      0xb02ebc6c as std::os::raw::c_ulong as z_crc_t,
      0x87447e6d as std::os::raw::c_ulong as z_crc_t,
      0xdefa386f as std::os::raw::c_ulong as z_crc_t,
      0xe990fa6e as std::os::raw::c_ulong as z_crc_t,
      0x6c86b56b as std::os::raw::c_ulong as z_crc_t,
      0x5bec776a as std::os::raw::c_ulong as z_crc_t,
      0x2523168 as std::os::raw::c_ulong as z_crc_t,
      0x3538f369 as std::os::raw::c_ulong as z_crc_t,
      0x87faf62 as std::os::raw::c_ulong as z_crc_t,
      0x3f156d63 as std::os::raw::c_ulong as z_crc_t,
      0x66ab2b61 as std::os::raw::c_ulong as z_crc_t,
      0x51c1e960 as std::os::raw::c_ulong as z_crc_t,
      0xd4d7a665 as std::os::raw::c_ulong as z_crc_t,
      0xe3bd6464 as std::os::raw::c_ulong as z_crc_t,
      0xba032266 as std::os::raw::c_ulong as z_crc_t,
      0x8d69e067 as std::os::raw::c_ulong as z_crc_t,
      0x20cbd748 as std::os::raw::c_ulong as z_crc_t,
      0x17a11549 as std::os::raw::c_ulong as z_crc_t,
      0x4e1f534b as std::os::raw::c_ulong as z_crc_t,
      0x7975914a as std::os::raw::c_ulong as z_crc_t,
      0xfc63de4f as std::os::raw::c_ulong as z_crc_t,
      0xcb091c4e as std::os::raw::c_ulong as z_crc_t,
      0x92b75a4c as std::os::raw::c_ulong as z_crc_t,
      0xa5dd984d as std::os::raw::c_ulong as z_crc_t,
      0x989ac446 as std::os::raw::c_ulong as z_crc_t,
      0xaff00647 as std::os::raw::c_ulong as z_crc_t,
      0xf64e4045 as std::os::raw::c_ulong as z_crc_t,
      0xc1248244 as std::os::raw::c_ulong as z_crc_t,
      0x4432cd41 as std::os::raw::c_ulong as z_crc_t,
      0x73580f40 as std::os::raw::c_ulong as z_crc_t,
      0x2ae64942 as std::os::raw::c_ulong as z_crc_t,
      0x1d8c8b43 as std::os::raw::c_ulong as z_crc_t,
      0x5068f154 as std::os::raw::c_ulong as z_crc_t,
      0x67023355 as std::os::raw::c_ulong as z_crc_t,
      0x3ebc7557 as std::os::raw::c_ulong as z_crc_t,
      0x9d6b756 as std::os::raw::c_ulong as z_crc_t,
      0x8cc0f853 as std::os::raw::c_ulong as z_crc_t,
      0xbbaa3a52 as std::os::raw::c_ulong as z_crc_t,
      0xe2147c50 as std::os::raw::c_ulong as z_crc_t,
      0xd57ebe51 as std::os::raw::c_ulong as z_crc_t,
      0xe839e25a as std::os::raw::c_ulong as z_crc_t,
      0xdf53205b as std::os::raw::c_ulong as z_crc_t,
      0x86ed6659 as std::os::raw::c_ulong as z_crc_t,
      0xb187a458 as std::os::raw::c_ulong as z_crc_t,
      0x3491eb5d as std::os::raw::c_ulong as z_crc_t,
      0x3fb295c as std::os::raw::c_ulong as z_crc_t,
      0x5a456f5e as std::os::raw::c_ulong as z_crc_t,
      0x6d2fad5f as std::os::raw::c_ulong as z_crc_t,
      0x801b35e1 as std::os::raw::c_ulong as z_crc_t,
      0xb771f7e0 as std::os::raw::c_ulong as z_crc_t,
      0xeecfb1e2 as std::os::raw::c_ulong as z_crc_t,
      0xd9a573e3 as std::os::raw::c_ulong as z_crc_t,
      0x5cb33ce6 as std::os::raw::c_ulong as z_crc_t,
      0x6bd9fee7 as std::os::raw::c_ulong as z_crc_t,
      0x3267b8e5 as std::os::raw::c_ulong as z_crc_t,
      0x50d7ae4 as std::os::raw::c_ulong as z_crc_t,
      0x384a26ef as std::os::raw::c_ulong as z_crc_t,
      0xf20e4ee as std::os::raw::c_ulong as z_crc_t,
      0x569ea2ec as std::os::raw::c_ulong as z_crc_t,
      0x61f460ed as std::os::raw::c_ulong as z_crc_t,
      0xe4e22fe8 as std::os::raw::c_ulong as z_crc_t,
      0xd388ede9 as std::os::raw::c_ulong as z_crc_t,
      0x8a36abeb as std::os::raw::c_ulong as z_crc_t,
      0xbd5c69ea as std::os::raw::c_ulong as z_crc_t,
      0xf0b813fd as std::os::raw::c_ulong as z_crc_t,
      0xc7d2d1fc as std::os::raw::c_ulong as z_crc_t,
      0x9e6c97fe as std::os::raw::c_ulong as z_crc_t,
      0xa90655ff as std::os::raw::c_ulong as z_crc_t,
      0x2c101afa as std::os::raw::c_ulong as z_crc_t,
      0x1b7ad8fb as std::os::raw::c_ulong as z_crc_t,
      0x42c49ef9 as std::os::raw::c_ulong as z_crc_t,
      0x75ae5cf8 as std::os::raw::c_ulong as z_crc_t,
      0x48e900f3 as std::os::raw::c_ulong as z_crc_t,
      0x7f83c2f2 as std::os::raw::c_ulong as z_crc_t,
      0x263d84f0 as std::os::raw::c_ulong as z_crc_t,
      0x115746f1 as std::os::raw::c_ulong as z_crc_t,
      0x944109f4 as std::os::raw::c_ulong as z_crc_t,
      0xa32bcbf5 as std::os::raw::c_ulong as z_crc_t,
      0xfa958df7 as std::os::raw::c_ulong as z_crc_t,
      0xcdff4ff6 as std::os::raw::c_ulong as z_crc_t,
      0x605d78d9 as std::os::raw::c_ulong as z_crc_t,
      0x5737bad8 as std::os::raw::c_ulong as z_crc_t,
      0xe89fcda as std::os::raw::c_ulong as z_crc_t,
      0x39e33edb as std::os::raw::c_ulong as z_crc_t,
      0xbcf571de as std::os::raw::c_ulong as z_crc_t,
      0x8b9fb3df as std::os::raw::c_ulong as z_crc_t,
      0xd221f5dd as std::os::raw::c_ulong as z_crc_t,
      0xe54b37dc as std::os::raw::c_ulong as z_crc_t,
      0xd80c6bd7 as std::os::raw::c_ulong as z_crc_t,
      0xef66a9d6 as std::os::raw::c_ulong as z_crc_t,
      0xb6d8efd4 as std::os::raw::c_ulong as z_crc_t,
      0x81b22dd5 as std::os::raw::c_ulong as z_crc_t,
      0x4a462d0 as std::os::raw::c_ulong as z_crc_t,
      0x33cea0d1 as std::os::raw::c_ulong as z_crc_t,
      0x6a70e6d3 as std::os::raw::c_ulong as z_crc_t,
      0x5d1a24d2 as std::os::raw::c_ulong as z_crc_t,
      0x10fe5ec5 as std::os::raw::c_ulong as z_crc_t,
      0x27949cc4 as std::os::raw::c_ulong as z_crc_t,
      0x7e2adac6 as std::os::raw::c_ulong as z_crc_t,
      0x494018c7 as std::os::raw::c_ulong as z_crc_t,
      0xcc5657c2 as std::os::raw::c_ulong as z_crc_t,
      0xfb3c95c3 as std::os::raw::c_ulong as z_crc_t,
      0xa282d3c1 as std::os::raw::c_ulong as z_crc_t,
      0x95e811c0 as std::os::raw::c_ulong as z_crc_t,
      0xa8af4dcb as std::os::raw::c_ulong as z_crc_t,
      0x9fc58fca as std::os::raw::c_ulong as z_crc_t,
      0xc67bc9c8 as std::os::raw::c_ulong as z_crc_t,
      0xf1110bc9 as std::os::raw::c_ulong as z_crc_t,
      0x740744cc as std::os::raw::c_ulong as z_crc_t,
      0x436d86cd as std::os::raw::c_ulong as z_crc_t,
      0x1ad3c0cf as std::os::raw::c_ulong as z_crc_t,
      0x2db902ce as std::os::raw::c_ulong as z_crc_t,
      0x4096af91 as std::os::raw::c_ulong as z_crc_t,
      0x77fc6d90 as std::os::raw::c_ulong as z_crc_t,
      0x2e422b92 as std::os::raw::c_ulong as z_crc_t,
      0x1928e993 as std::os::raw::c_ulong as z_crc_t,
      0x9c3ea696 as std::os::raw::c_ulong as z_crc_t,
      0xab546497 as std::os::raw::c_ulong as z_crc_t,
      0xf2ea2295 as std::os::raw::c_ulong as z_crc_t,
      0xc580e094 as std::os::raw::c_ulong as z_crc_t,
      0xf8c7bc9f as std::os::raw::c_ulong as z_crc_t,
      0xcfad7e9e as std::os::raw::c_ulong as z_crc_t,
      0x9613389c as std::os::raw::c_ulong as z_crc_t,
      0xa179fa9d as std::os::raw::c_ulong as z_crc_t,
      0x246fb598 as std::os::raw::c_ulong as z_crc_t,
      0x13057799 as std::os::raw::c_ulong as z_crc_t,
      0x4abb319b as std::os::raw::c_ulong as z_crc_t,
      0x7dd1f39a as std::os::raw::c_ulong as z_crc_t,
      0x3035898d as std::os::raw::c_ulong as z_crc_t,
      0x75f4b8c as std::os::raw::c_ulong as z_crc_t,
      0x5ee10d8e as std::os::raw::c_ulong as z_crc_t,
      0x698bcf8f as std::os::raw::c_ulong as z_crc_t,
      0xec9d808a as std::os::raw::c_ulong as z_crc_t,
      0xdbf7428b as std::os::raw::c_ulong as z_crc_t,
      0x82490489 as std::os::raw::c_ulong as z_crc_t,
      0xb523c688 as std::os::raw::c_ulong as z_crc_t,
      0x88649a83 as std::os::raw::c_ulong as z_crc_t,
      0xbf0e5882 as std::os::raw::c_ulong as z_crc_t,
      0xe6b01e80 as std::os::raw::c_ulong as z_crc_t,
      0xd1dadc81 as std::os::raw::c_ulong as z_crc_t,
      0x54cc9384 as std::os::raw::c_ulong as z_crc_t,
      0x63a65185 as std::os::raw::c_ulong as z_crc_t,
      0x3a181787 as std::os::raw::c_ulong as z_crc_t,
      0xd72d586 as std::os::raw::c_ulong as z_crc_t,
      0xa0d0e2a9 as std::os::raw::c_ulong as z_crc_t,
      0x97ba20a8 as std::os::raw::c_ulong as z_crc_t,
      0xce0466aa as std::os::raw::c_ulong as z_crc_t,
      0xf96ea4ab as std::os::raw::c_ulong as z_crc_t,
      0x7c78ebae as std::os::raw::c_ulong as z_crc_t,
      0x4b1229af as std::os::raw::c_ulong as z_crc_t,
      0x12ac6fad as std::os::raw::c_ulong as z_crc_t,
      0x25c6adac as std::os::raw::c_ulong as z_crc_t,
      0x1881f1a7 as std::os::raw::c_ulong as z_crc_t,
      0x2feb33a6 as std::os::raw::c_ulong as z_crc_t,
      0x765575a4 as std::os::raw::c_ulong as z_crc_t,
      0x413fb7a5 as std::os::raw::c_ulong as z_crc_t,
      0xc429f8a0 as std::os::raw::c_ulong as z_crc_t,
      0xf3433aa1 as std::os::raw::c_ulong as z_crc_t,
      0xaafd7ca3 as std::os::raw::c_ulong as z_crc_t,
      0x9d97bea2 as std::os::raw::c_ulong as z_crc_t,
      0xd073c4b5 as std::os::raw::c_ulong as z_crc_t,
      0xe71906b4 as std::os::raw::c_ulong as z_crc_t,
      0xbea740b6 as std::os::raw::c_ulong as z_crc_t,
      0x89cd82b7 as std::os::raw::c_ulong as z_crc_t,
      0xcdbcdb2 as std::os::raw::c_ulong as z_crc_t,
      0x3bb10fb3 as std::os::raw::c_ulong as z_crc_t,
      0x620f49b1 as std::os::raw::c_ulong as z_crc_t,
      0x55658bb0 as std::os::raw::c_ulong as z_crc_t,
      0x6822d7bb as std::os::raw::c_ulong as z_crc_t,
      0x5f4815ba as std::os::raw::c_ulong as z_crc_t,
      0x6f653b8 as std::os::raw::c_ulong as z_crc_t,
      0x319c91b9 as std::os::raw::c_ulong as z_crc_t,
      0xb48adebc as std::os::raw::c_ulong as z_crc_t,
      0x83e01cbd as std::os::raw::c_ulong as z_crc_t,
      0xda5e5abf as std::os::raw::c_ulong as z_crc_t,
      0xed3498be as std::os::raw::c_ulong as z_crc_t],
     [0 as std::os::raw::c_ulong as z_crc_t, 0x6567bcb8 as std::os::raw::c_ulong as z_crc_t,
      0x8bc809aa as std::os::raw::c_ulong as z_crc_t,
      0xeeafb512 as std::os::raw::c_ulong as z_crc_t,
      0x5797628f as std::os::raw::c_ulong as z_crc_t,
      0x32f0de37 as std::os::raw::c_ulong as z_crc_t,
      0xdc5f6b25 as std::os::raw::c_ulong as z_crc_t,
      0xb938d79d as std::os::raw::c_ulong as z_crc_t,
      0xef28b4c5 as std::os::raw::c_ulong as z_crc_t,
      0x8a4f087d as std::os::raw::c_ulong as z_crc_t,
      0x64e0bd6f as std::os::raw::c_ulong as z_crc_t,
      0x18701d7 as std::os::raw::c_ulong as z_crc_t,
      0xb8bfd64a as std::os::raw::c_ulong as z_crc_t,
      0xddd86af2 as std::os::raw::c_ulong as z_crc_t,
      0x3377dfe0 as std::os::raw::c_ulong as z_crc_t,
      0x56106358 as std::os::raw::c_ulong as z_crc_t,
      0x9f571950 as std::os::raw::c_ulong as z_crc_t,
      0xfa30a5e8 as std::os::raw::c_ulong as z_crc_t,
      0x149f10fa as std::os::raw::c_ulong as z_crc_t,
      0x71f8ac42 as std::os::raw::c_ulong as z_crc_t,
      0xc8c07bdf as std::os::raw::c_ulong as z_crc_t,
      0xada7c767 as std::os::raw::c_ulong as z_crc_t,
      0x43087275 as std::os::raw::c_ulong as z_crc_t,
      0x266fcecd as std::os::raw::c_ulong as z_crc_t,
      0x707fad95 as std::os::raw::c_ulong as z_crc_t,
      0x1518112d as std::os::raw::c_ulong as z_crc_t,
      0xfbb7a43f as std::os::raw::c_ulong as z_crc_t,
      0x9ed01887 as std::os::raw::c_ulong as z_crc_t,
      0x27e8cf1a as std::os::raw::c_ulong as z_crc_t,
      0x428f73a2 as std::os::raw::c_ulong as z_crc_t,
      0xac20c6b0 as std::os::raw::c_ulong as z_crc_t,
      0xc9477a08 as std::os::raw::c_ulong as z_crc_t,
      0x3eaf32a0 as std::os::raw::c_ulong as z_crc_t,
      0x5bc88e18 as std::os::raw::c_ulong as z_crc_t,
      0xb5673b0a as std::os::raw::c_ulong as z_crc_t,
      0xd00087b2 as std::os::raw::c_ulong as z_crc_t,
      0x6938502f as std::os::raw::c_ulong as z_crc_t,
      0xc5fec97 as std::os::raw::c_ulong as z_crc_t,
      0xe2f05985 as std::os::raw::c_ulong as z_crc_t,
      0x8797e53d as std::os::raw::c_ulong as z_crc_t,
      0xd1878665 as std::os::raw::c_ulong as z_crc_t,
      0xb4e03add as std::os::raw::c_ulong as z_crc_t,
      0x5a4f8fcf as std::os::raw::c_ulong as z_crc_t,
      0x3f283377 as std::os::raw::c_ulong as z_crc_t,
      0x8610e4ea as std::os::raw::c_ulong as z_crc_t,
      0xe3775852 as std::os::raw::c_ulong as z_crc_t,
      0xdd8ed40 as std::os::raw::c_ulong as z_crc_t,
      0x68bf51f8 as std::os::raw::c_ulong as z_crc_t,
      0xa1f82bf0 as std::os::raw::c_ulong as z_crc_t,
      0xc49f9748 as std::os::raw::c_ulong as z_crc_t,
      0x2a30225a as std::os::raw::c_ulong as z_crc_t,
      0x4f579ee2 as std::os::raw::c_ulong as z_crc_t,
      0xf66f497f as std::os::raw::c_ulong as z_crc_t,
      0x9308f5c7 as std::os::raw::c_ulong as z_crc_t,
      0x7da740d5 as std::os::raw::c_ulong as z_crc_t,
      0x18c0fc6d as std::os::raw::c_ulong as z_crc_t,
      0x4ed09f35 as std::os::raw::c_ulong as z_crc_t,
      0x2bb7238d as std::os::raw::c_ulong as z_crc_t,
      0xc518969f as std::os::raw::c_ulong as z_crc_t,
      0xa07f2a27 as std::os::raw::c_ulong as z_crc_t,
      0x1947fdba as std::os::raw::c_ulong as z_crc_t,
      0x7c204102 as std::os::raw::c_ulong as z_crc_t,
      0x928ff410 as std::os::raw::c_ulong as z_crc_t,
      0xf7e848a8 as std::os::raw::c_ulong as z_crc_t,
      0x3d58149b as std::os::raw::c_ulong as z_crc_t,
      0x583fa823 as std::os::raw::c_ulong as z_crc_t,
      0xb6901d31 as std::os::raw::c_ulong as z_crc_t,
      0xd3f7a189 as std::os::raw::c_ulong as z_crc_t,
      0x6acf7614 as std::os::raw::c_ulong as z_crc_t,
      0xfa8caac as std::os::raw::c_ulong as z_crc_t,
      0xe1077fbe as std::os::raw::c_ulong as z_crc_t,
      0x8460c306 as std::os::raw::c_ulong as z_crc_t,
      0xd270a05e as std::os::raw::c_ulong as z_crc_t,
      0xb7171ce6 as std::os::raw::c_ulong as z_crc_t,
      0x59b8a9f4 as std::os::raw::c_ulong as z_crc_t,
      0x3cdf154c as std::os::raw::c_ulong as z_crc_t,
      0x85e7c2d1 as std::os::raw::c_ulong as z_crc_t,
      0xe0807e69 as std::os::raw::c_ulong as z_crc_t,
      0xe2fcb7b as std::os::raw::c_ulong as z_crc_t,
      0x6b4877c3 as std::os::raw::c_ulong as z_crc_t,
      0xa20f0dcb as std::os::raw::c_ulong as z_crc_t,
      0xc768b173 as std::os::raw::c_ulong as z_crc_t,
      0x29c70461 as std::os::raw::c_ulong as z_crc_t,
      0x4ca0b8d9 as std::os::raw::c_ulong as z_crc_t,
      0xf5986f44 as std::os::raw::c_ulong as z_crc_t,
      0x90ffd3fc as std::os::raw::c_ulong as z_crc_t,
      0x7e5066ee as std::os::raw::c_ulong as z_crc_t,
      0x1b37da56 as std::os::raw::c_ulong as z_crc_t,
      0x4d27b90e as std::os::raw::c_ulong as z_crc_t,
      0x284005b6 as std::os::raw::c_ulong as z_crc_t,
      0xc6efb0a4 as std::os::raw::c_ulong as z_crc_t,
      0xa3880c1c as std::os::raw::c_ulong as z_crc_t,
      0x1ab0db81 as std::os::raw::c_ulong as z_crc_t,
      0x7fd76739 as std::os::raw::c_ulong as z_crc_t,
      0x9178d22b as std::os::raw::c_ulong as z_crc_t,
      0xf41f6e93 as std::os::raw::c_ulong as z_crc_t,
      0x3f7263b as std::os::raw::c_ulong as z_crc_t,
      0x66909a83 as std::os::raw::c_ulong as z_crc_t,
      0x883f2f91 as std::os::raw::c_ulong as z_crc_t,
      0xed589329 as std::os::raw::c_ulong as z_crc_t,
      0x546044b4 as std::os::raw::c_ulong as z_crc_t,
      0x3107f80c as std::os::raw::c_ulong as z_crc_t,
      0xdfa84d1e as std::os::raw::c_ulong as z_crc_t,
      0xbacff1a6 as std::os::raw::c_ulong as z_crc_t,
      0xecdf92fe as std::os::raw::c_ulong as z_crc_t,
      0x89b82e46 as std::os::raw::c_ulong as z_crc_t,
      0x67179b54 as std::os::raw::c_ulong as z_crc_t,
      0x27027ec as std::os::raw::c_ulong as z_crc_t,
      0xbb48f071 as std::os::raw::c_ulong as z_crc_t,
      0xde2f4cc9 as std::os::raw::c_ulong as z_crc_t,
      0x3080f9db as std::os::raw::c_ulong as z_crc_t,
      0x55e74563 as std::os::raw::c_ulong as z_crc_t,
      0x9ca03f6b as std::os::raw::c_ulong as z_crc_t,
      0xf9c783d3 as std::os::raw::c_ulong as z_crc_t,
      0x176836c1 as std::os::raw::c_ulong as z_crc_t,
      0x720f8a79 as std::os::raw::c_ulong as z_crc_t,
      0xcb375de4 as std::os::raw::c_ulong as z_crc_t,
      0xae50e15c as std::os::raw::c_ulong as z_crc_t,
      0x40ff544e as std::os::raw::c_ulong as z_crc_t,
      0x2598e8f6 as std::os::raw::c_ulong as z_crc_t,
      0x73888bae as std::os::raw::c_ulong as z_crc_t,
      0x16ef3716 as std::os::raw::c_ulong as z_crc_t,
      0xf8408204 as std::os::raw::c_ulong as z_crc_t,
      0x9d273ebc as std::os::raw::c_ulong as z_crc_t,
      0x241fe921 as std::os::raw::c_ulong as z_crc_t,
      0x41785599 as std::os::raw::c_ulong as z_crc_t,
      0xafd7e08b as std::os::raw::c_ulong as z_crc_t,
      0xcab05c33 as std::os::raw::c_ulong as z_crc_t,
      0x3bb659ed as std::os::raw::c_ulong as z_crc_t,
      0x5ed1e555 as std::os::raw::c_ulong as z_crc_t,
      0xb07e5047 as std::os::raw::c_ulong as z_crc_t,
      0xd519ecff as std::os::raw::c_ulong as z_crc_t,
      0x6c213b62 as std::os::raw::c_ulong as z_crc_t,
      0x94687da as std::os::raw::c_ulong as z_crc_t,
      0xe7e932c8 as std::os::raw::c_ulong as z_crc_t,
      0x828e8e70 as std::os::raw::c_ulong as z_crc_t,
      0xd49eed28 as std::os::raw::c_ulong as z_crc_t,
      0xb1f95190 as std::os::raw::c_ulong as z_crc_t,
      0x5f56e482 as std::os::raw::c_ulong as z_crc_t,
      0x3a31583a as std::os::raw::c_ulong as z_crc_t,
      0x83098fa7 as std::os::raw::c_ulong as z_crc_t,
      0xe66e331f as std::os::raw::c_ulong as z_crc_t,
      0x8c1860d as std::os::raw::c_ulong as z_crc_t,
      0x6da63ab5 as std::os::raw::c_ulong as z_crc_t,
      0xa4e140bd as std::os::raw::c_ulong as z_crc_t,
      0xc186fc05 as std::os::raw::c_ulong as z_crc_t,
      0x2f294917 as std::os::raw::c_ulong as z_crc_t,
      0x4a4ef5af as std::os::raw::c_ulong as z_crc_t,
      0xf3762232 as std::os::raw::c_ulong as z_crc_t,
      0x96119e8a as std::os::raw::c_ulong as z_crc_t,
      0x78be2b98 as std::os::raw::c_ulong as z_crc_t,
      0x1dd99720 as std::os::raw::c_ulong as z_crc_t,
      0x4bc9f478 as std::os::raw::c_ulong as z_crc_t,
      0x2eae48c0 as std::os::raw::c_ulong as z_crc_t,
      0xc001fdd2 as std::os::raw::c_ulong as z_crc_t,
      0xa566416a as std::os::raw::c_ulong as z_crc_t,
      0x1c5e96f7 as std::os::raw::c_ulong as z_crc_t,
      0x79392a4f as std::os::raw::c_ulong as z_crc_t,
      0x97969f5d as std::os::raw::c_ulong as z_crc_t,
      0xf2f123e5 as std::os::raw::c_ulong as z_crc_t,
      0x5196b4d as std::os::raw::c_ulong as z_crc_t,
      0x607ed7f5 as std::os::raw::c_ulong as z_crc_t,
      0x8ed162e7 as std::os::raw::c_ulong as z_crc_t,
      0xebb6de5f as std::os::raw::c_ulong as z_crc_t,
      0x528e09c2 as std::os::raw::c_ulong as z_crc_t,
      0x37e9b57a as std::os::raw::c_ulong as z_crc_t,
      0xd9460068 as std::os::raw::c_ulong as z_crc_t,
      0xbc21bcd0 as std::os::raw::c_ulong as z_crc_t,
      0xea31df88 as std::os::raw::c_ulong as z_crc_t,
      0x8f566330 as std::os::raw::c_ulong as z_crc_t,
      0x61f9d622 as std::os::raw::c_ulong as z_crc_t,
      0x49e6a9a as std::os::raw::c_ulong as z_crc_t,
      0xbda6bd07 as std::os::raw::c_ulong as z_crc_t,
      0xd8c101bf as std::os::raw::c_ulong as z_crc_t,
      0x366eb4ad as std::os::raw::c_ulong as z_crc_t,
      0x53090815 as std::os::raw::c_ulong as z_crc_t,
      0x9a4e721d as std::os::raw::c_ulong as z_crc_t,
      0xff29cea5 as std::os::raw::c_ulong as z_crc_t,
      0x11867bb7 as std::os::raw::c_ulong as z_crc_t,
      0x74e1c70f as std::os::raw::c_ulong as z_crc_t,
      0xcdd91092 as std::os::raw::c_ulong as z_crc_t,
      0xa8beac2a as std::os::raw::c_ulong as z_crc_t,
      0x46111938 as std::os::raw::c_ulong as z_crc_t,
      0x2376a580 as std::os::raw::c_ulong as z_crc_t,
      0x7566c6d8 as std::os::raw::c_ulong as z_crc_t,
      0x10017a60 as std::os::raw::c_ulong as z_crc_t,
      0xfeaecf72 as std::os::raw::c_ulong as z_crc_t,
      0x9bc973ca as std::os::raw::c_ulong as z_crc_t,
      0x22f1a457 as std::os::raw::c_ulong as z_crc_t,
      0x479618ef as std::os::raw::c_ulong as z_crc_t,
      0xa939adfd as std::os::raw::c_ulong as z_crc_t,
      0xcc5e1145 as std::os::raw::c_ulong as z_crc_t,
      0x6ee4d76 as std::os::raw::c_ulong as z_crc_t,
      0x6389f1ce as std::os::raw::c_ulong as z_crc_t,
      0x8d2644dc as std::os::raw::c_ulong as z_crc_t,
      0xe841f864 as std::os::raw::c_ulong as z_crc_t,
      0x51792ff9 as std::os::raw::c_ulong as z_crc_t,
      0x341e9341 as std::os::raw::c_ulong as z_crc_t,
      0xdab12653 as std::os::raw::c_ulong as z_crc_t,
      0xbfd69aeb as std::os::raw::c_ulong as z_crc_t,
      0xe9c6f9b3 as std::os::raw::c_ulong as z_crc_t,
      0x8ca1450b as std::os::raw::c_ulong as z_crc_t,
      0x620ef019 as std::os::raw::c_ulong as z_crc_t,
      0x7694ca1 as std::os::raw::c_ulong as z_crc_t,
      0xbe519b3c as std::os::raw::c_ulong as z_crc_t,
      0xdb362784 as std::os::raw::c_ulong as z_crc_t,
      0x35999296 as std::os::raw::c_ulong as z_crc_t,
      0x50fe2e2e as std::os::raw::c_ulong as z_crc_t,
      0x99b95426 as std::os::raw::c_ulong as z_crc_t,
      0xfcdee89e as std::os::raw::c_ulong as z_crc_t,
      0x12715d8c as std::os::raw::c_ulong as z_crc_t,
      0x7716e134 as std::os::raw::c_ulong as z_crc_t,
      0xce2e36a9 as std::os::raw::c_ulong as z_crc_t,
      0xab498a11 as std::os::raw::c_ulong as z_crc_t,
      0x45e63f03 as std::os::raw::c_ulong as z_crc_t,
      0x208183bb as std::os::raw::c_ulong as z_crc_t,
      0x7691e0e3 as std::os::raw::c_ulong as z_crc_t,
      0x13f65c5b as std::os::raw::c_ulong as z_crc_t,
      0xfd59e949 as std::os::raw::c_ulong as z_crc_t,
      0x983e55f1 as std::os::raw::c_ulong as z_crc_t,
      0x2106826c as std::os::raw::c_ulong as z_crc_t,
      0x44613ed4 as std::os::raw::c_ulong as z_crc_t,
      0xaace8bc6 as std::os::raw::c_ulong as z_crc_t,
      0xcfa9377e as std::os::raw::c_ulong as z_crc_t,
      0x38417fd6 as std::os::raw::c_ulong as z_crc_t,
      0x5d26c36e as std::os::raw::c_ulong as z_crc_t,
      0xb389767c as std::os::raw::c_ulong as z_crc_t,
      0xd6eecac4 as std::os::raw::c_ulong as z_crc_t,
      0x6fd61d59 as std::os::raw::c_ulong as z_crc_t,
      0xab1a1e1 as std::os::raw::c_ulong as z_crc_t,
      0xe41e14f3 as std::os::raw::c_ulong as z_crc_t,
      0x8179a84b as std::os::raw::c_ulong as z_crc_t,
      0xd769cb13 as std::os::raw::c_ulong as z_crc_t,
      0xb20e77ab as std::os::raw::c_ulong as z_crc_t,
      0x5ca1c2b9 as std::os::raw::c_ulong as z_crc_t,
      0x39c67e01 as std::os::raw::c_ulong as z_crc_t,
      0x80fea99c as std::os::raw::c_ulong as z_crc_t,
      0xe5991524 as std::os::raw::c_ulong as z_crc_t,
      0xb36a036 as std::os::raw::c_ulong as z_crc_t,
      0x6e511c8e as std::os::raw::c_ulong as z_crc_t,
      0xa7166686 as std::os::raw::c_ulong as z_crc_t,
      0xc271da3e as std::os::raw::c_ulong as z_crc_t,
      0x2cde6f2c as std::os::raw::c_ulong as z_crc_t,
      0x49b9d394 as std::os::raw::c_ulong as z_crc_t,
      0xf0810409 as std::os::raw::c_ulong as z_crc_t,
      0x95e6b8b1 as std::os::raw::c_ulong as z_crc_t,
      0x7b490da3 as std::os::raw::c_ulong as z_crc_t,
      0x1e2eb11b as std::os::raw::c_ulong as z_crc_t,
      0x483ed243 as std::os::raw::c_ulong as z_crc_t,
      0x2d596efb as std::os::raw::c_ulong as z_crc_t,
      0xc3f6dbe9 as std::os::raw::c_ulong as z_crc_t,
      0xa6916751 as std::os::raw::c_ulong as z_crc_t,
      0x1fa9b0cc as std::os::raw::c_ulong as z_crc_t,
      0x7ace0c74 as std::os::raw::c_ulong as z_crc_t,
      0x9461b966 as std::os::raw::c_ulong as z_crc_t,
      0xf10605de as std::os::raw::c_ulong as z_crc_t]];
/* !DYNAMIC_CRC_TABLE */
/* ========================================================================
 * Tables of CRC-32s of all single-byte values, made by make_crc_table().
 */
/* DYNAMIC_CRC_TABLE */
/* =========================================================================
 * This function can be used by asm versions of crc32()
 */
#[no_mangle]
pub unsafe extern "C" fn get_crc_table() -> *const z_crc_t {
    /* DYNAMIC_CRC_TABLE */
    return crc_table.as_ptr() as *const z_crc_t;
}
/* ========================================================================= */
/* ========================================================================= */
#[no_mangle]
pub unsafe extern "C" fn crc32_z(mut crc: std::os::raw::c_ulong,
                                 mut buf: *const std::os::raw::c_uchar,
                                 mut len: z_size_t) -> uLong {
    if buf.is_null() { return 0 as std::os::raw::c_ulong }
    /* DYNAMIC_CRC_TABLE */
    if ::std::mem::size_of::<*mut std::os::raw::c_void>() as std::os::raw::c_ulong ==
           ::std::mem::size_of::<ptrdiff_t>() as std::os::raw::c_ulong {
        let mut endian: z_crc_t = 0;
        endian = 1 as std::os::raw::c_int as z_crc_t;
        if *(&mut endian as *mut z_crc_t as *mut std::os::raw::c_uchar) != 0 {
            return crc32_little(crc, buf, len)
        } else { return crc32_big(crc, buf, len) }
    }
    /* BYFOUR */
    crc = crc ^ 0xffffffff as std::os::raw::c_ulong;
    while len >= 8 as std::os::raw::c_int as std::os::raw::c_ulong {
        let fresh0 = buf;
        buf = buf.offset(1);
        crc =
            crc_table[0 as std::os::raw::c_int as
                          usize][((crc as std::os::raw::c_int ^
                                       *fresh0 as std::os::raw::c_int) &
                                      0xff as std::os::raw::c_int) as usize] as
                std::os::raw::c_ulong ^ crc >> 8 as std::os::raw::c_int;
        let fresh1 = buf;
        buf = buf.offset(1);
        crc =
            crc_table[0 as std::os::raw::c_int as
                          usize][((crc as std::os::raw::c_int ^
                                       *fresh1 as std::os::raw::c_int) &
                                      0xff as std::os::raw::c_int) as usize] as
                std::os::raw::c_ulong ^ crc >> 8 as std::os::raw::c_int;
        let fresh2 = buf;
        buf = buf.offset(1);
        crc =
            crc_table[0 as std::os::raw::c_int as
                          usize][((crc as std::os::raw::c_int ^
                                       *fresh2 as std::os::raw::c_int) &
                                      0xff as std::os::raw::c_int) as usize] as
                std::os::raw::c_ulong ^ crc >> 8 as std::os::raw::c_int;
        let fresh3 = buf;
        buf = buf.offset(1);
        crc =
            crc_table[0 as std::os::raw::c_int as
                          usize][((crc as std::os::raw::c_int ^
                                       *fresh3 as std::os::raw::c_int) &
                                      0xff as std::os::raw::c_int) as usize] as
                std::os::raw::c_ulong ^ crc >> 8 as std::os::raw::c_int;
        let fresh4 = buf;
        buf = buf.offset(1);
        crc =
            crc_table[0 as std::os::raw::c_int as
                          usize][((crc as std::os::raw::c_int ^
                                       *fresh4 as std::os::raw::c_int) &
                                      0xff as std::os::raw::c_int) as usize] as
                std::os::raw::c_ulong ^ crc >> 8 as std::os::raw::c_int;
        let fresh5 = buf;
        buf = buf.offset(1);
        crc =
            crc_table[0 as std::os::raw::c_int as
                          usize][((crc as std::os::raw::c_int ^
                                       *fresh5 as std::os::raw::c_int) &
                                      0xff as std::os::raw::c_int) as usize] as
                std::os::raw::c_ulong ^ crc >> 8 as std::os::raw::c_int;
        let fresh6 = buf;
        buf = buf.offset(1);
        crc =
            crc_table[0 as std::os::raw::c_int as
                          usize][((crc as std::os::raw::c_int ^
                                       *fresh6 as std::os::raw::c_int) &
                                      0xff as std::os::raw::c_int) as usize] as
                std::os::raw::c_ulong ^ crc >> 8 as std::os::raw::c_int;
        let fresh7 = buf;
        buf = buf.offset(1);
        crc =
            crc_table[0 as std::os::raw::c_int as
                          usize][((crc as std::os::raw::c_int ^
                                       *fresh7 as std::os::raw::c_int) &
                                      0xff as std::os::raw::c_int) as usize] as
                std::os::raw::c_ulong ^ crc >> 8 as std::os::raw::c_int;
        len =
            (len as
                 std::os::raw::c_ulong).wrapping_sub(8 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as z_size_t as
                z_size_t
    }
    if len != 0 {
        loop  {
            let fresh8 = buf;
            buf = buf.offset(1);
            crc =
                crc_table[0 as std::os::raw::c_int as
                              usize][((crc as std::os::raw::c_int ^
                                           *fresh8 as std::os::raw::c_int) &
                                          0xff as std::os::raw::c_int) as usize] as
                    std::os::raw::c_ulong ^ crc >> 8 as std::os::raw::c_int;
            len = len.wrapping_sub(1);
            if !(len != 0) { break ; }
        }
    }
    return crc ^ 0xffffffff as std::os::raw::c_ulong;
}
/* ========================================================================= */
#[no_mangle]
pub unsafe extern "C" fn crc32(mut crc: std::os::raw::c_ulong,
                               mut buf: *const std::os::raw::c_uchar, mut len: uInt)
 -> uLong {
    return crc32_z(crc, buf, len as z_size_t);
}
/* crc32.c -- compute the CRC-32 of a data stream
 * Copyright (C) 1995-2006, 2010, 2011, 2012, 2016 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 *
 * Thanks to Rodney Brown <rbrown64@csc.com.au> for his contribution of faster
 * CRC methods: exclusive-oring 32 bits of data at a time, and pre-computing
 * tables for updating the shift register in one step with three exclusive-ors
 * instead of four steps with four exclusive-ors.  This results in about a
 * factor of two increase in speed on a Power PC G4 (PPC7455) using gcc -O3.
 */
/* @(#) $Id$ */
/*
  Note on the use of DYNAMIC_CRC_TABLE: there is no mutex or semaphore
  protection on the static variables used to control the first-use generation
  of the crc tables.  Therefore, if you #define DYNAMIC_CRC_TABLE, you should
  first call get_crc_table() to initialize the tables before allowing more than
  one thread to use crc32().

  DYNAMIC_CRC_TABLE and MAKECRCH can be #defined to write out crc32.h.
 */
/* MAKECRCH */
/* for STDC and FAR definitions */
/* Definitions for doing the crc four data bytes at a time. */
/*
   This BYFOUR code accesses the passed unsigned char * buffer with a 32-bit
   integer pointer type. This violates the strict aliasing rule, where a
   compiler can assume, for optimization purposes, that two pointers to
   fundamentally different types won't ever point to the same memory. This can
   manifest as a problem only if one of the pointers is written to. This code
   only reads from those pointers. So long as this code remains isolated in
   this compilation unit, there won't be a problem. For this reason, this code
   should not be copied and pasted into a compilation unit in which other code
   writes to the buffer that is passed to these routines.
 */
/* ========================================================================= */
/* ========================================================================= */
unsafe extern "C" fn crc32_little(mut crc: std::os::raw::c_ulong,
                                  mut buf: *const std::os::raw::c_uchar,
                                  mut len: z_size_t) -> std::os::raw::c_ulong {
    let mut c: z_crc_t = 0;
    let mut buf4: *const z_crc_t = 0 as *const z_crc_t;
    c = crc as z_crc_t;
    c = !c;
    while len != 0 && buf as ptrdiff_t & 3 as std::os::raw::c_int as std::os::raw::c_long != 0
          {
        let fresh9 = buf;
        buf = buf.offset(1);
        c =
            crc_table[0 as std::os::raw::c_int as
                          usize][((c ^ *fresh9 as std::os::raw::c_uint) &
                                      0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                                     usize] ^ c >> 8 as std::os::raw::c_int;
        len = len.wrapping_sub(1)
    }
    buf4 = buf as *const std::os::raw::c_void as *const z_crc_t;
    while len >= 32 as std::os::raw::c_int as std::os::raw::c_ulong {
        let fresh10 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh10;
        c =
            crc_table[3 as std::os::raw::c_int as
                          usize][(c & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                                     usize] ^
                crc_table[2 as std::os::raw::c_int as
                              usize][(c >> 8 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[1 as std::os::raw::c_int as
                              usize][(c >> 16 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[0 as std::os::raw::c_int as
                              usize][(c >> 24 as std::os::raw::c_int) as usize];
        let fresh11 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh11;
        c =
            crc_table[3 as std::os::raw::c_int as
                          usize][(c & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                                     usize] ^
                crc_table[2 as std::os::raw::c_int as
                              usize][(c >> 8 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[1 as std::os::raw::c_int as
                              usize][(c >> 16 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[0 as std::os::raw::c_int as
                              usize][(c >> 24 as std::os::raw::c_int) as usize];
        let fresh12 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh12;
        c =
            crc_table[3 as std::os::raw::c_int as
                          usize][(c & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                                     usize] ^
                crc_table[2 as std::os::raw::c_int as
                              usize][(c >> 8 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[1 as std::os::raw::c_int as
                              usize][(c >> 16 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[0 as std::os::raw::c_int as
                              usize][(c >> 24 as std::os::raw::c_int) as usize];
        let fresh13 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh13;
        c =
            crc_table[3 as std::os::raw::c_int as
                          usize][(c & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                                     usize] ^
                crc_table[2 as std::os::raw::c_int as
                              usize][(c >> 8 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[1 as std::os::raw::c_int as
                              usize][(c >> 16 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[0 as std::os::raw::c_int as
                              usize][(c >> 24 as std::os::raw::c_int) as usize];
        let fresh14 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh14;
        c =
            crc_table[3 as std::os::raw::c_int as
                          usize][(c & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                                     usize] ^
                crc_table[2 as std::os::raw::c_int as
                              usize][(c >> 8 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[1 as std::os::raw::c_int as
                              usize][(c >> 16 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[0 as std::os::raw::c_int as
                              usize][(c >> 24 as std::os::raw::c_int) as usize];
        let fresh15 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh15;
        c =
            crc_table[3 as std::os::raw::c_int as
                          usize][(c & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                                     usize] ^
                crc_table[2 as std::os::raw::c_int as
                              usize][(c >> 8 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[1 as std::os::raw::c_int as
                              usize][(c >> 16 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[0 as std::os::raw::c_int as
                              usize][(c >> 24 as std::os::raw::c_int) as usize];
        let fresh16 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh16;
        c =
            crc_table[3 as std::os::raw::c_int as
                          usize][(c & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                                     usize] ^
                crc_table[2 as std::os::raw::c_int as
                              usize][(c >> 8 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[1 as std::os::raw::c_int as
                              usize][(c >> 16 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[0 as std::os::raw::c_int as
                              usize][(c >> 24 as std::os::raw::c_int) as usize];
        let fresh17 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh17;
        c =
            crc_table[3 as std::os::raw::c_int as
                          usize][(c & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                                     usize] ^
                crc_table[2 as std::os::raw::c_int as
                              usize][(c >> 8 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[1 as std::os::raw::c_int as
                              usize][(c >> 16 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[0 as std::os::raw::c_int as
                              usize][(c >> 24 as std::os::raw::c_int) as usize];
        len =
            (len as
                 std::os::raw::c_ulong).wrapping_sub(32 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as z_size_t as
                z_size_t
    }
    while len >= 4 as std::os::raw::c_int as std::os::raw::c_ulong {
        let fresh18 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh18;
        c =
            crc_table[3 as std::os::raw::c_int as
                          usize][(c & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                                     usize] ^
                crc_table[2 as std::os::raw::c_int as
                              usize][(c >> 8 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[1 as std::os::raw::c_int as
                              usize][(c >> 16 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[0 as std::os::raw::c_int as
                              usize][(c >> 24 as std::os::raw::c_int) as usize];
        len =
            (len as
                 std::os::raw::c_ulong).wrapping_sub(4 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as z_size_t as
                z_size_t
    }
    buf = buf4 as *const std::os::raw::c_uchar;
    if len != 0 {
        loop  {
            let fresh19 = buf;
            buf = buf.offset(1);
            c =
                crc_table[0 as std::os::raw::c_int as
                              usize][((c ^ *fresh19 as std::os::raw::c_uint) &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^ c >> 8 as std::os::raw::c_int;
            len = len.wrapping_sub(1);
            if !(len != 0) { break ; }
        }
    }
    c = !c;
    return c as std::os::raw::c_ulong;
}
/* ========================================================================= */
/* ========================================================================= */
unsafe extern "C" fn crc32_big(mut crc: std::os::raw::c_ulong,
                               mut buf: *const std::os::raw::c_uchar,
                               mut len: z_size_t) -> std::os::raw::c_ulong {
    let mut c: z_crc_t = 0;
    let mut buf4: *const z_crc_t = 0 as *const z_crc_t;
    c =
        (crc as z_crc_t >> 24 as std::os::raw::c_int &
             0xff as std::os::raw::c_int as
                 std::os::raw::c_uint).wrapping_add(crc as z_crc_t >> 8 as std::os::raw::c_int
                                                &
                                                0xff00 as std::os::raw::c_int as
                                                    std::os::raw::c_uint).wrapping_add((crc
                                                                                    as
                                                                                    z_crc_t
                                                                                    &
                                                                                    0xff00
                                                                                        as
                                                                                        std::os::raw::c_int
                                                                                        as
                                                                                        std::os::raw::c_uint)
                                                                                   <<
                                                                                   8
                                                                                       as
                                                                                       std::os::raw::c_int).wrapping_add((crc
                                                                                                                      as
                                                                                                                      z_crc_t
                                                                                                                      &
                                                                                                                      0xff
                                                                                                                          as
                                                                                                                          std::os::raw::c_int
                                                                                                                          as
                                                                                                                          std::os::raw::c_uint)
                                                                                                                     <<
                                                                                                                     24
                                                                                                                         as
                                                                                                                         std::os::raw::c_int);
    c = !c;
    while len != 0 && buf as ptrdiff_t & 3 as std::os::raw::c_int as std::os::raw::c_long != 0
          {
        let fresh20 = buf;
        buf = buf.offset(1);
        c =
            crc_table[4 as std::os::raw::c_int as
                          usize][(c >> 24 as std::os::raw::c_int ^
                                      *fresh20 as std::os::raw::c_uint) as usize] ^
                c << 8 as std::os::raw::c_int;
        len = len.wrapping_sub(1)
    }
    buf4 = buf as *const std::os::raw::c_void as *const z_crc_t;
    while len >= 32 as std::os::raw::c_int as std::os::raw::c_ulong {
        let fresh21 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh21;
        c =
            crc_table[4 as std::os::raw::c_int as
                          usize][(c & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                                     usize] ^
                crc_table[5 as std::os::raw::c_int as
                              usize][(c >> 8 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[6 as std::os::raw::c_int as
                              usize][(c >> 16 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[7 as std::os::raw::c_int as
                              usize][(c >> 24 as std::os::raw::c_int) as usize];
        let fresh22 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh22;
        c =
            crc_table[4 as std::os::raw::c_int as
                          usize][(c & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                                     usize] ^
                crc_table[5 as std::os::raw::c_int as
                              usize][(c >> 8 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[6 as std::os::raw::c_int as
                              usize][(c >> 16 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[7 as std::os::raw::c_int as
                              usize][(c >> 24 as std::os::raw::c_int) as usize];
        let fresh23 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh23;
        c =
            crc_table[4 as std::os::raw::c_int as
                          usize][(c & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                                     usize] ^
                crc_table[5 as std::os::raw::c_int as
                              usize][(c >> 8 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[6 as std::os::raw::c_int as
                              usize][(c >> 16 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[7 as std::os::raw::c_int as
                              usize][(c >> 24 as std::os::raw::c_int) as usize];
        let fresh24 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh24;
        c =
            crc_table[4 as std::os::raw::c_int as
                          usize][(c & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                                     usize] ^
                crc_table[5 as std::os::raw::c_int as
                              usize][(c >> 8 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[6 as std::os::raw::c_int as
                              usize][(c >> 16 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[7 as std::os::raw::c_int as
                              usize][(c >> 24 as std::os::raw::c_int) as usize];
        let fresh25 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh25;
        c =
            crc_table[4 as std::os::raw::c_int as
                          usize][(c & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                                     usize] ^
                crc_table[5 as std::os::raw::c_int as
                              usize][(c >> 8 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[6 as std::os::raw::c_int as
                              usize][(c >> 16 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[7 as std::os::raw::c_int as
                              usize][(c >> 24 as std::os::raw::c_int) as usize];
        let fresh26 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh26;
        c =
            crc_table[4 as std::os::raw::c_int as
                          usize][(c & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                                     usize] ^
                crc_table[5 as std::os::raw::c_int as
                              usize][(c >> 8 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[6 as std::os::raw::c_int as
                              usize][(c >> 16 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[7 as std::os::raw::c_int as
                              usize][(c >> 24 as std::os::raw::c_int) as usize];
        let fresh27 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh27;
        c =
            crc_table[4 as std::os::raw::c_int as
                          usize][(c & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                                     usize] ^
                crc_table[5 as std::os::raw::c_int as
                              usize][(c >> 8 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[6 as std::os::raw::c_int as
                              usize][(c >> 16 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[7 as std::os::raw::c_int as
                              usize][(c >> 24 as std::os::raw::c_int) as usize];
        let fresh28 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh28;
        c =
            crc_table[4 as std::os::raw::c_int as
                          usize][(c & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                                     usize] ^
                crc_table[5 as std::os::raw::c_int as
                              usize][(c >> 8 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[6 as std::os::raw::c_int as
                              usize][(c >> 16 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[7 as std::os::raw::c_int as
                              usize][(c >> 24 as std::os::raw::c_int) as usize];
        len =
            (len as
                 std::os::raw::c_ulong).wrapping_sub(32 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as z_size_t as
                z_size_t
    }
    while len >= 4 as std::os::raw::c_int as std::os::raw::c_ulong {
        let fresh29 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh29;
        c =
            crc_table[4 as std::os::raw::c_int as
                          usize][(c & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                                     usize] ^
                crc_table[5 as std::os::raw::c_int as
                              usize][(c >> 8 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[6 as std::os::raw::c_int as
                              usize][(c >> 16 as std::os::raw::c_int &
                                          0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                         as usize] ^
                crc_table[7 as std::os::raw::c_int as
                              usize][(c >> 24 as std::os::raw::c_int) as usize];
        len =
            (len as
                 std::os::raw::c_ulong).wrapping_sub(4 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as z_size_t as
                z_size_t
    }
    buf = buf4 as *const std::os::raw::c_uchar;
    if len != 0 {
        loop  {
            let fresh30 = buf;
            buf = buf.offset(1);
            c =
                crc_table[4 as std::os::raw::c_int as
                              usize][(c >> 24 as std::os::raw::c_int ^
                                          *fresh30 as std::os::raw::c_uint) as usize]
                    ^ c << 8 as std::os::raw::c_int;
            len = len.wrapping_sub(1);
            if !(len != 0) { break ; }
        }
    }
    c = !c;
    return (c >> 24 as std::os::raw::c_int &
                0xff as std::os::raw::c_int as
                    std::os::raw::c_uint).wrapping_add(c >> 8 as std::os::raw::c_int &
                                                   0xff00 as std::os::raw::c_int as
                                                       std::os::raw::c_uint).wrapping_add((c
                                                                                       &
                                                                                       0xff00
                                                                                           as
                                                                                           std::os::raw::c_int
                                                                                           as
                                                                                           std::os::raw::c_uint)
                                                                                      <<
                                                                                      8
                                                                                          as
                                                                                          std::os::raw::c_int).wrapping_add((c
                                                                                                                         &
                                                                                                                         0xff
                                                                                                                             as
                                                                                                                             std::os::raw::c_int
                                                                                                                             as
                                                                                                                             std::os::raw::c_uint)
                                                                                                                        <<
                                                                                                                        24
                                                                                                                            as
                                                                                                                            std::os::raw::c_int)
               as std::os::raw::c_ulong;
}
/* BYFOUR */
/* Local functions for crc concatenation */
/* dimension of GF(2) vectors (length of CRC) */
/* ========================================================================= */
unsafe extern "C" fn gf2_matrix_times(mut mat: *mut std::os::raw::c_ulong,
                                      mut vec: std::os::raw::c_ulong)
 -> std::os::raw::c_ulong {
    let mut sum: std::os::raw::c_ulong = 0;
    sum = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
    while vec != 0 {
        if vec & 1 as std::os::raw::c_int as std::os::raw::c_ulong != 0 { sum ^= *mat }
        vec >>= 1 as std::os::raw::c_int;
        mat = mat.offset(1)
    }
    return sum;
}
/* ========================================================================= */
unsafe extern "C" fn gf2_matrix_square(mut square: *mut std::os::raw::c_ulong,
                                       mut mat: *mut std::os::raw::c_ulong) {
    let mut n: std::os::raw::c_int = 0;
    n = 0 as std::os::raw::c_int;
    while n < 32 as std::os::raw::c_int {
        *square.offset(n as isize) =
            gf2_matrix_times(mat, *mat.offset(n as isize));
        n += 1
    };
}
/* ========================================================================= */
unsafe extern "C" fn crc32_combine_(mut crc1: uLong, mut crc2: uLong,
                                    mut len2: off64_t) -> uLong {
    let mut n: std::os::raw::c_int = 0; /* even-power-of-two zeros operator */
    let mut row: std::os::raw::c_ulong = 0; /* odd-power-of-two zeros operator */
    let mut even: [std::os::raw::c_ulong; 32] = [0; 32];
    let mut odd: [std::os::raw::c_ulong; 32] = [0; 32];
    /* degenerate case (also disallow negative lengths) */
    if len2 <= 0 as std::os::raw::c_int as std::os::raw::c_long { return crc1 }
    /* put operator for one zero bit in odd */
    odd[0 as std::os::raw::c_int as usize] =
        0xedb88320 as std::os::raw::c_ulong; /* CRC-32 polynomial */
    row = 1 as std::os::raw::c_int as std::os::raw::c_ulong;
    n = 1 as std::os::raw::c_int;
    while n < 32 as std::os::raw::c_int {
        odd[n as usize] = row;
        row <<= 1 as std::os::raw::c_int;
        n += 1
    }
    /* put operator for two zero bits in even */
    gf2_matrix_square(even.as_mut_ptr(), odd.as_mut_ptr());
    /* put operator for four zero bits in odd */
    gf2_matrix_square(odd.as_mut_ptr(), even.as_mut_ptr());
    loop 
         /* apply len2 zeros to crc1 (first square will put the operator for one
       zero byte, eight zero bits, in even) */
         /* apply zeros operator for this bit of len2 */
         {
        gf2_matrix_square(even.as_mut_ptr(), odd.as_mut_ptr());
        if len2 & 1 as std::os::raw::c_int as std::os::raw::c_long != 0 {
            crc1 = gf2_matrix_times(even.as_mut_ptr(), crc1)
        }
        len2 >>= 1 as std::os::raw::c_int;
        /* if no more bits set, then done */
        if len2 == 0 as std::os::raw::c_int as std::os::raw::c_long { break ; }
        /* another iteration of the loop with odd and even swapped */
        gf2_matrix_square(odd.as_mut_ptr(), even.as_mut_ptr());
        if len2 & 1 as std::os::raw::c_int as std::os::raw::c_long != 0 {
            crc1 = gf2_matrix_times(odd.as_mut_ptr(), crc1)
        }
        len2 >>= 1 as std::os::raw::c_int;
        if !(len2 != 0 as std::os::raw::c_int as std::os::raw::c_long) { break ; }
        /* if no more bits set, then done */
    }
    /* return combined crc */
    crc1 ^= crc2;
    return crc1;
}
/* ========================================================================= */
#[no_mangle]
pub unsafe extern "C" fn crc32_combine(mut crc1: uLong, mut crc2: uLong,
                                       mut len2: off_t) -> uLong {
    return crc32_combine_(crc1, crc2, len2);
}
#[no_mangle]
pub unsafe extern "C" fn crc32_combine64(mut crc1: uLong, mut crc2: uLong,
                                         mut len2: off64_t) -> uLong {
    return crc32_combine_(crc1, crc2, len2);
}
