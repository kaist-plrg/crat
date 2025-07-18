use ::libc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA1Context {
    pub Message_Digest: [libc::c_uint; 5],
    pub Length_Low: libc::c_uint,
    pub Length_High: libc::c_uint,
    pub Message_Block: [libc::c_uchar; 64],
    pub Message_Block_Index: libc::c_int,
    pub Computed: libc::c_int,
    pub Corrupted: libc::c_int,
}
pub unsafe extern "C" fn SHA1Reset(mut context: *mut SHA1Context) {
    (*context).Length_Low = 0 as libc::c_int as libc::c_uint;
    (*context).Length_High = 0 as libc::c_int as libc::c_uint;
    (*context).Message_Block_Index = 0 as libc::c_int;
    (*context)
        .Message_Digest[0 as libc::c_int
        as usize] = 0x67452301 as libc::c_int as libc::c_uint;
    (*context).Message_Digest[1 as libc::c_int as usize] = 0xefcdab89 as libc::c_uint;
    (*context).Message_Digest[2 as libc::c_int as usize] = 0x98badcfe as libc::c_uint;
    (*context)
        .Message_Digest[3 as libc::c_int
        as usize] = 0x10325476 as libc::c_int as libc::c_uint;
    (*context).Message_Digest[4 as libc::c_int as usize] = 0xc3d2e1f0 as libc::c_uint;
    (*context).Computed = 0 as libc::c_int;
    (*context).Corrupted = 0 as libc::c_int;
}
pub unsafe extern "C" fn SHA1Result(mut context: *mut SHA1Context) -> libc::c_int {
    if (*context).Corrupted != 0 {
        return 0 as libc::c_int;
    }
    if (*context).Computed == 0 {
        SHA1PadMessage(context);
        (*context).Computed = 1 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn SHA1Input(
    mut context: *mut SHA1Context,
    mut message_array: *const libc::c_uchar,
    mut length: libc::c_uint,
) {
    if length == 0 {
        return;
    }
    if (*context).Computed != 0 || (*context).Corrupted != 0 {
        (*context).Corrupted = 1 as libc::c_int;
        return;
    }
    loop {
        let fresh0 = length;
        length = length.wrapping_sub(1);
        if !(fresh0 != 0 && (*context).Corrupted == 0) {
            break;
        }
        let fresh1 = (*context).Message_Block_Index;
        (*context).Message_Block_Index = (*context).Message_Block_Index + 1;
        (*context)
            .Message_Block[fresh1
            as usize] = (*message_array as libc::c_int & 0xff as libc::c_int)
            as libc::c_uchar;
        (*context)
            .Length_Low = ((*context).Length_Low)
            .wrapping_add(8 as libc::c_int as libc::c_uint);
        (*context).Length_Low &= 0xffffffff as libc::c_uint;
        if (*context).Length_Low == 0 as libc::c_int as libc::c_uint {
            (*context).Length_High = ((*context).Length_High).wrapping_add(1);
            (*context).Length_High;
            (*context).Length_High &= 0xffffffff as libc::c_uint;
            if (*context).Length_High == 0 as libc::c_int as libc::c_uint {
                (*context).Corrupted = 1 as libc::c_int;
            }
        }
        if (*context).Message_Block_Index == 64 as libc::c_int {
            SHA1ProcessMessageBlock(context);
        }
        message_array = message_array.offset(1);
        message_array;
    };
}
pub unsafe extern "C" fn SHA1ProcessMessageBlock(mut context: *mut SHA1Context) {
    let K: [libc::c_uint; 4] = [
        0x5a827999 as libc::c_int as libc::c_uint,
        0x6ed9eba1 as libc::c_int as libc::c_uint,
        0x8f1bbcdc as libc::c_uint,
        0xca62c1d6 as libc::c_uint,
    ];
    let mut t: libc::c_int = 0;
    let mut temp: libc::c_uint = 0;
    let mut W: [libc::c_uint; 80] = [0; 80];
    let mut A: libc::c_uint = 0;
    let mut B: libc::c_uint = 0;
    let mut C: libc::c_uint = 0;
    let mut D: libc::c_uint = 0;
    let mut E: libc::c_uint = 0;
    t = 0 as libc::c_int;
    while t < 16 as libc::c_int {
        W[t
            as usize] = ((*context).Message_Block[(t * 4 as libc::c_int) as usize]
            as libc::c_uint) << 24 as libc::c_int;
        W[t as usize]
            |= ((*context)
                .Message_Block[(t * 4 as libc::c_int + 1 as libc::c_int) as usize]
                as libc::c_uint) << 16 as libc::c_int;
        W[t as usize]
            |= ((*context)
                .Message_Block[(t * 4 as libc::c_int + 2 as libc::c_int) as usize]
                as libc::c_uint) << 8 as libc::c_int;
        W[t as usize]
            |= (*context)
                .Message_Block[(t * 4 as libc::c_int + 3 as libc::c_int) as usize]
                as libc::c_uint;
        t += 1;
        t;
    }
    t = 16 as libc::c_int;
    while t < 80 as libc::c_int {
        W[t
            as usize] = (W[(t - 3 as libc::c_int) as usize]
            ^ W[(t - 8 as libc::c_int) as usize] ^ W[(t - 14 as libc::c_int) as usize]
            ^ W[(t - 16 as libc::c_int) as usize]) << 1 as libc::c_int
            & 0xffffffff as libc::c_uint
            | (W[(t - 3 as libc::c_int) as usize] ^ W[(t - 8 as libc::c_int) as usize]
                ^ W[(t - 14 as libc::c_int) as usize]
                ^ W[(t - 16 as libc::c_int) as usize])
                >> 32 as libc::c_int - 1 as libc::c_int;
        t += 1;
        t;
    }
    A = (*context).Message_Digest[0 as libc::c_int as usize];
    B = (*context).Message_Digest[1 as libc::c_int as usize];
    C = (*context).Message_Digest[2 as libc::c_int as usize];
    D = (*context).Message_Digest[3 as libc::c_int as usize];
    E = (*context).Message_Digest[4 as libc::c_int as usize];
    t = 0 as libc::c_int;
    while t < 20 as libc::c_int {
        temp = (A << 5 as libc::c_int & 0xffffffff as libc::c_uint
            | A >> 32 as libc::c_int - 5 as libc::c_int)
            .wrapping_add(B & C | !B & D)
            .wrapping_add(E)
            .wrapping_add(W[t as usize])
            .wrapping_add(K[0 as libc::c_int as usize]);
        temp &= 0xffffffff as libc::c_uint;
        E = D;
        D = C;
        C = B << 30 as libc::c_int & 0xffffffff as libc::c_uint
            | B >> 32 as libc::c_int - 30 as libc::c_int;
        B = A;
        A = temp;
        t += 1;
        t;
    }
    t = 20 as libc::c_int;
    while t < 40 as libc::c_int {
        temp = (A << 5 as libc::c_int & 0xffffffff as libc::c_uint
            | A >> 32 as libc::c_int - 5 as libc::c_int)
            .wrapping_add(B ^ C ^ D)
            .wrapping_add(E)
            .wrapping_add(W[t as usize])
            .wrapping_add(K[1 as libc::c_int as usize]);
        temp &= 0xffffffff as libc::c_uint;
        E = D;
        D = C;
        C = B << 30 as libc::c_int & 0xffffffff as libc::c_uint
            | B >> 32 as libc::c_int - 30 as libc::c_int;
        B = A;
        A = temp;
        t += 1;
        t;
    }
    t = 40 as libc::c_int;
    while t < 60 as libc::c_int {
        temp = (A << 5 as libc::c_int & 0xffffffff as libc::c_uint
            | A >> 32 as libc::c_int - 5 as libc::c_int)
            .wrapping_add(B & C | B & D | C & D)
            .wrapping_add(E)
            .wrapping_add(W[t as usize])
            .wrapping_add(K[2 as libc::c_int as usize]);
        temp &= 0xffffffff as libc::c_uint;
        E = D;
        D = C;
        C = B << 30 as libc::c_int & 0xffffffff as libc::c_uint
            | B >> 32 as libc::c_int - 30 as libc::c_int;
        B = A;
        A = temp;
        t += 1;
        t;
    }
    t = 60 as libc::c_int;
    while t < 80 as libc::c_int {
        temp = (A << 5 as libc::c_int & 0xffffffff as libc::c_uint
            | A >> 32 as libc::c_int - 5 as libc::c_int)
            .wrapping_add(B ^ C ^ D)
            .wrapping_add(E)
            .wrapping_add(W[t as usize])
            .wrapping_add(K[3 as libc::c_int as usize]);
        temp &= 0xffffffff as libc::c_uint;
        E = D;
        D = C;
        C = B << 30 as libc::c_int & 0xffffffff as libc::c_uint
            | B >> 32 as libc::c_int - 30 as libc::c_int;
        B = A;
        A = temp;
        t += 1;
        t;
    }
    (*context)
        .Message_Digest[0 as libc::c_int
        as usize] = ((*context).Message_Digest[0 as libc::c_int as usize])
        .wrapping_add(A) & 0xffffffff as libc::c_uint;
    (*context)
        .Message_Digest[1 as libc::c_int
        as usize] = ((*context).Message_Digest[1 as libc::c_int as usize])
        .wrapping_add(B) & 0xffffffff as libc::c_uint;
    (*context)
        .Message_Digest[2 as libc::c_int
        as usize] = ((*context).Message_Digest[2 as libc::c_int as usize])
        .wrapping_add(C) & 0xffffffff as libc::c_uint;
    (*context)
        .Message_Digest[3 as libc::c_int
        as usize] = ((*context).Message_Digest[3 as libc::c_int as usize])
        .wrapping_add(D) & 0xffffffff as libc::c_uint;
    (*context)
        .Message_Digest[4 as libc::c_int
        as usize] = ((*context).Message_Digest[4 as libc::c_int as usize])
        .wrapping_add(E) & 0xffffffff as libc::c_uint;
    (*context).Message_Block_Index = 0 as libc::c_int;
}
pub unsafe extern "C" fn SHA1PadMessage(mut context: *mut SHA1Context) {
    if (*context).Message_Block_Index > 55 as libc::c_int {
        let fresh2 = (*context).Message_Block_Index;
        (*context).Message_Block_Index = (*context).Message_Block_Index + 1;
        (*context).Message_Block[fresh2 as usize] = 0x80 as libc::c_int as libc::c_uchar;
        while (*context).Message_Block_Index < 64 as libc::c_int {
            let fresh3 = (*context).Message_Block_Index;
            (*context).Message_Block_Index = (*context).Message_Block_Index + 1;
            (*context)
                .Message_Block[fresh3 as usize] = 0 as libc::c_int as libc::c_uchar;
        }
        SHA1ProcessMessageBlock(context);
        while (*context).Message_Block_Index < 56 as libc::c_int {
            let fresh4 = (*context).Message_Block_Index;
            (*context).Message_Block_Index = (*context).Message_Block_Index + 1;
            (*context)
                .Message_Block[fresh4 as usize] = 0 as libc::c_int as libc::c_uchar;
        }
    } else {
        let fresh5 = (*context).Message_Block_Index;
        (*context).Message_Block_Index = (*context).Message_Block_Index + 1;
        (*context).Message_Block[fresh5 as usize] = 0x80 as libc::c_int as libc::c_uchar;
        while (*context).Message_Block_Index < 56 as libc::c_int {
            let fresh6 = (*context).Message_Block_Index;
            (*context).Message_Block_Index = (*context).Message_Block_Index + 1;
            (*context)
                .Message_Block[fresh6 as usize] = 0 as libc::c_int as libc::c_uchar;
        }
    }
    (*context)
        .Message_Block[56 as libc::c_int
        as usize] = ((*context).Length_High >> 24 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    (*context)
        .Message_Block[57 as libc::c_int
        as usize] = ((*context).Length_High >> 16 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    (*context)
        .Message_Block[58 as libc::c_int
        as usize] = ((*context).Length_High >> 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    (*context)
        .Message_Block[59 as libc::c_int
        as usize] = ((*context).Length_High & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    (*context)
        .Message_Block[60 as libc::c_int
        as usize] = ((*context).Length_Low >> 24 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    (*context)
        .Message_Block[61 as libc::c_int
        as usize] = ((*context).Length_Low >> 16 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    (*context)
        .Message_Block[62 as libc::c_int
        as usize] = ((*context).Length_Low >> 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    (*context)
        .Message_Block[63 as libc::c_int
        as usize] = ((*context).Length_Low & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    SHA1ProcessMessageBlock(context);
}
