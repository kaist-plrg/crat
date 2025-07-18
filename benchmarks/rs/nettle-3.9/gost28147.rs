use ::libc;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gost28147_param {
    pub sbox: [[uint32_t; 256]; 4],
}
pub static mut _nettle_gost28147_param_test_3411: gost28147_param = {
    let mut init = gost28147_param {
        sbox: [
            [
                0x72000 as libc::c_int as uint32_t,
                0x75000 as libc::c_int as uint32_t,
                0x74800 as libc::c_int as uint32_t,
                0x71000 as libc::c_int as uint32_t,
                0x76800 as libc::c_int as uint32_t,
                0x74000 as libc::c_int as uint32_t,
                0x70000 as libc::c_int as uint32_t,
                0x77000 as libc::c_int as uint32_t,
                0x73000 as libc::c_int as uint32_t,
                0x75800 as libc::c_int as uint32_t,
                0x70800 as libc::c_int as uint32_t,
                0x76000 as libc::c_int as uint32_t,
                0x73800 as libc::c_int as uint32_t,
                0x77800 as libc::c_int as uint32_t,
                0x72800 as libc::c_int as uint32_t,
                0x71800 as libc::c_int as uint32_t,
                0x5a000 as libc::c_int as uint32_t,
                0x5d000 as libc::c_int as uint32_t,
                0x5c800 as libc::c_int as uint32_t,
                0x59000 as libc::c_int as uint32_t,
                0x5e800 as libc::c_int as uint32_t,
                0x5c000 as libc::c_int as uint32_t,
                0x58000 as libc::c_int as uint32_t,
                0x5f000 as libc::c_int as uint32_t,
                0x5b000 as libc::c_int as uint32_t,
                0x5d800 as libc::c_int as uint32_t,
                0x58800 as libc::c_int as uint32_t,
                0x5e000 as libc::c_int as uint32_t,
                0x5b800 as libc::c_int as uint32_t,
                0x5f800 as libc::c_int as uint32_t,
                0x5a800 as libc::c_int as uint32_t,
                0x59800 as libc::c_int as uint32_t,
                0x22000 as libc::c_int as uint32_t,
                0x25000 as libc::c_int as uint32_t,
                0x24800 as libc::c_int as uint32_t,
                0x21000 as libc::c_int as uint32_t,
                0x26800 as libc::c_int as uint32_t,
                0x24000 as libc::c_int as uint32_t,
                0x20000 as libc::c_int as uint32_t,
                0x27000 as libc::c_int as uint32_t,
                0x23000 as libc::c_int as uint32_t,
                0x25800 as libc::c_int as uint32_t,
                0x20800 as libc::c_int as uint32_t,
                0x26000 as libc::c_int as uint32_t,
                0x23800 as libc::c_int as uint32_t,
                0x27800 as libc::c_int as uint32_t,
                0x22800 as libc::c_int as uint32_t,
                0x21800 as libc::c_int as uint32_t,
                0x62000 as libc::c_int as uint32_t,
                0x65000 as libc::c_int as uint32_t,
                0x64800 as libc::c_int as uint32_t,
                0x61000 as libc::c_int as uint32_t,
                0x66800 as libc::c_int as uint32_t,
                0x64000 as libc::c_int as uint32_t,
                0x60000 as libc::c_int as uint32_t,
                0x67000 as libc::c_int as uint32_t,
                0x63000 as libc::c_int as uint32_t,
                0x65800 as libc::c_int as uint32_t,
                0x60800 as libc::c_int as uint32_t,
                0x66000 as libc::c_int as uint32_t,
                0x63800 as libc::c_int as uint32_t,
                0x67800 as libc::c_int as uint32_t,
                0x62800 as libc::c_int as uint32_t,
                0x61800 as libc::c_int as uint32_t,
                0x32000 as libc::c_int as uint32_t,
                0x35000 as libc::c_int as uint32_t,
                0x34800 as libc::c_int as uint32_t,
                0x31000 as libc::c_int as uint32_t,
                0x36800 as libc::c_int as uint32_t,
                0x34000 as libc::c_int as uint32_t,
                0x30000 as libc::c_int as uint32_t,
                0x37000 as libc::c_int as uint32_t,
                0x33000 as libc::c_int as uint32_t,
                0x35800 as libc::c_int as uint32_t,
                0x30800 as libc::c_int as uint32_t,
                0x36000 as libc::c_int as uint32_t,
                0x33800 as libc::c_int as uint32_t,
                0x37800 as libc::c_int as uint32_t,
                0x32800 as libc::c_int as uint32_t,
                0x31800 as libc::c_int as uint32_t,
                0x6a000 as libc::c_int as uint32_t,
                0x6d000 as libc::c_int as uint32_t,
                0x6c800 as libc::c_int as uint32_t,
                0x69000 as libc::c_int as uint32_t,
                0x6e800 as libc::c_int as uint32_t,
                0x6c000 as libc::c_int as uint32_t,
                0x68000 as libc::c_int as uint32_t,
                0x6f000 as libc::c_int as uint32_t,
                0x6b000 as libc::c_int as uint32_t,
                0x6d800 as libc::c_int as uint32_t,
                0x68800 as libc::c_int as uint32_t,
                0x6e000 as libc::c_int as uint32_t,
                0x6b800 as libc::c_int as uint32_t,
                0x6f800 as libc::c_int as uint32_t,
                0x6a800 as libc::c_int as uint32_t,
                0x69800 as libc::c_int as uint32_t,
                0x7a000 as libc::c_int as uint32_t,
                0x7d000 as libc::c_int as uint32_t,
                0x7c800 as libc::c_int as uint32_t,
                0x79000 as libc::c_int as uint32_t,
                0x7e800 as libc::c_int as uint32_t,
                0x7c000 as libc::c_int as uint32_t,
                0x78000 as libc::c_int as uint32_t,
                0x7f000 as libc::c_int as uint32_t,
                0x7b000 as libc::c_int as uint32_t,
                0x7d800 as libc::c_int as uint32_t,
                0x78800 as libc::c_int as uint32_t,
                0x7e000 as libc::c_int as uint32_t,
                0x7b800 as libc::c_int as uint32_t,
                0x7f800 as libc::c_int as uint32_t,
                0x7a800 as libc::c_int as uint32_t,
                0x79800 as libc::c_int as uint32_t,
                0x52000 as libc::c_int as uint32_t,
                0x55000 as libc::c_int as uint32_t,
                0x54800 as libc::c_int as uint32_t,
                0x51000 as libc::c_int as uint32_t,
                0x56800 as libc::c_int as uint32_t,
                0x54000 as libc::c_int as uint32_t,
                0x50000 as libc::c_int as uint32_t,
                0x57000 as libc::c_int as uint32_t,
                0x53000 as libc::c_int as uint32_t,
                0x55800 as libc::c_int as uint32_t,
                0x50800 as libc::c_int as uint32_t,
                0x56000 as libc::c_int as uint32_t,
                0x53800 as libc::c_int as uint32_t,
                0x57800 as libc::c_int as uint32_t,
                0x52800 as libc::c_int as uint32_t,
                0x51800 as libc::c_int as uint32_t,
                0x12000 as libc::c_int as uint32_t,
                0x15000 as libc::c_int as uint32_t,
                0x14800 as libc::c_int as uint32_t,
                0x11000 as libc::c_int as uint32_t,
                0x16800 as libc::c_int as uint32_t,
                0x14000 as libc::c_int as uint32_t,
                0x10000 as libc::c_int as uint32_t,
                0x17000 as libc::c_int as uint32_t,
                0x13000 as libc::c_int as uint32_t,
                0x15800 as libc::c_int as uint32_t,
                0x10800 as libc::c_int as uint32_t,
                0x16000 as libc::c_int as uint32_t,
                0x13800 as libc::c_int as uint32_t,
                0x17800 as libc::c_int as uint32_t,
                0x12800 as libc::c_int as uint32_t,
                0x11800 as libc::c_int as uint32_t,
                0x1a000 as libc::c_int as uint32_t,
                0x1d000 as libc::c_int as uint32_t,
                0x1c800 as libc::c_int as uint32_t,
                0x19000 as libc::c_int as uint32_t,
                0x1e800 as libc::c_int as uint32_t,
                0x1c000 as libc::c_int as uint32_t,
                0x18000 as libc::c_int as uint32_t,
                0x1f000 as libc::c_int as uint32_t,
                0x1b000 as libc::c_int as uint32_t,
                0x1d800 as libc::c_int as uint32_t,
                0x18800 as libc::c_int as uint32_t,
                0x1e000 as libc::c_int as uint32_t,
                0x1b800 as libc::c_int as uint32_t,
                0x1f800 as libc::c_int as uint32_t,
                0x1a800 as libc::c_int as uint32_t,
                0x19800 as libc::c_int as uint32_t,
                0x42000 as libc::c_int as uint32_t,
                0x45000 as libc::c_int as uint32_t,
                0x44800 as libc::c_int as uint32_t,
                0x41000 as libc::c_int as uint32_t,
                0x46800 as libc::c_int as uint32_t,
                0x44000 as libc::c_int as uint32_t,
                0x40000 as libc::c_int as uint32_t,
                0x47000 as libc::c_int as uint32_t,
                0x43000 as libc::c_int as uint32_t,
                0x45800 as libc::c_int as uint32_t,
                0x40800 as libc::c_int as uint32_t,
                0x46000 as libc::c_int as uint32_t,
                0x43800 as libc::c_int as uint32_t,
                0x47800 as libc::c_int as uint32_t,
                0x42800 as libc::c_int as uint32_t,
                0x41800 as libc::c_int as uint32_t,
                0xa000 as libc::c_int as uint32_t,
                0xd000 as libc::c_int as uint32_t,
                0xc800 as libc::c_int as uint32_t,
                0x9000 as libc::c_int as uint32_t,
                0xe800 as libc::c_int as uint32_t,
                0xc000 as libc::c_int as uint32_t,
                0x8000 as libc::c_int as uint32_t,
                0xf000 as libc::c_int as uint32_t,
                0xb000 as libc::c_int as uint32_t,
                0xd800 as libc::c_int as uint32_t,
                0x8800 as libc::c_int as uint32_t,
                0xe000 as libc::c_int as uint32_t,
                0xb800 as libc::c_int as uint32_t,
                0xf800 as libc::c_int as uint32_t,
                0xa800 as libc::c_int as uint32_t,
                0x9800 as libc::c_int as uint32_t,
                0x2000 as libc::c_int as uint32_t,
                0x5000 as libc::c_int as uint32_t,
                0x4800 as libc::c_int as uint32_t,
                0x1000 as libc::c_int as uint32_t,
                0x6800 as libc::c_int as uint32_t,
                0x4000 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
                0x7000 as libc::c_int as uint32_t,
                0x3000 as libc::c_int as uint32_t,
                0x5800 as libc::c_int as uint32_t,
                0x800 as libc::c_int as uint32_t,
                0x6000 as libc::c_int as uint32_t,
                0x3800 as libc::c_int as uint32_t,
                0x7800 as libc::c_int as uint32_t,
                0x2800 as libc::c_int as uint32_t,
                0x1800 as libc::c_int as uint32_t,
                0x3a000 as libc::c_int as uint32_t,
                0x3d000 as libc::c_int as uint32_t,
                0x3c800 as libc::c_int as uint32_t,
                0x39000 as libc::c_int as uint32_t,
                0x3e800 as libc::c_int as uint32_t,
                0x3c000 as libc::c_int as uint32_t,
                0x38000 as libc::c_int as uint32_t,
                0x3f000 as libc::c_int as uint32_t,
                0x3b000 as libc::c_int as uint32_t,
                0x3d800 as libc::c_int as uint32_t,
                0x38800 as libc::c_int as uint32_t,
                0x3e000 as libc::c_int as uint32_t,
                0x3b800 as libc::c_int as uint32_t,
                0x3f800 as libc::c_int as uint32_t,
                0x3a800 as libc::c_int as uint32_t,
                0x39800 as libc::c_int as uint32_t,
                0x2a000 as libc::c_int as uint32_t,
                0x2d000 as libc::c_int as uint32_t,
                0x2c800 as libc::c_int as uint32_t,
                0x29000 as libc::c_int as uint32_t,
                0x2e800 as libc::c_int as uint32_t,
                0x2c000 as libc::c_int as uint32_t,
                0x28000 as libc::c_int as uint32_t,
                0x2f000 as libc::c_int as uint32_t,
                0x2b000 as libc::c_int as uint32_t,
                0x2d800 as libc::c_int as uint32_t,
                0x28800 as libc::c_int as uint32_t,
                0x2e000 as libc::c_int as uint32_t,
                0x2b800 as libc::c_int as uint32_t,
                0x2f800 as libc::c_int as uint32_t,
                0x2a800 as libc::c_int as uint32_t,
                0x29800 as libc::c_int as uint32_t,
                0x4a000 as libc::c_int as uint32_t,
                0x4d000 as libc::c_int as uint32_t,
                0x4c800 as libc::c_int as uint32_t,
                0x49000 as libc::c_int as uint32_t,
                0x4e800 as libc::c_int as uint32_t,
                0x4c000 as libc::c_int as uint32_t,
                0x48000 as libc::c_int as uint32_t,
                0x4f000 as libc::c_int as uint32_t,
                0x4b000 as libc::c_int as uint32_t,
                0x4d800 as libc::c_int as uint32_t,
                0x48800 as libc::c_int as uint32_t,
                0x4e000 as libc::c_int as uint32_t,
                0x4b800 as libc::c_int as uint32_t,
                0x4f800 as libc::c_int as uint32_t,
                0x4a800 as libc::c_int as uint32_t,
                0x49800 as libc::c_int as uint32_t,
            ],
            [
                0x3a80000 as libc::c_int as uint32_t,
                0x3c00000 as libc::c_int as uint32_t,
                0x3880000 as libc::c_int as uint32_t,
                0x3e80000 as libc::c_int as uint32_t,
                0x3d00000 as libc::c_int as uint32_t,
                0x3980000 as libc::c_int as uint32_t,
                0x3a00000 as libc::c_int as uint32_t,
                0x3900000 as libc::c_int as uint32_t,
                0x3f00000 as libc::c_int as uint32_t,
                0x3f80000 as libc::c_int as uint32_t,
                0x3e00000 as libc::c_int as uint32_t,
                0x3b80000 as libc::c_int as uint32_t,
                0x3b00000 as libc::c_int as uint32_t,
                0x3800000 as libc::c_int as uint32_t,
                0x3c80000 as libc::c_int as uint32_t,
                0x3d80000 as libc::c_int as uint32_t,
                0x6a80000 as libc::c_int as uint32_t,
                0x6c00000 as libc::c_int as uint32_t,
                0x6880000 as libc::c_int as uint32_t,
                0x6e80000 as libc::c_int as uint32_t,
                0x6d00000 as libc::c_int as uint32_t,
                0x6980000 as libc::c_int as uint32_t,
                0x6a00000 as libc::c_int as uint32_t,
                0x6900000 as libc::c_int as uint32_t,
                0x6f00000 as libc::c_int as uint32_t,
                0x6f80000 as libc::c_int as uint32_t,
                0x6e00000 as libc::c_int as uint32_t,
                0x6b80000 as libc::c_int as uint32_t,
                0x6b00000 as libc::c_int as uint32_t,
                0x6800000 as libc::c_int as uint32_t,
                0x6c80000 as libc::c_int as uint32_t,
                0x6d80000 as libc::c_int as uint32_t,
                0x5280000 as libc::c_int as uint32_t,
                0x5400000 as libc::c_int as uint32_t,
                0x5080000 as libc::c_int as uint32_t,
                0x5680000 as libc::c_int as uint32_t,
                0x5500000 as libc::c_int as uint32_t,
                0x5180000 as libc::c_int as uint32_t,
                0x5200000 as libc::c_int as uint32_t,
                0x5100000 as libc::c_int as uint32_t,
                0x5700000 as libc::c_int as uint32_t,
                0x5780000 as libc::c_int as uint32_t,
                0x5600000 as libc::c_int as uint32_t,
                0x5380000 as libc::c_int as uint32_t,
                0x5300000 as libc::c_int as uint32_t,
                0x5000000 as libc::c_int as uint32_t,
                0x5480000 as libc::c_int as uint32_t,
                0x5580000 as libc::c_int as uint32_t,
                0xa80000 as libc::c_int as uint32_t,
                0xc00000 as libc::c_int as uint32_t,
                0x880000 as libc::c_int as uint32_t,
                0xe80000 as libc::c_int as uint32_t,
                0xd00000 as libc::c_int as uint32_t,
                0x980000 as libc::c_int as uint32_t,
                0xa00000 as libc::c_int as uint32_t,
                0x900000 as libc::c_int as uint32_t,
                0xf00000 as libc::c_int as uint32_t,
                0xf80000 as libc::c_int as uint32_t,
                0xe00000 as libc::c_int as uint32_t,
                0xb80000 as libc::c_int as uint32_t,
                0xb00000 as libc::c_int as uint32_t,
                0x800000 as libc::c_int as uint32_t,
                0xc80000 as libc::c_int as uint32_t,
                0xd80000 as libc::c_int as uint32_t,
                0x280000 as libc::c_int as uint32_t,
                0x400000 as libc::c_int as uint32_t,
                0x80000 as libc::c_int as uint32_t,
                0x680000 as libc::c_int as uint32_t,
                0x500000 as libc::c_int as uint32_t,
                0x180000 as libc::c_int as uint32_t,
                0x200000 as libc::c_int as uint32_t,
                0x100000 as libc::c_int as uint32_t,
                0x700000 as libc::c_int as uint32_t,
                0x780000 as libc::c_int as uint32_t,
                0x600000 as libc::c_int as uint32_t,
                0x380000 as libc::c_int as uint32_t,
                0x300000 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
                0x480000 as libc::c_int as uint32_t,
                0x580000 as libc::c_int as uint32_t,
                0x4280000 as libc::c_int as uint32_t,
                0x4400000 as libc::c_int as uint32_t,
                0x4080000 as libc::c_int as uint32_t,
                0x4680000 as libc::c_int as uint32_t,
                0x4500000 as libc::c_int as uint32_t,
                0x4180000 as libc::c_int as uint32_t,
                0x4200000 as libc::c_int as uint32_t,
                0x4100000 as libc::c_int as uint32_t,
                0x4700000 as libc::c_int as uint32_t,
                0x4780000 as libc::c_int as uint32_t,
                0x4600000 as libc::c_int as uint32_t,
                0x4380000 as libc::c_int as uint32_t,
                0x4300000 as libc::c_int as uint32_t,
                0x4000000 as libc::c_int as uint32_t,
                0x4480000 as libc::c_int as uint32_t,
                0x4580000 as libc::c_int as uint32_t,
                0x4a80000 as libc::c_int as uint32_t,
                0x4c00000 as libc::c_int as uint32_t,
                0x4880000 as libc::c_int as uint32_t,
                0x4e80000 as libc::c_int as uint32_t,
                0x4d00000 as libc::c_int as uint32_t,
                0x4980000 as libc::c_int as uint32_t,
                0x4a00000 as libc::c_int as uint32_t,
                0x4900000 as libc::c_int as uint32_t,
                0x4f00000 as libc::c_int as uint32_t,
                0x4f80000 as libc::c_int as uint32_t,
                0x4e00000 as libc::c_int as uint32_t,
                0x4b80000 as libc::c_int as uint32_t,
                0x4b00000 as libc::c_int as uint32_t,
                0x4800000 as libc::c_int as uint32_t,
                0x4c80000 as libc::c_int as uint32_t,
                0x4d80000 as libc::c_int as uint32_t,
                0x7a80000 as libc::c_int as uint32_t,
                0x7c00000 as libc::c_int as uint32_t,
                0x7880000 as libc::c_int as uint32_t,
                0x7e80000 as libc::c_int as uint32_t,
                0x7d00000 as libc::c_int as uint32_t,
                0x7980000 as libc::c_int as uint32_t,
                0x7a00000 as libc::c_int as uint32_t,
                0x7900000 as libc::c_int as uint32_t,
                0x7f00000 as libc::c_int as uint32_t,
                0x7f80000 as libc::c_int as uint32_t,
                0x7e00000 as libc::c_int as uint32_t,
                0x7b80000 as libc::c_int as uint32_t,
                0x7b00000 as libc::c_int as uint32_t,
                0x7800000 as libc::c_int as uint32_t,
                0x7c80000 as libc::c_int as uint32_t,
                0x7d80000 as libc::c_int as uint32_t,
                0x7280000 as libc::c_int as uint32_t,
                0x7400000 as libc::c_int as uint32_t,
                0x7080000 as libc::c_int as uint32_t,
                0x7680000 as libc::c_int as uint32_t,
                0x7500000 as libc::c_int as uint32_t,
                0x7180000 as libc::c_int as uint32_t,
                0x7200000 as libc::c_int as uint32_t,
                0x7100000 as libc::c_int as uint32_t,
                0x7700000 as libc::c_int as uint32_t,
                0x7780000 as libc::c_int as uint32_t,
                0x7600000 as libc::c_int as uint32_t,
                0x7380000 as libc::c_int as uint32_t,
                0x7300000 as libc::c_int as uint32_t,
                0x7000000 as libc::c_int as uint32_t,
                0x7480000 as libc::c_int as uint32_t,
                0x7580000 as libc::c_int as uint32_t,
                0x2280000 as libc::c_int as uint32_t,
                0x2400000 as libc::c_int as uint32_t,
                0x2080000 as libc::c_int as uint32_t,
                0x2680000 as libc::c_int as uint32_t,
                0x2500000 as libc::c_int as uint32_t,
                0x2180000 as libc::c_int as uint32_t,
                0x2200000 as libc::c_int as uint32_t,
                0x2100000 as libc::c_int as uint32_t,
                0x2700000 as libc::c_int as uint32_t,
                0x2780000 as libc::c_int as uint32_t,
                0x2600000 as libc::c_int as uint32_t,
                0x2380000 as libc::c_int as uint32_t,
                0x2300000 as libc::c_int as uint32_t,
                0x2000000 as libc::c_int as uint32_t,
                0x2480000 as libc::c_int as uint32_t,
                0x2580000 as libc::c_int as uint32_t,
                0x3280000 as libc::c_int as uint32_t,
                0x3400000 as libc::c_int as uint32_t,
                0x3080000 as libc::c_int as uint32_t,
                0x3680000 as libc::c_int as uint32_t,
                0x3500000 as libc::c_int as uint32_t,
                0x3180000 as libc::c_int as uint32_t,
                0x3200000 as libc::c_int as uint32_t,
                0x3100000 as libc::c_int as uint32_t,
                0x3700000 as libc::c_int as uint32_t,
                0x3780000 as libc::c_int as uint32_t,
                0x3600000 as libc::c_int as uint32_t,
                0x3380000 as libc::c_int as uint32_t,
                0x3300000 as libc::c_int as uint32_t,
                0x3000000 as libc::c_int as uint32_t,
                0x3480000 as libc::c_int as uint32_t,
                0x3580000 as libc::c_int as uint32_t,
                0x6280000 as libc::c_int as uint32_t,
                0x6400000 as libc::c_int as uint32_t,
                0x6080000 as libc::c_int as uint32_t,
                0x6680000 as libc::c_int as uint32_t,
                0x6500000 as libc::c_int as uint32_t,
                0x6180000 as libc::c_int as uint32_t,
                0x6200000 as libc::c_int as uint32_t,
                0x6100000 as libc::c_int as uint32_t,
                0x6700000 as libc::c_int as uint32_t,
                0x6780000 as libc::c_int as uint32_t,
                0x6600000 as libc::c_int as uint32_t,
                0x6380000 as libc::c_int as uint32_t,
                0x6300000 as libc::c_int as uint32_t,
                0x6000000 as libc::c_int as uint32_t,
                0x6480000 as libc::c_int as uint32_t,
                0x6580000 as libc::c_int as uint32_t,
                0x5a80000 as libc::c_int as uint32_t,
                0x5c00000 as libc::c_int as uint32_t,
                0x5880000 as libc::c_int as uint32_t,
                0x5e80000 as libc::c_int as uint32_t,
                0x5d00000 as libc::c_int as uint32_t,
                0x5980000 as libc::c_int as uint32_t,
                0x5a00000 as libc::c_int as uint32_t,
                0x5900000 as libc::c_int as uint32_t,
                0x5f00000 as libc::c_int as uint32_t,
                0x5f80000 as libc::c_int as uint32_t,
                0x5e00000 as libc::c_int as uint32_t,
                0x5b80000 as libc::c_int as uint32_t,
                0x5b00000 as libc::c_int as uint32_t,
                0x5800000 as libc::c_int as uint32_t,
                0x5c80000 as libc::c_int as uint32_t,
                0x5d80000 as libc::c_int as uint32_t,
                0x1280000 as libc::c_int as uint32_t,
                0x1400000 as libc::c_int as uint32_t,
                0x1080000 as libc::c_int as uint32_t,
                0x1680000 as libc::c_int as uint32_t,
                0x1500000 as libc::c_int as uint32_t,
                0x1180000 as libc::c_int as uint32_t,
                0x1200000 as libc::c_int as uint32_t,
                0x1100000 as libc::c_int as uint32_t,
                0x1700000 as libc::c_int as uint32_t,
                0x1780000 as libc::c_int as uint32_t,
                0x1600000 as libc::c_int as uint32_t,
                0x1380000 as libc::c_int as uint32_t,
                0x1300000 as libc::c_int as uint32_t,
                0x1000000 as libc::c_int as uint32_t,
                0x1480000 as libc::c_int as uint32_t,
                0x1580000 as libc::c_int as uint32_t,
                0x2a80000 as libc::c_int as uint32_t,
                0x2c00000 as libc::c_int as uint32_t,
                0x2880000 as libc::c_int as uint32_t,
                0x2e80000 as libc::c_int as uint32_t,
                0x2d00000 as libc::c_int as uint32_t,
                0x2980000 as libc::c_int as uint32_t,
                0x2a00000 as libc::c_int as uint32_t,
                0x2900000 as libc::c_int as uint32_t,
                0x2f00000 as libc::c_int as uint32_t,
                0x2f80000 as libc::c_int as uint32_t,
                0x2e00000 as libc::c_int as uint32_t,
                0x2b80000 as libc::c_int as uint32_t,
                0x2b00000 as libc::c_int as uint32_t,
                0x2800000 as libc::c_int as uint32_t,
                0x2c80000 as libc::c_int as uint32_t,
                0x2d80000 as libc::c_int as uint32_t,
                0x1a80000 as libc::c_int as uint32_t,
                0x1c00000 as libc::c_int as uint32_t,
                0x1880000 as libc::c_int as uint32_t,
                0x1e80000 as libc::c_int as uint32_t,
                0x1d00000 as libc::c_int as uint32_t,
                0x1980000 as libc::c_int as uint32_t,
                0x1a00000 as libc::c_int as uint32_t,
                0x1900000 as libc::c_int as uint32_t,
                0x1f00000 as libc::c_int as uint32_t,
                0x1f80000 as libc::c_int as uint32_t,
                0x1e00000 as libc::c_int as uint32_t,
                0x1b80000 as libc::c_int as uint32_t,
                0x1b00000 as libc::c_int as uint32_t,
                0x1800000 as libc::c_int as uint32_t,
                0x1c80000 as libc::c_int as uint32_t,
                0x1d80000 as libc::c_int as uint32_t,
            ],
            [
                0x30000002 as libc::c_int as uint32_t,
                0x60000002 as libc::c_int as uint32_t,
                0x38000002 as libc::c_int as uint32_t,
                0x8000002 as libc::c_int as uint32_t,
                0x28000002 as libc::c_int as uint32_t,
                0x78000002 as libc::c_int as uint32_t,
                0x68000002 as libc::c_int as uint32_t,
                0x40000002 as libc::c_int as uint32_t,
                0x20000002 as libc::c_int as uint32_t,
                0x50000002 as libc::c_int as uint32_t,
                0x48000002 as libc::c_int as uint32_t,
                0x70000002 as libc::c_int as uint32_t,
                0x2 as libc::c_int as uint32_t,
                0x18000002 as libc::c_int as uint32_t,
                0x58000002 as libc::c_int as uint32_t,
                0x10000002 as libc::c_int as uint32_t,
                0xb0000005 as libc::c_uint,
                0xe0000005 as libc::c_uint,
                0xb8000005 as libc::c_uint,
                0x88000005 as libc::c_uint,
                0xa8000005 as libc::c_uint,
                0xf8000005 as libc::c_uint,
                0xe8000005 as libc::c_uint,
                0xc0000005 as libc::c_uint,
                0xa0000005 as libc::c_uint,
                0xd0000005 as libc::c_uint,
                0xc8000005 as libc::c_uint,
                0xf0000005 as libc::c_uint,
                0x80000005 as libc::c_uint,
                0x98000005 as libc::c_uint,
                0xd8000005 as libc::c_uint,
                0x90000005 as libc::c_uint,
                0x30000005 as libc::c_int as uint32_t,
                0x60000005 as libc::c_int as uint32_t,
                0x38000005 as libc::c_int as uint32_t,
                0x8000005 as libc::c_int as uint32_t,
                0x28000005 as libc::c_int as uint32_t,
                0x78000005 as libc::c_int as uint32_t,
                0x68000005 as libc::c_int as uint32_t,
                0x40000005 as libc::c_int as uint32_t,
                0x20000005 as libc::c_int as uint32_t,
                0x50000005 as libc::c_int as uint32_t,
                0x48000005 as libc::c_int as uint32_t,
                0x70000005 as libc::c_int as uint32_t,
                0x5 as libc::c_int as uint32_t,
                0x18000005 as libc::c_int as uint32_t,
                0x58000005 as libc::c_int as uint32_t,
                0x10000005 as libc::c_int as uint32_t,
                0x30000000 as libc::c_int as uint32_t,
                0x60000000 as libc::c_int as uint32_t,
                0x38000000 as libc::c_int as uint32_t,
                0x8000000 as libc::c_int as uint32_t,
                0x28000000 as libc::c_int as uint32_t,
                0x78000000 as libc::c_int as uint32_t,
                0x68000000 as libc::c_int as uint32_t,
                0x40000000 as libc::c_int as uint32_t,
                0x20000000 as libc::c_int as uint32_t,
                0x50000000 as libc::c_int as uint32_t,
                0x48000000 as libc::c_int as uint32_t,
                0x70000000 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
                0x18000000 as libc::c_int as uint32_t,
                0x58000000 as libc::c_int as uint32_t,
                0x10000000 as libc::c_int as uint32_t,
                0xb0000003 as libc::c_uint,
                0xe0000003 as libc::c_uint,
                0xb8000003 as libc::c_uint,
                0x88000003 as libc::c_uint,
                0xa8000003 as libc::c_uint,
                0xf8000003 as libc::c_uint,
                0xe8000003 as libc::c_uint,
                0xc0000003 as libc::c_uint,
                0xa0000003 as libc::c_uint,
                0xd0000003 as libc::c_uint,
                0xc8000003 as libc::c_uint,
                0xf0000003 as libc::c_uint,
                0x80000003 as libc::c_uint,
                0x98000003 as libc::c_uint,
                0xd8000003 as libc::c_uint,
                0x90000003 as libc::c_uint,
                0x30000001 as libc::c_int as uint32_t,
                0x60000001 as libc::c_int as uint32_t,
                0x38000001 as libc::c_int as uint32_t,
                0x8000001 as libc::c_int as uint32_t,
                0x28000001 as libc::c_int as uint32_t,
                0x78000001 as libc::c_int as uint32_t,
                0x68000001 as libc::c_int as uint32_t,
                0x40000001 as libc::c_int as uint32_t,
                0x20000001 as libc::c_int as uint32_t,
                0x50000001 as libc::c_int as uint32_t,
                0x48000001 as libc::c_int as uint32_t,
                0x70000001 as libc::c_int as uint32_t,
                0x1 as libc::c_int as uint32_t,
                0x18000001 as libc::c_int as uint32_t,
                0x58000001 as libc::c_int as uint32_t,
                0x10000001 as libc::c_int as uint32_t,
                0xb0000000 as libc::c_uint,
                0xe0000000 as libc::c_uint,
                0xb8000000 as libc::c_uint,
                0x88000000 as libc::c_uint,
                0xa8000000 as libc::c_uint,
                0xf8000000 as libc::c_uint,
                0xe8000000 as libc::c_uint,
                0xc0000000 as libc::c_uint,
                0xa0000000 as libc::c_uint,
                0xd0000000 as libc::c_uint,
                0xc8000000 as libc::c_uint,
                0xf0000000 as libc::c_uint,
                0x80000000 as libc::c_uint,
                0x98000000 as libc::c_uint,
                0xd8000000 as libc::c_uint,
                0x90000000 as libc::c_uint,
                0xb0000006 as libc::c_uint,
                0xe0000006 as libc::c_uint,
                0xb8000006 as libc::c_uint,
                0x88000006 as libc::c_uint,
                0xa8000006 as libc::c_uint,
                0xf8000006 as libc::c_uint,
                0xe8000006 as libc::c_uint,
                0xc0000006 as libc::c_uint,
                0xa0000006 as libc::c_uint,
                0xd0000006 as libc::c_uint,
                0xc8000006 as libc::c_uint,
                0xf0000006 as libc::c_uint,
                0x80000006 as libc::c_uint,
                0x98000006 as libc::c_uint,
                0xd8000006 as libc::c_uint,
                0x90000006 as libc::c_uint,
                0xb0000001 as libc::c_uint,
                0xe0000001 as libc::c_uint,
                0xb8000001 as libc::c_uint,
                0x88000001 as libc::c_uint,
                0xa8000001 as libc::c_uint,
                0xf8000001 as libc::c_uint,
                0xe8000001 as libc::c_uint,
                0xc0000001 as libc::c_uint,
                0xa0000001 as libc::c_uint,
                0xd0000001 as libc::c_uint,
                0xc8000001 as libc::c_uint,
                0xf0000001 as libc::c_uint,
                0x80000001 as libc::c_uint,
                0x98000001 as libc::c_uint,
                0xd8000001 as libc::c_uint,
                0x90000001 as libc::c_uint,
                0x30000003 as libc::c_int as uint32_t,
                0x60000003 as libc::c_int as uint32_t,
                0x38000003 as libc::c_int as uint32_t,
                0x8000003 as libc::c_int as uint32_t,
                0x28000003 as libc::c_int as uint32_t,
                0x78000003 as libc::c_int as uint32_t,
                0x68000003 as libc::c_int as uint32_t,
                0x40000003 as libc::c_int as uint32_t,
                0x20000003 as libc::c_int as uint32_t,
                0x50000003 as libc::c_int as uint32_t,
                0x48000003 as libc::c_int as uint32_t,
                0x70000003 as libc::c_int as uint32_t,
                0x3 as libc::c_int as uint32_t,
                0x18000003 as libc::c_int as uint32_t,
                0x58000003 as libc::c_int as uint32_t,
                0x10000003 as libc::c_int as uint32_t,
                0x30000004 as libc::c_int as uint32_t,
                0x60000004 as libc::c_int as uint32_t,
                0x38000004 as libc::c_int as uint32_t,
                0x8000004 as libc::c_int as uint32_t,
                0x28000004 as libc::c_int as uint32_t,
                0x78000004 as libc::c_int as uint32_t,
                0x68000004 as libc::c_int as uint32_t,
                0x40000004 as libc::c_int as uint32_t,
                0x20000004 as libc::c_int as uint32_t,
                0x50000004 as libc::c_int as uint32_t,
                0x48000004 as libc::c_int as uint32_t,
                0x70000004 as libc::c_int as uint32_t,
                0x4 as libc::c_int as uint32_t,
                0x18000004 as libc::c_int as uint32_t,
                0x58000004 as libc::c_int as uint32_t,
                0x10000004 as libc::c_int as uint32_t,
                0xb0000002 as libc::c_uint,
                0xe0000002 as libc::c_uint,
                0xb8000002 as libc::c_uint,
                0x88000002 as libc::c_uint,
                0xa8000002 as libc::c_uint,
                0xf8000002 as libc::c_uint,
                0xe8000002 as libc::c_uint,
                0xc0000002 as libc::c_uint,
                0xa0000002 as libc::c_uint,
                0xd0000002 as libc::c_uint,
                0xc8000002 as libc::c_uint,
                0xf0000002 as libc::c_uint,
                0x80000002 as libc::c_uint,
                0x98000002 as libc::c_uint,
                0xd8000002 as libc::c_uint,
                0x90000002 as libc::c_uint,
                0xb0000004 as libc::c_uint,
                0xe0000004 as libc::c_uint,
                0xb8000004 as libc::c_uint,
                0x88000004 as libc::c_uint,
                0xa8000004 as libc::c_uint,
                0xf8000004 as libc::c_uint,
                0xe8000004 as libc::c_uint,
                0xc0000004 as libc::c_uint,
                0xa0000004 as libc::c_uint,
                0xd0000004 as libc::c_uint,
                0xc8000004 as libc::c_uint,
                0xf0000004 as libc::c_uint,
                0x80000004 as libc::c_uint,
                0x98000004 as libc::c_uint,
                0xd8000004 as libc::c_uint,
                0x90000004 as libc::c_uint,
                0x30000006 as libc::c_int as uint32_t,
                0x60000006 as libc::c_int as uint32_t,
                0x38000006 as libc::c_int as uint32_t,
                0x8000006 as libc::c_int as uint32_t,
                0x28000006 as libc::c_int as uint32_t,
                0x78000006 as libc::c_int as uint32_t,
                0x68000006 as libc::c_int as uint32_t,
                0x40000006 as libc::c_int as uint32_t,
                0x20000006 as libc::c_int as uint32_t,
                0x50000006 as libc::c_int as uint32_t,
                0x48000006 as libc::c_int as uint32_t,
                0x70000006 as libc::c_int as uint32_t,
                0x6 as libc::c_int as uint32_t,
                0x18000006 as libc::c_int as uint32_t,
                0x58000006 as libc::c_int as uint32_t,
                0x10000006 as libc::c_int as uint32_t,
                0xb0000007 as libc::c_uint,
                0xe0000007 as libc::c_uint,
                0xb8000007 as libc::c_uint,
                0x88000007 as libc::c_uint,
                0xa8000007 as libc::c_uint,
                0xf8000007 as libc::c_uint,
                0xe8000007 as libc::c_uint,
                0xc0000007 as libc::c_uint,
                0xa0000007 as libc::c_uint,
                0xd0000007 as libc::c_uint,
                0xc8000007 as libc::c_uint,
                0xf0000007 as libc::c_uint,
                0x80000007 as libc::c_uint,
                0x98000007 as libc::c_uint,
                0xd8000007 as libc::c_uint,
                0x90000007 as libc::c_uint,
                0x30000007 as libc::c_int as uint32_t,
                0x60000007 as libc::c_int as uint32_t,
                0x38000007 as libc::c_int as uint32_t,
                0x8000007 as libc::c_int as uint32_t,
                0x28000007 as libc::c_int as uint32_t,
                0x78000007 as libc::c_int as uint32_t,
                0x68000007 as libc::c_int as uint32_t,
                0x40000007 as libc::c_int as uint32_t,
                0x20000007 as libc::c_int as uint32_t,
                0x50000007 as libc::c_int as uint32_t,
                0x48000007 as libc::c_int as uint32_t,
                0x70000007 as libc::c_int as uint32_t,
                0x7 as libc::c_int as uint32_t,
                0x18000007 as libc::c_int as uint32_t,
                0x58000007 as libc::c_int as uint32_t,
                0x10000007 as libc::c_int as uint32_t,
            ],
            [
                0xe8 as libc::c_int as uint32_t,
                0xd8 as libc::c_int as uint32_t,
                0xa0 as libc::c_int as uint32_t,
                0x88 as libc::c_int as uint32_t,
                0x98 as libc::c_int as uint32_t,
                0xf8 as libc::c_int as uint32_t,
                0xa8 as libc::c_int as uint32_t,
                0xc8 as libc::c_int as uint32_t,
                0x80 as libc::c_int as uint32_t,
                0xd0 as libc::c_int as uint32_t,
                0xf0 as libc::c_int as uint32_t,
                0xb8 as libc::c_int as uint32_t,
                0xb0 as libc::c_int as uint32_t,
                0xc0 as libc::c_int as uint32_t,
                0x90 as libc::c_int as uint32_t,
                0xe0 as libc::c_int as uint32_t,
                0x7e8 as libc::c_int as uint32_t,
                0x7d8 as libc::c_int as uint32_t,
                0x7a0 as libc::c_int as uint32_t,
                0x788 as libc::c_int as uint32_t,
                0x798 as libc::c_int as uint32_t,
                0x7f8 as libc::c_int as uint32_t,
                0x7a8 as libc::c_int as uint32_t,
                0x7c8 as libc::c_int as uint32_t,
                0x780 as libc::c_int as uint32_t,
                0x7d0 as libc::c_int as uint32_t,
                0x7f0 as libc::c_int as uint32_t,
                0x7b8 as libc::c_int as uint32_t,
                0x7b0 as libc::c_int as uint32_t,
                0x7c0 as libc::c_int as uint32_t,
                0x790 as libc::c_int as uint32_t,
                0x7e0 as libc::c_int as uint32_t,
                0x6e8 as libc::c_int as uint32_t,
                0x6d8 as libc::c_int as uint32_t,
                0x6a0 as libc::c_int as uint32_t,
                0x688 as libc::c_int as uint32_t,
                0x698 as libc::c_int as uint32_t,
                0x6f8 as libc::c_int as uint32_t,
                0x6a8 as libc::c_int as uint32_t,
                0x6c8 as libc::c_int as uint32_t,
                0x680 as libc::c_int as uint32_t,
                0x6d0 as libc::c_int as uint32_t,
                0x6f0 as libc::c_int as uint32_t,
                0x6b8 as libc::c_int as uint32_t,
                0x6b0 as libc::c_int as uint32_t,
                0x6c0 as libc::c_int as uint32_t,
                0x690 as libc::c_int as uint32_t,
                0x6e0 as libc::c_int as uint32_t,
                0x68 as libc::c_int as uint32_t,
                0x58 as libc::c_int as uint32_t,
                0x20 as libc::c_int as uint32_t,
                0x8 as libc::c_int as uint32_t,
                0x18 as libc::c_int as uint32_t,
                0x78 as libc::c_int as uint32_t,
                0x28 as libc::c_int as uint32_t,
                0x48 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
                0x50 as libc::c_int as uint32_t,
                0x70 as libc::c_int as uint32_t,
                0x38 as libc::c_int as uint32_t,
                0x30 as libc::c_int as uint32_t,
                0x40 as libc::c_int as uint32_t,
                0x10 as libc::c_int as uint32_t,
                0x60 as libc::c_int as uint32_t,
                0x2e8 as libc::c_int as uint32_t,
                0x2d8 as libc::c_int as uint32_t,
                0x2a0 as libc::c_int as uint32_t,
                0x288 as libc::c_int as uint32_t,
                0x298 as libc::c_int as uint32_t,
                0x2f8 as libc::c_int as uint32_t,
                0x2a8 as libc::c_int as uint32_t,
                0x2c8 as libc::c_int as uint32_t,
                0x280 as libc::c_int as uint32_t,
                0x2d0 as libc::c_int as uint32_t,
                0x2f0 as libc::c_int as uint32_t,
                0x2b8 as libc::c_int as uint32_t,
                0x2b0 as libc::c_int as uint32_t,
                0x2c0 as libc::c_int as uint32_t,
                0x290 as libc::c_int as uint32_t,
                0x2e0 as libc::c_int as uint32_t,
                0x3e8 as libc::c_int as uint32_t,
                0x3d8 as libc::c_int as uint32_t,
                0x3a0 as libc::c_int as uint32_t,
                0x388 as libc::c_int as uint32_t,
                0x398 as libc::c_int as uint32_t,
                0x3f8 as libc::c_int as uint32_t,
                0x3a8 as libc::c_int as uint32_t,
                0x3c8 as libc::c_int as uint32_t,
                0x380 as libc::c_int as uint32_t,
                0x3d0 as libc::c_int as uint32_t,
                0x3f0 as libc::c_int as uint32_t,
                0x3b8 as libc::c_int as uint32_t,
                0x3b0 as libc::c_int as uint32_t,
                0x3c0 as libc::c_int as uint32_t,
                0x390 as libc::c_int as uint32_t,
                0x3e0 as libc::c_int as uint32_t,
                0x568 as libc::c_int as uint32_t,
                0x558 as libc::c_int as uint32_t,
                0x520 as libc::c_int as uint32_t,
                0x508 as libc::c_int as uint32_t,
                0x518 as libc::c_int as uint32_t,
                0x578 as libc::c_int as uint32_t,
                0x528 as libc::c_int as uint32_t,
                0x548 as libc::c_int as uint32_t,
                0x500 as libc::c_int as uint32_t,
                0x550 as libc::c_int as uint32_t,
                0x570 as libc::c_int as uint32_t,
                0x538 as libc::c_int as uint32_t,
                0x530 as libc::c_int as uint32_t,
                0x540 as libc::c_int as uint32_t,
                0x510 as libc::c_int as uint32_t,
                0x560 as libc::c_int as uint32_t,
                0x268 as libc::c_int as uint32_t,
                0x258 as libc::c_int as uint32_t,
                0x220 as libc::c_int as uint32_t,
                0x208 as libc::c_int as uint32_t,
                0x218 as libc::c_int as uint32_t,
                0x278 as libc::c_int as uint32_t,
                0x228 as libc::c_int as uint32_t,
                0x248 as libc::c_int as uint32_t,
                0x200 as libc::c_int as uint32_t,
                0x250 as libc::c_int as uint32_t,
                0x270 as libc::c_int as uint32_t,
                0x238 as libc::c_int as uint32_t,
                0x230 as libc::c_int as uint32_t,
                0x240 as libc::c_int as uint32_t,
                0x210 as libc::c_int as uint32_t,
                0x260 as libc::c_int as uint32_t,
                0x4e8 as libc::c_int as uint32_t,
                0x4d8 as libc::c_int as uint32_t,
                0x4a0 as libc::c_int as uint32_t,
                0x488 as libc::c_int as uint32_t,
                0x498 as libc::c_int as uint32_t,
                0x4f8 as libc::c_int as uint32_t,
                0x4a8 as libc::c_int as uint32_t,
                0x4c8 as libc::c_int as uint32_t,
                0x480 as libc::c_int as uint32_t,
                0x4d0 as libc::c_int as uint32_t,
                0x4f0 as libc::c_int as uint32_t,
                0x4b8 as libc::c_int as uint32_t,
                0x4b0 as libc::c_int as uint32_t,
                0x4c0 as libc::c_int as uint32_t,
                0x490 as libc::c_int as uint32_t,
                0x4e0 as libc::c_int as uint32_t,
                0x168 as libc::c_int as uint32_t,
                0x158 as libc::c_int as uint32_t,
                0x120 as libc::c_int as uint32_t,
                0x108 as libc::c_int as uint32_t,
                0x118 as libc::c_int as uint32_t,
                0x178 as libc::c_int as uint32_t,
                0x128 as libc::c_int as uint32_t,
                0x148 as libc::c_int as uint32_t,
                0x100 as libc::c_int as uint32_t,
                0x150 as libc::c_int as uint32_t,
                0x170 as libc::c_int as uint32_t,
                0x138 as libc::c_int as uint32_t,
                0x130 as libc::c_int as uint32_t,
                0x140 as libc::c_int as uint32_t,
                0x110 as libc::c_int as uint32_t,
                0x160 as libc::c_int as uint32_t,
                0x1e8 as libc::c_int as uint32_t,
                0x1d8 as libc::c_int as uint32_t,
                0x1a0 as libc::c_int as uint32_t,
                0x188 as libc::c_int as uint32_t,
                0x198 as libc::c_int as uint32_t,
                0x1f8 as libc::c_int as uint32_t,
                0x1a8 as libc::c_int as uint32_t,
                0x1c8 as libc::c_int as uint32_t,
                0x180 as libc::c_int as uint32_t,
                0x1d0 as libc::c_int as uint32_t,
                0x1f0 as libc::c_int as uint32_t,
                0x1b8 as libc::c_int as uint32_t,
                0x1b0 as libc::c_int as uint32_t,
                0x1c0 as libc::c_int as uint32_t,
                0x190 as libc::c_int as uint32_t,
                0x1e0 as libc::c_int as uint32_t,
                0x768 as libc::c_int as uint32_t,
                0x758 as libc::c_int as uint32_t,
                0x720 as libc::c_int as uint32_t,
                0x708 as libc::c_int as uint32_t,
                0x718 as libc::c_int as uint32_t,
                0x778 as libc::c_int as uint32_t,
                0x728 as libc::c_int as uint32_t,
                0x748 as libc::c_int as uint32_t,
                0x700 as libc::c_int as uint32_t,
                0x750 as libc::c_int as uint32_t,
                0x770 as libc::c_int as uint32_t,
                0x738 as libc::c_int as uint32_t,
                0x730 as libc::c_int as uint32_t,
                0x740 as libc::c_int as uint32_t,
                0x710 as libc::c_int as uint32_t,
                0x760 as libc::c_int as uint32_t,
                0x368 as libc::c_int as uint32_t,
                0x358 as libc::c_int as uint32_t,
                0x320 as libc::c_int as uint32_t,
                0x308 as libc::c_int as uint32_t,
                0x318 as libc::c_int as uint32_t,
                0x378 as libc::c_int as uint32_t,
                0x328 as libc::c_int as uint32_t,
                0x348 as libc::c_int as uint32_t,
                0x300 as libc::c_int as uint32_t,
                0x350 as libc::c_int as uint32_t,
                0x370 as libc::c_int as uint32_t,
                0x338 as libc::c_int as uint32_t,
                0x330 as libc::c_int as uint32_t,
                0x340 as libc::c_int as uint32_t,
                0x310 as libc::c_int as uint32_t,
                0x360 as libc::c_int as uint32_t,
                0x5e8 as libc::c_int as uint32_t,
                0x5d8 as libc::c_int as uint32_t,
                0x5a0 as libc::c_int as uint32_t,
                0x588 as libc::c_int as uint32_t,
                0x598 as libc::c_int as uint32_t,
                0x5f8 as libc::c_int as uint32_t,
                0x5a8 as libc::c_int as uint32_t,
                0x5c8 as libc::c_int as uint32_t,
                0x580 as libc::c_int as uint32_t,
                0x5d0 as libc::c_int as uint32_t,
                0x5f0 as libc::c_int as uint32_t,
                0x5b8 as libc::c_int as uint32_t,
                0x5b0 as libc::c_int as uint32_t,
                0x5c0 as libc::c_int as uint32_t,
                0x590 as libc::c_int as uint32_t,
                0x5e0 as libc::c_int as uint32_t,
                0x468 as libc::c_int as uint32_t,
                0x458 as libc::c_int as uint32_t,
                0x420 as libc::c_int as uint32_t,
                0x408 as libc::c_int as uint32_t,
                0x418 as libc::c_int as uint32_t,
                0x478 as libc::c_int as uint32_t,
                0x428 as libc::c_int as uint32_t,
                0x448 as libc::c_int as uint32_t,
                0x400 as libc::c_int as uint32_t,
                0x450 as libc::c_int as uint32_t,
                0x470 as libc::c_int as uint32_t,
                0x438 as libc::c_int as uint32_t,
                0x430 as libc::c_int as uint32_t,
                0x440 as libc::c_int as uint32_t,
                0x410 as libc::c_int as uint32_t,
                0x460 as libc::c_int as uint32_t,
                0x668 as libc::c_int as uint32_t,
                0x658 as libc::c_int as uint32_t,
                0x620 as libc::c_int as uint32_t,
                0x608 as libc::c_int as uint32_t,
                0x618 as libc::c_int as uint32_t,
                0x678 as libc::c_int as uint32_t,
                0x628 as libc::c_int as uint32_t,
                0x648 as libc::c_int as uint32_t,
                0x600 as libc::c_int as uint32_t,
                0x650 as libc::c_int as uint32_t,
                0x670 as libc::c_int as uint32_t,
                0x638 as libc::c_int as uint32_t,
                0x630 as libc::c_int as uint32_t,
                0x640 as libc::c_int as uint32_t,
                0x610 as libc::c_int as uint32_t,
                0x660 as libc::c_int as uint32_t,
            ],
        ],
    };
    init
};
pub static mut _nettle_gost28147_param_CryptoPro_3411: gost28147_param = {
    let mut init = gost28147_param {
        sbox: [
            [
                0x2d000 as libc::c_int as uint32_t,
                0x2a000 as libc::c_int as uint32_t,
                0x2a800 as libc::c_int as uint32_t,
                0x2b000 as libc::c_int as uint32_t,
                0x2c000 as libc::c_int as uint32_t,
                0x28800 as libc::c_int as uint32_t,
                0x29800 as libc::c_int as uint32_t,
                0x2b800 as libc::c_int as uint32_t,
                0x2e800 as libc::c_int as uint32_t,
                0x2e000 as libc::c_int as uint32_t,
                0x2f000 as libc::c_int as uint32_t,
                0x28000 as libc::c_int as uint32_t,
                0x2c800 as libc::c_int as uint32_t,
                0x29000 as libc::c_int as uint32_t,
                0x2d800 as libc::c_int as uint32_t,
                0x2f800 as libc::c_int as uint32_t,
                0x7d000 as libc::c_int as uint32_t,
                0x7a000 as libc::c_int as uint32_t,
                0x7a800 as libc::c_int as uint32_t,
                0x7b000 as libc::c_int as uint32_t,
                0x7c000 as libc::c_int as uint32_t,
                0x78800 as libc::c_int as uint32_t,
                0x79800 as libc::c_int as uint32_t,
                0x7b800 as libc::c_int as uint32_t,
                0x7e800 as libc::c_int as uint32_t,
                0x7e000 as libc::c_int as uint32_t,
                0x7f000 as libc::c_int as uint32_t,
                0x78000 as libc::c_int as uint32_t,
                0x7c800 as libc::c_int as uint32_t,
                0x79000 as libc::c_int as uint32_t,
                0x7d800 as libc::c_int as uint32_t,
                0x7f800 as libc::c_int as uint32_t,
                0x25000 as libc::c_int as uint32_t,
                0x22000 as libc::c_int as uint32_t,
                0x22800 as libc::c_int as uint32_t,
                0x23000 as libc::c_int as uint32_t,
                0x24000 as libc::c_int as uint32_t,
                0x20800 as libc::c_int as uint32_t,
                0x21800 as libc::c_int as uint32_t,
                0x23800 as libc::c_int as uint32_t,
                0x26800 as libc::c_int as uint32_t,
                0x26000 as libc::c_int as uint32_t,
                0x27000 as libc::c_int as uint32_t,
                0x20000 as libc::c_int as uint32_t,
                0x24800 as libc::c_int as uint32_t,
                0x21000 as libc::c_int as uint32_t,
                0x25800 as libc::c_int as uint32_t,
                0x27800 as libc::c_int as uint32_t,
                0x5000 as libc::c_int as uint32_t,
                0x2000 as libc::c_int as uint32_t,
                0x2800 as libc::c_int as uint32_t,
                0x3000 as libc::c_int as uint32_t,
                0x4000 as libc::c_int as uint32_t,
                0x800 as libc::c_int as uint32_t,
                0x1800 as libc::c_int as uint32_t,
                0x3800 as libc::c_int as uint32_t,
                0x6800 as libc::c_int as uint32_t,
                0x6000 as libc::c_int as uint32_t,
                0x7000 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
                0x4800 as libc::c_int as uint32_t,
                0x1000 as libc::c_int as uint32_t,
                0x5800 as libc::c_int as uint32_t,
                0x7800 as libc::c_int as uint32_t,
                0x15000 as libc::c_int as uint32_t,
                0x12000 as libc::c_int as uint32_t,
                0x12800 as libc::c_int as uint32_t,
                0x13000 as libc::c_int as uint32_t,
                0x14000 as libc::c_int as uint32_t,
                0x10800 as libc::c_int as uint32_t,
                0x11800 as libc::c_int as uint32_t,
                0x13800 as libc::c_int as uint32_t,
                0x16800 as libc::c_int as uint32_t,
                0x16000 as libc::c_int as uint32_t,
                0x17000 as libc::c_int as uint32_t,
                0x10000 as libc::c_int as uint32_t,
                0x14800 as libc::c_int as uint32_t,
                0x11000 as libc::c_int as uint32_t,
                0x15800 as libc::c_int as uint32_t,
                0x17800 as libc::c_int as uint32_t,
                0x6d000 as libc::c_int as uint32_t,
                0x6a000 as libc::c_int as uint32_t,
                0x6a800 as libc::c_int as uint32_t,
                0x6b000 as libc::c_int as uint32_t,
                0x6c000 as libc::c_int as uint32_t,
                0x68800 as libc::c_int as uint32_t,
                0x69800 as libc::c_int as uint32_t,
                0x6b800 as libc::c_int as uint32_t,
                0x6e800 as libc::c_int as uint32_t,
                0x6e000 as libc::c_int as uint32_t,
                0x6f000 as libc::c_int as uint32_t,
                0x68000 as libc::c_int as uint32_t,
                0x6c800 as libc::c_int as uint32_t,
                0x69000 as libc::c_int as uint32_t,
                0x6d800 as libc::c_int as uint32_t,
                0x6f800 as libc::c_int as uint32_t,
                0x5d000 as libc::c_int as uint32_t,
                0x5a000 as libc::c_int as uint32_t,
                0x5a800 as libc::c_int as uint32_t,
                0x5b000 as libc::c_int as uint32_t,
                0x5c000 as libc::c_int as uint32_t,
                0x58800 as libc::c_int as uint32_t,
                0x59800 as libc::c_int as uint32_t,
                0x5b800 as libc::c_int as uint32_t,
                0x5e800 as libc::c_int as uint32_t,
                0x5e000 as libc::c_int as uint32_t,
                0x5f000 as libc::c_int as uint32_t,
                0x58000 as libc::c_int as uint32_t,
                0x5c800 as libc::c_int as uint32_t,
                0x59000 as libc::c_int as uint32_t,
                0x5d800 as libc::c_int as uint32_t,
                0x5f800 as libc::c_int as uint32_t,
                0x4d000 as libc::c_int as uint32_t,
                0x4a000 as libc::c_int as uint32_t,
                0x4a800 as libc::c_int as uint32_t,
                0x4b000 as libc::c_int as uint32_t,
                0x4c000 as libc::c_int as uint32_t,
                0x48800 as libc::c_int as uint32_t,
                0x49800 as libc::c_int as uint32_t,
                0x4b800 as libc::c_int as uint32_t,
                0x4e800 as libc::c_int as uint32_t,
                0x4e000 as libc::c_int as uint32_t,
                0x4f000 as libc::c_int as uint32_t,
                0x48000 as libc::c_int as uint32_t,
                0x4c800 as libc::c_int as uint32_t,
                0x49000 as libc::c_int as uint32_t,
                0x4d800 as libc::c_int as uint32_t,
                0x4f800 as libc::c_int as uint32_t,
                0xd000 as libc::c_int as uint32_t,
                0xa000 as libc::c_int as uint32_t,
                0xa800 as libc::c_int as uint32_t,
                0xb000 as libc::c_int as uint32_t,
                0xc000 as libc::c_int as uint32_t,
                0x8800 as libc::c_int as uint32_t,
                0x9800 as libc::c_int as uint32_t,
                0xb800 as libc::c_int as uint32_t,
                0xe800 as libc::c_int as uint32_t,
                0xe000 as libc::c_int as uint32_t,
                0xf000 as libc::c_int as uint32_t,
                0x8000 as libc::c_int as uint32_t,
                0xc800 as libc::c_int as uint32_t,
                0x9000 as libc::c_int as uint32_t,
                0xd800 as libc::c_int as uint32_t,
                0xf800 as libc::c_int as uint32_t,
                0x3d000 as libc::c_int as uint32_t,
                0x3a000 as libc::c_int as uint32_t,
                0x3a800 as libc::c_int as uint32_t,
                0x3b000 as libc::c_int as uint32_t,
                0x3c000 as libc::c_int as uint32_t,
                0x38800 as libc::c_int as uint32_t,
                0x39800 as libc::c_int as uint32_t,
                0x3b800 as libc::c_int as uint32_t,
                0x3e800 as libc::c_int as uint32_t,
                0x3e000 as libc::c_int as uint32_t,
                0x3f000 as libc::c_int as uint32_t,
                0x38000 as libc::c_int as uint32_t,
                0x3c800 as libc::c_int as uint32_t,
                0x39000 as libc::c_int as uint32_t,
                0x3d800 as libc::c_int as uint32_t,
                0x3f800 as libc::c_int as uint32_t,
                0x35000 as libc::c_int as uint32_t,
                0x32000 as libc::c_int as uint32_t,
                0x32800 as libc::c_int as uint32_t,
                0x33000 as libc::c_int as uint32_t,
                0x34000 as libc::c_int as uint32_t,
                0x30800 as libc::c_int as uint32_t,
                0x31800 as libc::c_int as uint32_t,
                0x33800 as libc::c_int as uint32_t,
                0x36800 as libc::c_int as uint32_t,
                0x36000 as libc::c_int as uint32_t,
                0x37000 as libc::c_int as uint32_t,
                0x30000 as libc::c_int as uint32_t,
                0x34800 as libc::c_int as uint32_t,
                0x31000 as libc::c_int as uint32_t,
                0x35800 as libc::c_int as uint32_t,
                0x37800 as libc::c_int as uint32_t,
                0x1d000 as libc::c_int as uint32_t,
                0x1a000 as libc::c_int as uint32_t,
                0x1a800 as libc::c_int as uint32_t,
                0x1b000 as libc::c_int as uint32_t,
                0x1c000 as libc::c_int as uint32_t,
                0x18800 as libc::c_int as uint32_t,
                0x19800 as libc::c_int as uint32_t,
                0x1b800 as libc::c_int as uint32_t,
                0x1e800 as libc::c_int as uint32_t,
                0x1e000 as libc::c_int as uint32_t,
                0x1f000 as libc::c_int as uint32_t,
                0x18000 as libc::c_int as uint32_t,
                0x1c800 as libc::c_int as uint32_t,
                0x19000 as libc::c_int as uint32_t,
                0x1d800 as libc::c_int as uint32_t,
                0x1f800 as libc::c_int as uint32_t,
                0x65000 as libc::c_int as uint32_t,
                0x62000 as libc::c_int as uint32_t,
                0x62800 as libc::c_int as uint32_t,
                0x63000 as libc::c_int as uint32_t,
                0x64000 as libc::c_int as uint32_t,
                0x60800 as libc::c_int as uint32_t,
                0x61800 as libc::c_int as uint32_t,
                0x63800 as libc::c_int as uint32_t,
                0x66800 as libc::c_int as uint32_t,
                0x66000 as libc::c_int as uint32_t,
                0x67000 as libc::c_int as uint32_t,
                0x60000 as libc::c_int as uint32_t,
                0x64800 as libc::c_int as uint32_t,
                0x61000 as libc::c_int as uint32_t,
                0x65800 as libc::c_int as uint32_t,
                0x67800 as libc::c_int as uint32_t,
                0x75000 as libc::c_int as uint32_t,
                0x72000 as libc::c_int as uint32_t,
                0x72800 as libc::c_int as uint32_t,
                0x73000 as libc::c_int as uint32_t,
                0x74000 as libc::c_int as uint32_t,
                0x70800 as libc::c_int as uint32_t,
                0x71800 as libc::c_int as uint32_t,
                0x73800 as libc::c_int as uint32_t,
                0x76800 as libc::c_int as uint32_t,
                0x76000 as libc::c_int as uint32_t,
                0x77000 as libc::c_int as uint32_t,
                0x70000 as libc::c_int as uint32_t,
                0x74800 as libc::c_int as uint32_t,
                0x71000 as libc::c_int as uint32_t,
                0x75800 as libc::c_int as uint32_t,
                0x77800 as libc::c_int as uint32_t,
                0x55000 as libc::c_int as uint32_t,
                0x52000 as libc::c_int as uint32_t,
                0x52800 as libc::c_int as uint32_t,
                0x53000 as libc::c_int as uint32_t,
                0x54000 as libc::c_int as uint32_t,
                0x50800 as libc::c_int as uint32_t,
                0x51800 as libc::c_int as uint32_t,
                0x53800 as libc::c_int as uint32_t,
                0x56800 as libc::c_int as uint32_t,
                0x56000 as libc::c_int as uint32_t,
                0x57000 as libc::c_int as uint32_t,
                0x50000 as libc::c_int as uint32_t,
                0x54800 as libc::c_int as uint32_t,
                0x51000 as libc::c_int as uint32_t,
                0x55800 as libc::c_int as uint32_t,
                0x57800 as libc::c_int as uint32_t,
                0x45000 as libc::c_int as uint32_t,
                0x42000 as libc::c_int as uint32_t,
                0x42800 as libc::c_int as uint32_t,
                0x43000 as libc::c_int as uint32_t,
                0x44000 as libc::c_int as uint32_t,
                0x40800 as libc::c_int as uint32_t,
                0x41800 as libc::c_int as uint32_t,
                0x43800 as libc::c_int as uint32_t,
                0x46800 as libc::c_int as uint32_t,
                0x46000 as libc::c_int as uint32_t,
                0x47000 as libc::c_int as uint32_t,
                0x40000 as libc::c_int as uint32_t,
                0x44800 as libc::c_int as uint32_t,
                0x41000 as libc::c_int as uint32_t,
                0x45800 as libc::c_int as uint32_t,
                0x47800 as libc::c_int as uint32_t,
            ],
            [
                0x2380000 as libc::c_int as uint32_t,
                0x2780000 as libc::c_int as uint32_t,
                0x2600000 as libc::c_int as uint32_t,
                0x2700000 as libc::c_int as uint32_t,
                0x2480000 as libc::c_int as uint32_t,
                0x2200000 as libc::c_int as uint32_t,
                0x2080000 as libc::c_int as uint32_t,
                0x2000000 as libc::c_int as uint32_t,
                0x2180000 as libc::c_int as uint32_t,
                0x2580000 as libc::c_int as uint32_t,
                0x2280000 as libc::c_int as uint32_t,
                0x2100000 as libc::c_int as uint32_t,
                0x2300000 as libc::c_int as uint32_t,
                0x2500000 as libc::c_int as uint32_t,
                0x2400000 as libc::c_int as uint32_t,
                0x2680000 as libc::c_int as uint32_t,
                0x5380000 as libc::c_int as uint32_t,
                0x5780000 as libc::c_int as uint32_t,
                0x5600000 as libc::c_int as uint32_t,
                0x5700000 as libc::c_int as uint32_t,
                0x5480000 as libc::c_int as uint32_t,
                0x5200000 as libc::c_int as uint32_t,
                0x5080000 as libc::c_int as uint32_t,
                0x5000000 as libc::c_int as uint32_t,
                0x5180000 as libc::c_int as uint32_t,
                0x5580000 as libc::c_int as uint32_t,
                0x5280000 as libc::c_int as uint32_t,
                0x5100000 as libc::c_int as uint32_t,
                0x5300000 as libc::c_int as uint32_t,
                0x5500000 as libc::c_int as uint32_t,
                0x5400000 as libc::c_int as uint32_t,
                0x5680000 as libc::c_int as uint32_t,
                0x3b80000 as libc::c_int as uint32_t,
                0x3f80000 as libc::c_int as uint32_t,
                0x3e00000 as libc::c_int as uint32_t,
                0x3f00000 as libc::c_int as uint32_t,
                0x3c80000 as libc::c_int as uint32_t,
                0x3a00000 as libc::c_int as uint32_t,
                0x3880000 as libc::c_int as uint32_t,
                0x3800000 as libc::c_int as uint32_t,
                0x3980000 as libc::c_int as uint32_t,
                0x3d80000 as libc::c_int as uint32_t,
                0x3a80000 as libc::c_int as uint32_t,
                0x3900000 as libc::c_int as uint32_t,
                0x3b00000 as libc::c_int as uint32_t,
                0x3d00000 as libc::c_int as uint32_t,
                0x3c00000 as libc::c_int as uint32_t,
                0x3e80000 as libc::c_int as uint32_t,
                0x6380000 as libc::c_int as uint32_t,
                0x6780000 as libc::c_int as uint32_t,
                0x6600000 as libc::c_int as uint32_t,
                0x6700000 as libc::c_int as uint32_t,
                0x6480000 as libc::c_int as uint32_t,
                0x6200000 as libc::c_int as uint32_t,
                0x6080000 as libc::c_int as uint32_t,
                0x6000000 as libc::c_int as uint32_t,
                0x6180000 as libc::c_int as uint32_t,
                0x6580000 as libc::c_int as uint32_t,
                0x6280000 as libc::c_int as uint32_t,
                0x6100000 as libc::c_int as uint32_t,
                0x6300000 as libc::c_int as uint32_t,
                0x6500000 as libc::c_int as uint32_t,
                0x6400000 as libc::c_int as uint32_t,
                0x6680000 as libc::c_int as uint32_t,
                0x380000 as libc::c_int as uint32_t,
                0x780000 as libc::c_int as uint32_t,
                0x600000 as libc::c_int as uint32_t,
                0x700000 as libc::c_int as uint32_t,
                0x480000 as libc::c_int as uint32_t,
                0x200000 as libc::c_int as uint32_t,
                0x80000 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
                0x180000 as libc::c_int as uint32_t,
                0x580000 as libc::c_int as uint32_t,
                0x280000 as libc::c_int as uint32_t,
                0x100000 as libc::c_int as uint32_t,
                0x300000 as libc::c_int as uint32_t,
                0x500000 as libc::c_int as uint32_t,
                0x400000 as libc::c_int as uint32_t,
                0x680000 as libc::c_int as uint32_t,
                0x7b80000 as libc::c_int as uint32_t,
                0x7f80000 as libc::c_int as uint32_t,
                0x7e00000 as libc::c_int as uint32_t,
                0x7f00000 as libc::c_int as uint32_t,
                0x7c80000 as libc::c_int as uint32_t,
                0x7a00000 as libc::c_int as uint32_t,
                0x7880000 as libc::c_int as uint32_t,
                0x7800000 as libc::c_int as uint32_t,
                0x7980000 as libc::c_int as uint32_t,
                0x7d80000 as libc::c_int as uint32_t,
                0x7a80000 as libc::c_int as uint32_t,
                0x7900000 as libc::c_int as uint32_t,
                0x7b00000 as libc::c_int as uint32_t,
                0x7d00000 as libc::c_int as uint32_t,
                0x7c00000 as libc::c_int as uint32_t,
                0x7e80000 as libc::c_int as uint32_t,
                0x1380000 as libc::c_int as uint32_t,
                0x1780000 as libc::c_int as uint32_t,
                0x1600000 as libc::c_int as uint32_t,
                0x1700000 as libc::c_int as uint32_t,
                0x1480000 as libc::c_int as uint32_t,
                0x1200000 as libc::c_int as uint32_t,
                0x1080000 as libc::c_int as uint32_t,
                0x1000000 as libc::c_int as uint32_t,
                0x1180000 as libc::c_int as uint32_t,
                0x1580000 as libc::c_int as uint32_t,
                0x1280000 as libc::c_int as uint32_t,
                0x1100000 as libc::c_int as uint32_t,
                0x1300000 as libc::c_int as uint32_t,
                0x1500000 as libc::c_int as uint32_t,
                0x1400000 as libc::c_int as uint32_t,
                0x1680000 as libc::c_int as uint32_t,
                0x4380000 as libc::c_int as uint32_t,
                0x4780000 as libc::c_int as uint32_t,
                0x4600000 as libc::c_int as uint32_t,
                0x4700000 as libc::c_int as uint32_t,
                0x4480000 as libc::c_int as uint32_t,
                0x4200000 as libc::c_int as uint32_t,
                0x4080000 as libc::c_int as uint32_t,
                0x4000000 as libc::c_int as uint32_t,
                0x4180000 as libc::c_int as uint32_t,
                0x4580000 as libc::c_int as uint32_t,
                0x4280000 as libc::c_int as uint32_t,
                0x4100000 as libc::c_int as uint32_t,
                0x4300000 as libc::c_int as uint32_t,
                0x4500000 as libc::c_int as uint32_t,
                0x4400000 as libc::c_int as uint32_t,
                0x4680000 as libc::c_int as uint32_t,
                0x7380000 as libc::c_int as uint32_t,
                0x7780000 as libc::c_int as uint32_t,
                0x7600000 as libc::c_int as uint32_t,
                0x7700000 as libc::c_int as uint32_t,
                0x7480000 as libc::c_int as uint32_t,
                0x7200000 as libc::c_int as uint32_t,
                0x7080000 as libc::c_int as uint32_t,
                0x7000000 as libc::c_int as uint32_t,
                0x7180000 as libc::c_int as uint32_t,
                0x7580000 as libc::c_int as uint32_t,
                0x7280000 as libc::c_int as uint32_t,
                0x7100000 as libc::c_int as uint32_t,
                0x7300000 as libc::c_int as uint32_t,
                0x7500000 as libc::c_int as uint32_t,
                0x7400000 as libc::c_int as uint32_t,
                0x7680000 as libc::c_int as uint32_t,
                0xb80000 as libc::c_int as uint32_t,
                0xf80000 as libc::c_int as uint32_t,
                0xe00000 as libc::c_int as uint32_t,
                0xf00000 as libc::c_int as uint32_t,
                0xc80000 as libc::c_int as uint32_t,
                0xa00000 as libc::c_int as uint32_t,
                0x880000 as libc::c_int as uint32_t,
                0x800000 as libc::c_int as uint32_t,
                0x980000 as libc::c_int as uint32_t,
                0xd80000 as libc::c_int as uint32_t,
                0xa80000 as libc::c_int as uint32_t,
                0x900000 as libc::c_int as uint32_t,
                0xb00000 as libc::c_int as uint32_t,
                0xd00000 as libc::c_int as uint32_t,
                0xc00000 as libc::c_int as uint32_t,
                0xe80000 as libc::c_int as uint32_t,
                0x3380000 as libc::c_int as uint32_t,
                0x3780000 as libc::c_int as uint32_t,
                0x3600000 as libc::c_int as uint32_t,
                0x3700000 as libc::c_int as uint32_t,
                0x3480000 as libc::c_int as uint32_t,
                0x3200000 as libc::c_int as uint32_t,
                0x3080000 as libc::c_int as uint32_t,
                0x3000000 as libc::c_int as uint32_t,
                0x3180000 as libc::c_int as uint32_t,
                0x3580000 as libc::c_int as uint32_t,
                0x3280000 as libc::c_int as uint32_t,
                0x3100000 as libc::c_int as uint32_t,
                0x3300000 as libc::c_int as uint32_t,
                0x3500000 as libc::c_int as uint32_t,
                0x3400000 as libc::c_int as uint32_t,
                0x3680000 as libc::c_int as uint32_t,
                0x2b80000 as libc::c_int as uint32_t,
                0x2f80000 as libc::c_int as uint32_t,
                0x2e00000 as libc::c_int as uint32_t,
                0x2f00000 as libc::c_int as uint32_t,
                0x2c80000 as libc::c_int as uint32_t,
                0x2a00000 as libc::c_int as uint32_t,
                0x2880000 as libc::c_int as uint32_t,
                0x2800000 as libc::c_int as uint32_t,
                0x2980000 as libc::c_int as uint32_t,
                0x2d80000 as libc::c_int as uint32_t,
                0x2a80000 as libc::c_int as uint32_t,
                0x2900000 as libc::c_int as uint32_t,
                0x2b00000 as libc::c_int as uint32_t,
                0x2d00000 as libc::c_int as uint32_t,
                0x2c00000 as libc::c_int as uint32_t,
                0x2e80000 as libc::c_int as uint32_t,
                0x6b80000 as libc::c_int as uint32_t,
                0x6f80000 as libc::c_int as uint32_t,
                0x6e00000 as libc::c_int as uint32_t,
                0x6f00000 as libc::c_int as uint32_t,
                0x6c80000 as libc::c_int as uint32_t,
                0x6a00000 as libc::c_int as uint32_t,
                0x6880000 as libc::c_int as uint32_t,
                0x6800000 as libc::c_int as uint32_t,
                0x6980000 as libc::c_int as uint32_t,
                0x6d80000 as libc::c_int as uint32_t,
                0x6a80000 as libc::c_int as uint32_t,
                0x6900000 as libc::c_int as uint32_t,
                0x6b00000 as libc::c_int as uint32_t,
                0x6d00000 as libc::c_int as uint32_t,
                0x6c00000 as libc::c_int as uint32_t,
                0x6e80000 as libc::c_int as uint32_t,
                0x5b80000 as libc::c_int as uint32_t,
                0x5f80000 as libc::c_int as uint32_t,
                0x5e00000 as libc::c_int as uint32_t,
                0x5f00000 as libc::c_int as uint32_t,
                0x5c80000 as libc::c_int as uint32_t,
                0x5a00000 as libc::c_int as uint32_t,
                0x5880000 as libc::c_int as uint32_t,
                0x5800000 as libc::c_int as uint32_t,
                0x5980000 as libc::c_int as uint32_t,
                0x5d80000 as libc::c_int as uint32_t,
                0x5a80000 as libc::c_int as uint32_t,
                0x5900000 as libc::c_int as uint32_t,
                0x5b00000 as libc::c_int as uint32_t,
                0x5d00000 as libc::c_int as uint32_t,
                0x5c00000 as libc::c_int as uint32_t,
                0x5e80000 as libc::c_int as uint32_t,
                0x4b80000 as libc::c_int as uint32_t,
                0x4f80000 as libc::c_int as uint32_t,
                0x4e00000 as libc::c_int as uint32_t,
                0x4f00000 as libc::c_int as uint32_t,
                0x4c80000 as libc::c_int as uint32_t,
                0x4a00000 as libc::c_int as uint32_t,
                0x4880000 as libc::c_int as uint32_t,
                0x4800000 as libc::c_int as uint32_t,
                0x4980000 as libc::c_int as uint32_t,
                0x4d80000 as libc::c_int as uint32_t,
                0x4a80000 as libc::c_int as uint32_t,
                0x4900000 as libc::c_int as uint32_t,
                0x4b00000 as libc::c_int as uint32_t,
                0x4d00000 as libc::c_int as uint32_t,
                0x4c00000 as libc::c_int as uint32_t,
                0x4e80000 as libc::c_int as uint32_t,
                0x1b80000 as libc::c_int as uint32_t,
                0x1f80000 as libc::c_int as uint32_t,
                0x1e00000 as libc::c_int as uint32_t,
                0x1f00000 as libc::c_int as uint32_t,
                0x1c80000 as libc::c_int as uint32_t,
                0x1a00000 as libc::c_int as uint32_t,
                0x1880000 as libc::c_int as uint32_t,
                0x1800000 as libc::c_int as uint32_t,
                0x1980000 as libc::c_int as uint32_t,
                0x1d80000 as libc::c_int as uint32_t,
                0x1a80000 as libc::c_int as uint32_t,
                0x1900000 as libc::c_int as uint32_t,
                0x1b00000 as libc::c_int as uint32_t,
                0x1d00000 as libc::c_int as uint32_t,
                0x1c00000 as libc::c_int as uint32_t,
                0x1e80000 as libc::c_int as uint32_t,
            ],
            [
                0xb8000003 as libc::c_uint,
                0xb0000003 as libc::c_uint,
                0xa0000003 as libc::c_uint,
                0xd8000003 as libc::c_uint,
                0xc8000003 as libc::c_uint,
                0xe0000003 as libc::c_uint,
                0x90000003 as libc::c_uint,
                0xd0000003 as libc::c_uint,
                0x88000003 as libc::c_uint,
                0xc0000003 as libc::c_uint,
                0x80000003 as libc::c_uint,
                0xf0000003 as libc::c_uint,
                0xf8000003 as libc::c_uint,
                0xe8000003 as libc::c_uint,
                0x98000003 as libc::c_uint,
                0xa8000003 as libc::c_uint,
                0x38000003 as libc::c_int as uint32_t,
                0x30000003 as libc::c_int as uint32_t,
                0x20000003 as libc::c_int as uint32_t,
                0x58000003 as libc::c_int as uint32_t,
                0x48000003 as libc::c_int as uint32_t,
                0x60000003 as libc::c_int as uint32_t,
                0x10000003 as libc::c_int as uint32_t,
                0x50000003 as libc::c_int as uint32_t,
                0x8000003 as libc::c_int as uint32_t,
                0x40000003 as libc::c_int as uint32_t,
                0x3 as libc::c_int as uint32_t,
                0x70000003 as libc::c_int as uint32_t,
                0x78000003 as libc::c_int as uint32_t,
                0x68000003 as libc::c_int as uint32_t,
                0x18000003 as libc::c_int as uint32_t,
                0x28000003 as libc::c_int as uint32_t,
                0x38000001 as libc::c_int as uint32_t,
                0x30000001 as libc::c_int as uint32_t,
                0x20000001 as libc::c_int as uint32_t,
                0x58000001 as libc::c_int as uint32_t,
                0x48000001 as libc::c_int as uint32_t,
                0x60000001 as libc::c_int as uint32_t,
                0x10000001 as libc::c_int as uint32_t,
                0x50000001 as libc::c_int as uint32_t,
                0x8000001 as libc::c_int as uint32_t,
                0x40000001 as libc::c_int as uint32_t,
                0x1 as libc::c_int as uint32_t,
                0x70000001 as libc::c_int as uint32_t,
                0x78000001 as libc::c_int as uint32_t,
                0x68000001 as libc::c_int as uint32_t,
                0x18000001 as libc::c_int as uint32_t,
                0x28000001 as libc::c_int as uint32_t,
                0x38000002 as libc::c_int as uint32_t,
                0x30000002 as libc::c_int as uint32_t,
                0x20000002 as libc::c_int as uint32_t,
                0x58000002 as libc::c_int as uint32_t,
                0x48000002 as libc::c_int as uint32_t,
                0x60000002 as libc::c_int as uint32_t,
                0x10000002 as libc::c_int as uint32_t,
                0x50000002 as libc::c_int as uint32_t,
                0x8000002 as libc::c_int as uint32_t,
                0x40000002 as libc::c_int as uint32_t,
                0x2 as libc::c_int as uint32_t,
                0x70000002 as libc::c_int as uint32_t,
                0x78000002 as libc::c_int as uint32_t,
                0x68000002 as libc::c_int as uint32_t,
                0x18000002 as libc::c_int as uint32_t,
                0x28000002 as libc::c_int as uint32_t,
                0xb8000006 as libc::c_uint,
                0xb0000006 as libc::c_uint,
                0xa0000006 as libc::c_uint,
                0xd8000006 as libc::c_uint,
                0xc8000006 as libc::c_uint,
                0xe0000006 as libc::c_uint,
                0x90000006 as libc::c_uint,
                0xd0000006 as libc::c_uint,
                0x88000006 as libc::c_uint,
                0xc0000006 as libc::c_uint,
                0x80000006 as libc::c_uint,
                0xf0000006 as libc::c_uint,
                0xf8000006 as libc::c_uint,
                0xe8000006 as libc::c_uint,
                0x98000006 as libc::c_uint,
                0xa8000006 as libc::c_uint,
                0xb8000004 as libc::c_uint,
                0xb0000004 as libc::c_uint,
                0xa0000004 as libc::c_uint,
                0xd8000004 as libc::c_uint,
                0xc8000004 as libc::c_uint,
                0xe0000004 as libc::c_uint,
                0x90000004 as libc::c_uint,
                0xd0000004 as libc::c_uint,
                0x88000004 as libc::c_uint,
                0xc0000004 as libc::c_uint,
                0x80000004 as libc::c_uint,
                0xf0000004 as libc::c_uint,
                0xf8000004 as libc::c_uint,
                0xe8000004 as libc::c_uint,
                0x98000004 as libc::c_uint,
                0xa8000004 as libc::c_uint,
                0xb8000007 as libc::c_uint,
                0xb0000007 as libc::c_uint,
                0xa0000007 as libc::c_uint,
                0xd8000007 as libc::c_uint,
                0xc8000007 as libc::c_uint,
                0xe0000007 as libc::c_uint,
                0x90000007 as libc::c_uint,
                0xd0000007 as libc::c_uint,
                0x88000007 as libc::c_uint,
                0xc0000007 as libc::c_uint,
                0x80000007 as libc::c_uint,
                0xf0000007 as libc::c_uint,
                0xf8000007 as libc::c_uint,
                0xe8000007 as libc::c_uint,
                0x98000007 as libc::c_uint,
                0xa8000007 as libc::c_uint,
                0x38000000 as libc::c_int as uint32_t,
                0x30000000 as libc::c_int as uint32_t,
                0x20000000 as libc::c_int as uint32_t,
                0x58000000 as libc::c_int as uint32_t,
                0x48000000 as libc::c_int as uint32_t,
                0x60000000 as libc::c_int as uint32_t,
                0x10000000 as libc::c_int as uint32_t,
                0x50000000 as libc::c_int as uint32_t,
                0x8000000 as libc::c_int as uint32_t,
                0x40000000 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
                0x70000000 as libc::c_int as uint32_t,
                0x78000000 as libc::c_int as uint32_t,
                0x68000000 as libc::c_int as uint32_t,
                0x18000000 as libc::c_int as uint32_t,
                0x28000000 as libc::c_int as uint32_t,
                0x38000005 as libc::c_int as uint32_t,
                0x30000005 as libc::c_int as uint32_t,
                0x20000005 as libc::c_int as uint32_t,
                0x58000005 as libc::c_int as uint32_t,
                0x48000005 as libc::c_int as uint32_t,
                0x60000005 as libc::c_int as uint32_t,
                0x10000005 as libc::c_int as uint32_t,
                0x50000005 as libc::c_int as uint32_t,
                0x8000005 as libc::c_int as uint32_t,
                0x40000005 as libc::c_int as uint32_t,
                0x5 as libc::c_int as uint32_t,
                0x70000005 as libc::c_int as uint32_t,
                0x78000005 as libc::c_int as uint32_t,
                0x68000005 as libc::c_int as uint32_t,
                0x18000005 as libc::c_int as uint32_t,
                0x28000005 as libc::c_int as uint32_t,
                0xb8000000 as libc::c_uint,
                0xb0000000 as libc::c_uint,
                0xa0000000 as libc::c_uint,
                0xd8000000 as libc::c_uint,
                0xc8000000 as libc::c_uint,
                0xe0000000 as libc::c_uint,
                0x90000000 as libc::c_uint,
                0xd0000000 as libc::c_uint,
                0x88000000 as libc::c_uint,
                0xc0000000 as libc::c_uint,
                0x80000000 as libc::c_uint,
                0xf0000000 as libc::c_uint,
                0xf8000000 as libc::c_uint,
                0xe8000000 as libc::c_uint,
                0x98000000 as libc::c_uint,
                0xa8000000 as libc::c_uint,
                0xb8000002 as libc::c_uint,
                0xb0000002 as libc::c_uint,
                0xa0000002 as libc::c_uint,
                0xd8000002 as libc::c_uint,
                0xc8000002 as libc::c_uint,
                0xe0000002 as libc::c_uint,
                0x90000002 as libc::c_uint,
                0xd0000002 as libc::c_uint,
                0x88000002 as libc::c_uint,
                0xc0000002 as libc::c_uint,
                0x80000002 as libc::c_uint,
                0xf0000002 as libc::c_uint,
                0xf8000002 as libc::c_uint,
                0xe8000002 as libc::c_uint,
                0x98000002 as libc::c_uint,
                0xa8000002 as libc::c_uint,
                0xb8000005 as libc::c_uint,
                0xb0000005 as libc::c_uint,
                0xa0000005 as libc::c_uint,
                0xd8000005 as libc::c_uint,
                0xc8000005 as libc::c_uint,
                0xe0000005 as libc::c_uint,
                0x90000005 as libc::c_uint,
                0xd0000005 as libc::c_uint,
                0x88000005 as libc::c_uint,
                0xc0000005 as libc::c_uint,
                0x80000005 as libc::c_uint,
                0xf0000005 as libc::c_uint,
                0xf8000005 as libc::c_uint,
                0xe8000005 as libc::c_uint,
                0x98000005 as libc::c_uint,
                0xa8000005 as libc::c_uint,
                0x38000004 as libc::c_int as uint32_t,
                0x30000004 as libc::c_int as uint32_t,
                0x20000004 as libc::c_int as uint32_t,
                0x58000004 as libc::c_int as uint32_t,
                0x48000004 as libc::c_int as uint32_t,
                0x60000004 as libc::c_int as uint32_t,
                0x10000004 as libc::c_int as uint32_t,
                0x50000004 as libc::c_int as uint32_t,
                0x8000004 as libc::c_int as uint32_t,
                0x40000004 as libc::c_int as uint32_t,
                0x4 as libc::c_int as uint32_t,
                0x70000004 as libc::c_int as uint32_t,
                0x78000004 as libc::c_int as uint32_t,
                0x68000004 as libc::c_int as uint32_t,
                0x18000004 as libc::c_int as uint32_t,
                0x28000004 as libc::c_int as uint32_t,
                0x38000007 as libc::c_int as uint32_t,
                0x30000007 as libc::c_int as uint32_t,
                0x20000007 as libc::c_int as uint32_t,
                0x58000007 as libc::c_int as uint32_t,
                0x48000007 as libc::c_int as uint32_t,
                0x60000007 as libc::c_int as uint32_t,
                0x10000007 as libc::c_int as uint32_t,
                0x50000007 as libc::c_int as uint32_t,
                0x8000007 as libc::c_int as uint32_t,
                0x40000007 as libc::c_int as uint32_t,
                0x7 as libc::c_int as uint32_t,
                0x70000007 as libc::c_int as uint32_t,
                0x78000007 as libc::c_int as uint32_t,
                0x68000007 as libc::c_int as uint32_t,
                0x18000007 as libc::c_int as uint32_t,
                0x28000007 as libc::c_int as uint32_t,
                0x38000006 as libc::c_int as uint32_t,
                0x30000006 as libc::c_int as uint32_t,
                0x20000006 as libc::c_int as uint32_t,
                0x58000006 as libc::c_int as uint32_t,
                0x48000006 as libc::c_int as uint32_t,
                0x60000006 as libc::c_int as uint32_t,
                0x10000006 as libc::c_int as uint32_t,
                0x50000006 as libc::c_int as uint32_t,
                0x8000006 as libc::c_int as uint32_t,
                0x40000006 as libc::c_int as uint32_t,
                0x6 as libc::c_int as uint32_t,
                0x70000006 as libc::c_int as uint32_t,
                0x78000006 as libc::c_int as uint32_t,
                0x68000006 as libc::c_int as uint32_t,
                0x18000006 as libc::c_int as uint32_t,
                0x28000006 as libc::c_int as uint32_t,
                0xb8000001 as libc::c_uint,
                0xb0000001 as libc::c_uint,
                0xa0000001 as libc::c_uint,
                0xd8000001 as libc::c_uint,
                0xc8000001 as libc::c_uint,
                0xe0000001 as libc::c_uint,
                0x90000001 as libc::c_uint,
                0xd0000001 as libc::c_uint,
                0x88000001 as libc::c_uint,
                0xc0000001 as libc::c_uint,
                0x80000001 as libc::c_uint,
                0xf0000001 as libc::c_uint,
                0xf8000001 as libc::c_uint,
                0xe8000001 as libc::c_uint,
                0x98000001 as libc::c_uint,
                0xa8000001 as libc::c_uint,
            ],
            [
                0xe8 as libc::c_int as uint32_t,
                0xf0 as libc::c_int as uint32_t,
                0xa0 as libc::c_int as uint32_t,
                0x88 as libc::c_int as uint32_t,
                0xb8 as libc::c_int as uint32_t,
                0x80 as libc::c_int as uint32_t,
                0xa8 as libc::c_int as uint32_t,
                0xd0 as libc::c_int as uint32_t,
                0x98 as libc::c_int as uint32_t,
                0xe0 as libc::c_int as uint32_t,
                0xc0 as libc::c_int as uint32_t,
                0xf8 as libc::c_int as uint32_t,
                0xb0 as libc::c_int as uint32_t,
                0x90 as libc::c_int as uint32_t,
                0xc8 as libc::c_int as uint32_t,
                0xd8 as libc::c_int as uint32_t,
                0x1e8 as libc::c_int as uint32_t,
                0x1f0 as libc::c_int as uint32_t,
                0x1a0 as libc::c_int as uint32_t,
                0x188 as libc::c_int as uint32_t,
                0x1b8 as libc::c_int as uint32_t,
                0x180 as libc::c_int as uint32_t,
                0x1a8 as libc::c_int as uint32_t,
                0x1d0 as libc::c_int as uint32_t,
                0x198 as libc::c_int as uint32_t,
                0x1e0 as libc::c_int as uint32_t,
                0x1c0 as libc::c_int as uint32_t,
                0x1f8 as libc::c_int as uint32_t,
                0x1b0 as libc::c_int as uint32_t,
                0x190 as libc::c_int as uint32_t,
                0x1c8 as libc::c_int as uint32_t,
                0x1d8 as libc::c_int as uint32_t,
                0x568 as libc::c_int as uint32_t,
                0x570 as libc::c_int as uint32_t,
                0x520 as libc::c_int as uint32_t,
                0x508 as libc::c_int as uint32_t,
                0x538 as libc::c_int as uint32_t,
                0x500 as libc::c_int as uint32_t,
                0x528 as libc::c_int as uint32_t,
                0x550 as libc::c_int as uint32_t,
                0x518 as libc::c_int as uint32_t,
                0x560 as libc::c_int as uint32_t,
                0x540 as libc::c_int as uint32_t,
                0x578 as libc::c_int as uint32_t,
                0x530 as libc::c_int as uint32_t,
                0x510 as libc::c_int as uint32_t,
                0x548 as libc::c_int as uint32_t,
                0x558 as libc::c_int as uint32_t,
                0x4e8 as libc::c_int as uint32_t,
                0x4f0 as libc::c_int as uint32_t,
                0x4a0 as libc::c_int as uint32_t,
                0x488 as libc::c_int as uint32_t,
                0x4b8 as libc::c_int as uint32_t,
                0x480 as libc::c_int as uint32_t,
                0x4a8 as libc::c_int as uint32_t,
                0x4d0 as libc::c_int as uint32_t,
                0x498 as libc::c_int as uint32_t,
                0x4e0 as libc::c_int as uint32_t,
                0x4c0 as libc::c_int as uint32_t,
                0x4f8 as libc::c_int as uint32_t,
                0x4b0 as libc::c_int as uint32_t,
                0x490 as libc::c_int as uint32_t,
                0x4c8 as libc::c_int as uint32_t,
                0x4d8 as libc::c_int as uint32_t,
                0x2e8 as libc::c_int as uint32_t,
                0x2f0 as libc::c_int as uint32_t,
                0x2a0 as libc::c_int as uint32_t,
                0x288 as libc::c_int as uint32_t,
                0x2b8 as libc::c_int as uint32_t,
                0x280 as libc::c_int as uint32_t,
                0x2a8 as libc::c_int as uint32_t,
                0x2d0 as libc::c_int as uint32_t,
                0x298 as libc::c_int as uint32_t,
                0x2e0 as libc::c_int as uint32_t,
                0x2c0 as libc::c_int as uint32_t,
                0x2f8 as libc::c_int as uint32_t,
                0x2b0 as libc::c_int as uint32_t,
                0x290 as libc::c_int as uint32_t,
                0x2c8 as libc::c_int as uint32_t,
                0x2d8 as libc::c_int as uint32_t,
                0x5e8 as libc::c_int as uint32_t,
                0x5f0 as libc::c_int as uint32_t,
                0x5a0 as libc::c_int as uint32_t,
                0x588 as libc::c_int as uint32_t,
                0x5b8 as libc::c_int as uint32_t,
                0x580 as libc::c_int as uint32_t,
                0x5a8 as libc::c_int as uint32_t,
                0x5d0 as libc::c_int as uint32_t,
                0x598 as libc::c_int as uint32_t,
                0x5e0 as libc::c_int as uint32_t,
                0x5c0 as libc::c_int as uint32_t,
                0x5f8 as libc::c_int as uint32_t,
                0x5b0 as libc::c_int as uint32_t,
                0x590 as libc::c_int as uint32_t,
                0x5c8 as libc::c_int as uint32_t,
                0x5d8 as libc::c_int as uint32_t,
                0x268 as libc::c_int as uint32_t,
                0x270 as libc::c_int as uint32_t,
                0x220 as libc::c_int as uint32_t,
                0x208 as libc::c_int as uint32_t,
                0x238 as libc::c_int as uint32_t,
                0x200 as libc::c_int as uint32_t,
                0x228 as libc::c_int as uint32_t,
                0x250 as libc::c_int as uint32_t,
                0x218 as libc::c_int as uint32_t,
                0x260 as libc::c_int as uint32_t,
                0x240 as libc::c_int as uint32_t,
                0x278 as libc::c_int as uint32_t,
                0x230 as libc::c_int as uint32_t,
                0x210 as libc::c_int as uint32_t,
                0x248 as libc::c_int as uint32_t,
                0x258 as libc::c_int as uint32_t,
                0x7e8 as libc::c_int as uint32_t,
                0x7f0 as libc::c_int as uint32_t,
                0x7a0 as libc::c_int as uint32_t,
                0x788 as libc::c_int as uint32_t,
                0x7b8 as libc::c_int as uint32_t,
                0x780 as libc::c_int as uint32_t,
                0x7a8 as libc::c_int as uint32_t,
                0x7d0 as libc::c_int as uint32_t,
                0x798 as libc::c_int as uint32_t,
                0x7e0 as libc::c_int as uint32_t,
                0x7c0 as libc::c_int as uint32_t,
                0x7f8 as libc::c_int as uint32_t,
                0x7b0 as libc::c_int as uint32_t,
                0x790 as libc::c_int as uint32_t,
                0x7c8 as libc::c_int as uint32_t,
                0x7d8 as libc::c_int as uint32_t,
                0x468 as libc::c_int as uint32_t,
                0x470 as libc::c_int as uint32_t,
                0x420 as libc::c_int as uint32_t,
                0x408 as libc::c_int as uint32_t,
                0x438 as libc::c_int as uint32_t,
                0x400 as libc::c_int as uint32_t,
                0x428 as libc::c_int as uint32_t,
                0x450 as libc::c_int as uint32_t,
                0x418 as libc::c_int as uint32_t,
                0x460 as libc::c_int as uint32_t,
                0x440 as libc::c_int as uint32_t,
                0x478 as libc::c_int as uint32_t,
                0x430 as libc::c_int as uint32_t,
                0x410 as libc::c_int as uint32_t,
                0x448 as libc::c_int as uint32_t,
                0x458 as libc::c_int as uint32_t,
                0x368 as libc::c_int as uint32_t,
                0x370 as libc::c_int as uint32_t,
                0x320 as libc::c_int as uint32_t,
                0x308 as libc::c_int as uint32_t,
                0x338 as libc::c_int as uint32_t,
                0x300 as libc::c_int as uint32_t,
                0x328 as libc::c_int as uint32_t,
                0x350 as libc::c_int as uint32_t,
                0x318 as libc::c_int as uint32_t,
                0x360 as libc::c_int as uint32_t,
                0x340 as libc::c_int as uint32_t,
                0x378 as libc::c_int as uint32_t,
                0x330 as libc::c_int as uint32_t,
                0x310 as libc::c_int as uint32_t,
                0x348 as libc::c_int as uint32_t,
                0x358 as libc::c_int as uint32_t,
                0x3e8 as libc::c_int as uint32_t,
                0x3f0 as libc::c_int as uint32_t,
                0x3a0 as libc::c_int as uint32_t,
                0x388 as libc::c_int as uint32_t,
                0x3b8 as libc::c_int as uint32_t,
                0x380 as libc::c_int as uint32_t,
                0x3a8 as libc::c_int as uint32_t,
                0x3d0 as libc::c_int as uint32_t,
                0x398 as libc::c_int as uint32_t,
                0x3e0 as libc::c_int as uint32_t,
                0x3c0 as libc::c_int as uint32_t,
                0x3f8 as libc::c_int as uint32_t,
                0x3b0 as libc::c_int as uint32_t,
                0x390 as libc::c_int as uint32_t,
                0x3c8 as libc::c_int as uint32_t,
                0x3d8 as libc::c_int as uint32_t,
                0x768 as libc::c_int as uint32_t,
                0x770 as libc::c_int as uint32_t,
                0x720 as libc::c_int as uint32_t,
                0x708 as libc::c_int as uint32_t,
                0x738 as libc::c_int as uint32_t,
                0x700 as libc::c_int as uint32_t,
                0x728 as libc::c_int as uint32_t,
                0x750 as libc::c_int as uint32_t,
                0x718 as libc::c_int as uint32_t,
                0x760 as libc::c_int as uint32_t,
                0x740 as libc::c_int as uint32_t,
                0x778 as libc::c_int as uint32_t,
                0x730 as libc::c_int as uint32_t,
                0x710 as libc::c_int as uint32_t,
                0x748 as libc::c_int as uint32_t,
                0x758 as libc::c_int as uint32_t,
                0x6e8 as libc::c_int as uint32_t,
                0x6f0 as libc::c_int as uint32_t,
                0x6a0 as libc::c_int as uint32_t,
                0x688 as libc::c_int as uint32_t,
                0x6b8 as libc::c_int as uint32_t,
                0x680 as libc::c_int as uint32_t,
                0x6a8 as libc::c_int as uint32_t,
                0x6d0 as libc::c_int as uint32_t,
                0x698 as libc::c_int as uint32_t,
                0x6e0 as libc::c_int as uint32_t,
                0x6c0 as libc::c_int as uint32_t,
                0x6f8 as libc::c_int as uint32_t,
                0x6b0 as libc::c_int as uint32_t,
                0x690 as libc::c_int as uint32_t,
                0x6c8 as libc::c_int as uint32_t,
                0x6d8 as libc::c_int as uint32_t,
                0x68 as libc::c_int as uint32_t,
                0x70 as libc::c_int as uint32_t,
                0x20 as libc::c_int as uint32_t,
                0x8 as libc::c_int as uint32_t,
                0x38 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
                0x28 as libc::c_int as uint32_t,
                0x50 as libc::c_int as uint32_t,
                0x18 as libc::c_int as uint32_t,
                0x60 as libc::c_int as uint32_t,
                0x40 as libc::c_int as uint32_t,
                0x78 as libc::c_int as uint32_t,
                0x30 as libc::c_int as uint32_t,
                0x10 as libc::c_int as uint32_t,
                0x48 as libc::c_int as uint32_t,
                0x58 as libc::c_int as uint32_t,
                0x168 as libc::c_int as uint32_t,
                0x170 as libc::c_int as uint32_t,
                0x120 as libc::c_int as uint32_t,
                0x108 as libc::c_int as uint32_t,
                0x138 as libc::c_int as uint32_t,
                0x100 as libc::c_int as uint32_t,
                0x128 as libc::c_int as uint32_t,
                0x150 as libc::c_int as uint32_t,
                0x118 as libc::c_int as uint32_t,
                0x160 as libc::c_int as uint32_t,
                0x140 as libc::c_int as uint32_t,
                0x178 as libc::c_int as uint32_t,
                0x130 as libc::c_int as uint32_t,
                0x110 as libc::c_int as uint32_t,
                0x148 as libc::c_int as uint32_t,
                0x158 as libc::c_int as uint32_t,
                0x668 as libc::c_int as uint32_t,
                0x670 as libc::c_int as uint32_t,
                0x620 as libc::c_int as uint32_t,
                0x608 as libc::c_int as uint32_t,
                0x638 as libc::c_int as uint32_t,
                0x600 as libc::c_int as uint32_t,
                0x628 as libc::c_int as uint32_t,
                0x650 as libc::c_int as uint32_t,
                0x618 as libc::c_int as uint32_t,
                0x660 as libc::c_int as uint32_t,
                0x640 as libc::c_int as uint32_t,
                0x678 as libc::c_int as uint32_t,
                0x630 as libc::c_int as uint32_t,
                0x610 as libc::c_int as uint32_t,
                0x648 as libc::c_int as uint32_t,
                0x658 as libc::c_int as uint32_t,
            ],
        ],
    };
    init
};
pub unsafe extern "C" fn _nettle_gost28147_encrypt_block(
    mut key: *const uint32_t,
    mut sbox: *const [uint32_t; 256],
    mut in_0: *const uint32_t,
    mut out: *mut uint32_t,
) {
    let mut l: uint32_t = 0;
    let mut r: uint32_t = 0;
    r = *in_0.offset(0 as libc::c_int as isize);
    l = *in_0.offset(1 as libc::c_int as isize);
    let mut round_tmp: uint32_t = 0;
    round_tmp = (*key.offset(0 as libc::c_int as isize)).wrapping_add(r);
    l
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp >> 24 as libc::c_int) as usize];
    round_tmp = (*key.offset(1 as libc::c_int as isize)).wrapping_add(l);
    r
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp >> 24 as libc::c_int) as usize];
    let mut round_tmp_0: uint32_t = 0;
    round_tmp_0 = (*key.offset(2 as libc::c_int as isize)).wrapping_add(r);
    l
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_0 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_0 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_0 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_0 >> 24 as libc::c_int) as usize];
    round_tmp_0 = (*key.offset(3 as libc::c_int as isize)).wrapping_add(l);
    r
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_0 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_0 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_0 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_0 >> 24 as libc::c_int) as usize];
    let mut round_tmp_1: uint32_t = 0;
    round_tmp_1 = (*key.offset(4 as libc::c_int as isize)).wrapping_add(r);
    l
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_1 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_1 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_1 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_1 >> 24 as libc::c_int) as usize];
    round_tmp_1 = (*key.offset(5 as libc::c_int as isize)).wrapping_add(l);
    r
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_1 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_1 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_1 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_1 >> 24 as libc::c_int) as usize];
    let mut round_tmp_2: uint32_t = 0;
    round_tmp_2 = (*key.offset(6 as libc::c_int as isize)).wrapping_add(r);
    l
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_2 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_2 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_2 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_2 >> 24 as libc::c_int) as usize];
    round_tmp_2 = (*key.offset(7 as libc::c_int as isize)).wrapping_add(l);
    r
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_2 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_2 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_2 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_2 >> 24 as libc::c_int) as usize];
    let mut round_tmp_3: uint32_t = 0;
    round_tmp_3 = (*key.offset(0 as libc::c_int as isize)).wrapping_add(r);
    l
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_3 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_3 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_3 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_3 >> 24 as libc::c_int) as usize];
    round_tmp_3 = (*key.offset(1 as libc::c_int as isize)).wrapping_add(l);
    r
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_3 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_3 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_3 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_3 >> 24 as libc::c_int) as usize];
    let mut round_tmp_4: uint32_t = 0;
    round_tmp_4 = (*key.offset(2 as libc::c_int as isize)).wrapping_add(r);
    l
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_4 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_4 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_4 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_4 >> 24 as libc::c_int) as usize];
    round_tmp_4 = (*key.offset(3 as libc::c_int as isize)).wrapping_add(l);
    r
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_4 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_4 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_4 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_4 >> 24 as libc::c_int) as usize];
    let mut round_tmp_5: uint32_t = 0;
    round_tmp_5 = (*key.offset(4 as libc::c_int as isize)).wrapping_add(r);
    l
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_5 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_5 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_5 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_5 >> 24 as libc::c_int) as usize];
    round_tmp_5 = (*key.offset(5 as libc::c_int as isize)).wrapping_add(l);
    r
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_5 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_5 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_5 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_5 >> 24 as libc::c_int) as usize];
    let mut round_tmp_6: uint32_t = 0;
    round_tmp_6 = (*key.offset(6 as libc::c_int as isize)).wrapping_add(r);
    l
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_6 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_6 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_6 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_6 >> 24 as libc::c_int) as usize];
    round_tmp_6 = (*key.offset(7 as libc::c_int as isize)).wrapping_add(l);
    r
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_6 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_6 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_6 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_6 >> 24 as libc::c_int) as usize];
    let mut round_tmp_7: uint32_t = 0;
    round_tmp_7 = (*key.offset(0 as libc::c_int as isize)).wrapping_add(r);
    l
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_7 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_7 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_7 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_7 >> 24 as libc::c_int) as usize];
    round_tmp_7 = (*key.offset(1 as libc::c_int as isize)).wrapping_add(l);
    r
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_7 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_7 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_7 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_7 >> 24 as libc::c_int) as usize];
    let mut round_tmp_8: uint32_t = 0;
    round_tmp_8 = (*key.offset(2 as libc::c_int as isize)).wrapping_add(r);
    l
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_8 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_8 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_8 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_8 >> 24 as libc::c_int) as usize];
    round_tmp_8 = (*key.offset(3 as libc::c_int as isize)).wrapping_add(l);
    r
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_8 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_8 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_8 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_8 >> 24 as libc::c_int) as usize];
    let mut round_tmp_9: uint32_t = 0;
    round_tmp_9 = (*key.offset(4 as libc::c_int as isize)).wrapping_add(r);
    l
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_9 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_9 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_9 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_9 >> 24 as libc::c_int) as usize];
    round_tmp_9 = (*key.offset(5 as libc::c_int as isize)).wrapping_add(l);
    r
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_9 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_9 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_9 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_9 >> 24 as libc::c_int) as usize];
    let mut round_tmp_10: uint32_t = 0;
    round_tmp_10 = (*key.offset(6 as libc::c_int as isize)).wrapping_add(r);
    l
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_10 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_10 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_10 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_10 >> 24 as libc::c_int) as usize];
    round_tmp_10 = (*key.offset(7 as libc::c_int as isize)).wrapping_add(l);
    r
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_10 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_10 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_10 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_10 >> 24 as libc::c_int) as usize];
    let mut round_tmp_11: uint32_t = 0;
    round_tmp_11 = (*key.offset(7 as libc::c_int as isize)).wrapping_add(r);
    l
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_11 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_11 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_11 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_11 >> 24 as libc::c_int) as usize];
    round_tmp_11 = (*key.offset(6 as libc::c_int as isize)).wrapping_add(l);
    r
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_11 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_11 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_11 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_11 >> 24 as libc::c_int) as usize];
    let mut round_tmp_12: uint32_t = 0;
    round_tmp_12 = (*key.offset(5 as libc::c_int as isize)).wrapping_add(r);
    l
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_12 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_12 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_12 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_12 >> 24 as libc::c_int) as usize];
    round_tmp_12 = (*key.offset(4 as libc::c_int as isize)).wrapping_add(l);
    r
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_12 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_12 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_12 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_12 >> 24 as libc::c_int) as usize];
    let mut round_tmp_13: uint32_t = 0;
    round_tmp_13 = (*key.offset(3 as libc::c_int as isize)).wrapping_add(r);
    l
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_13 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_13 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_13 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_13 >> 24 as libc::c_int) as usize];
    round_tmp_13 = (*key.offset(2 as libc::c_int as isize)).wrapping_add(l);
    r
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_13 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_13 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_13 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_13 >> 24 as libc::c_int) as usize];
    let mut round_tmp_14: uint32_t = 0;
    round_tmp_14 = (*key.offset(1 as libc::c_int as isize)).wrapping_add(r);
    l
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_14 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_14 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_14 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_14 >> 24 as libc::c_int) as usize];
    round_tmp_14 = (*key.offset(0 as libc::c_int as isize)).wrapping_add(l);
    r
        ^= (*sbox
            .offset(
                0 as libc::c_int as isize,
            ))[(round_tmp_14 & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    1 as libc::c_int as isize,
                ))[(round_tmp_14 >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    2 as libc::c_int as isize,
                ))[(round_tmp_14 >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ (*sbox
                .offset(
                    3 as libc::c_int as isize,
                ))[(round_tmp_14 >> 24 as libc::c_int) as usize];
    *out = l;
    *out.offset(1 as libc::c_int as isize) = r;
}
