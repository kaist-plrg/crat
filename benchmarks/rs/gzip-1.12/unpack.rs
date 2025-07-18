use ::libc;
extern "C" {
    static mut inbuf: [uch; 0];
    static mut outbuf: [uch; 0];
    static mut window: [uch; 0];
    static mut insize: libc::c_uint;
    static mut inptr: libc::c_uint;
    static mut outcnt: libc::c_uint;
    static mut bytes_out: off_t;
    static mut ifd: libc::c_int;
    static mut ofd: libc::c_int;
    fn gzip_error(m: *const libc::c_char);
    fn flush_window();
    fn fill_inbuf(eof_ok: libc::c_int) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
pub type uch = libc::c_uchar;
pub type ulg = libc::c_ulong;
static mut orig_len: ulg = 0;
static mut max_len: libc::c_int = 0;
static mut literal: [uch; 256] = [0; 256];
static mut lit_base: [libc::c_int; 26] = [0; 26];
static mut leaves: [libc::c_int; 26] = [0; 26];
static mut parents: [libc::c_int; 26] = [0; 26];
static mut peek_bits: libc::c_int = 0;
static mut bitbuf: ulg = 0;
static mut valid: libc::c_int = 0;
unsafe extern "C" fn read_byte() -> libc::c_uchar {
    let mut b: libc::c_int = if inptr < insize {
        let fresh0 = inptr;
        inptr = inptr.wrapping_add(1);
        *inbuf.as_mut_ptr().offset(fresh0 as isize) as libc::c_int
    } else {
        fill_inbuf(0 as libc::c_int)
    };
    if b < 0 as libc::c_int {
        gzip_error(
            b"invalid compressed data -- unexpected end of file\0" as *const u8
                as *const libc::c_char,
        );
    }
    return b as libc::c_uchar;
}
unsafe extern "C" fn read_tree() {
    let mut len: libc::c_int = 0;
    let mut base: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut max_leaves: libc::c_int = 1 as libc::c_int;
    orig_len = 0 as libc::c_int as ulg;
    n = 1 as libc::c_int;
    while n <= 4 as libc::c_int {
        orig_len = orig_len << 8 as libc::c_int | read_byte() as libc::c_ulong;
        n += 1;
        n;
    }
    max_len = read_byte() as libc::c_int;
    if !((0 as libc::c_int) < max_len && max_len <= 25 as libc::c_int) {
        gzip_error(
            b"invalid compressed data -- Huffman code bit length out of range\0"
                as *const u8 as *const libc::c_char,
        );
    }
    n = 0 as libc::c_int;
    len = 1 as libc::c_int;
    while len <= max_len {
        leaves[len as usize] = read_byte() as libc::c_int;
        if (max_leaves - (len == max_len) as libc::c_int) < leaves[len as usize] {
            gzip_error(
                b"too many leaves in Huffman tree\0" as *const u8 as *const libc::c_char,
            );
        }
        max_leaves = (max_leaves - leaves[len as usize] + 1 as libc::c_int)
            * 2 as libc::c_int - 1 as libc::c_int;
        n += leaves[len as usize];
        len += 1;
        len;
    }
    if 256 as libc::c_int <= n {
        gzip_error(
            b"too many leaves in Huffman tree\0" as *const u8 as *const libc::c_char,
        );
    }
    leaves[max_len as usize] += 1;
    leaves[max_len as usize];
    base = 0 as libc::c_int;
    len = 1 as libc::c_int;
    while len <= max_len {
        lit_base[len as usize] = base;
        n = leaves[len as usize];
        while n > 0 as libc::c_int {
            let fresh1 = base;
            base = base + 1;
            literal[fresh1 as usize] = read_byte();
            n -= 1;
            n;
        }
        len += 1;
        len;
    }
    leaves[max_len as usize] += 1;
    leaves[max_len as usize];
}
unsafe extern "C" fn build_tree() {
    let mut nodes: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0;
    let mut prefixp: *mut uch = 0 as *mut uch;
    len = max_len;
    while len >= 1 as libc::c_int {
        nodes >>= 1 as libc::c_int;
        parents[len as usize] = nodes;
        lit_base[len as usize] -= nodes;
        nodes += leaves[len as usize];
        len -= 1;
        len;
    }
    if nodes >> 1 as libc::c_int != 1 as libc::c_int {
        gzip_error(
            b"too few leaves in Huffman tree\0" as *const u8 as *const libc::c_char,
        );
    }
    peek_bits = if max_len <= 12 as libc::c_int { max_len } else { 12 as libc::c_int };
    prefixp = &mut *outbuf
        .as_mut_ptr()
        .offset(((1 as libc::c_int) << peek_bits) as isize) as *mut uch;
    len = 1 as libc::c_int;
    while len <= peek_bits {
        let mut prefixes: libc::c_int = leaves[len as usize] << peek_bits - len;
        loop {
            let fresh2 = prefixes;
            prefixes = prefixes - 1;
            if !(fresh2 != 0) {
                break;
            }
            prefixp = prefixp.offset(-1);
            *prefixp = len as uch;
        }
        len += 1;
        len;
    }
    while prefixp > outbuf.as_mut_ptr() {
        prefixp = prefixp.offset(-1);
        *prefixp = 0 as libc::c_int as uch;
    }
}
pub unsafe extern "C" fn unpack(
    mut in_0: libc::c_int,
    mut out: libc::c_int,
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut eob: libc::c_uint = 0;
    let mut peek: libc::c_uint = 0;
    let mut peek_mask: libc::c_uint = 0;
    ifd = in_0;
    ofd = out;
    read_tree();
    build_tree();
    valid = 0 as libc::c_int;
    bitbuf = 0 as libc::c_int as ulg;
    peek_mask = (((1 as libc::c_int) << peek_bits) - 1 as libc::c_int) as libc::c_uint;
    eob = (leaves[max_len as usize] - 1 as libc::c_int) as libc::c_uint;
    loop {
        while valid < peek_bits {
            bitbuf = bitbuf << 8 as libc::c_int | read_byte() as libc::c_ulong;
            valid += 8 as libc::c_int;
        }
        peek = (bitbuf >> valid - peek_bits & peek_mask as libc::c_ulong)
            as libc::c_uint;
        len = *outbuf.as_mut_ptr().offset(peek as isize) as libc::c_int;
        if len > 0 as libc::c_int {
            peek >>= peek_bits - len;
        } else {
            let mut mask: ulg = peek_mask as ulg;
            len = peek_bits;
            while peek < parents[len as usize] as libc::c_uint {
                len += 1;
                len;
                mask = (mask << 1 as libc::c_int)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
                while valid < len {
                    bitbuf = bitbuf << 8 as libc::c_int | read_byte() as libc::c_ulong;
                    valid += 8 as libc::c_int;
                }
                peek = (bitbuf >> valid - len & mask) as libc::c_uint;
            }
        }
        if peek == eob && len == max_len {
            break;
        }
        let fresh3 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *window
            .as_mut_ptr()
            .offset(
                fresh3 as isize,
            ) = literal[peek.wrapping_add(lit_base[len as usize] as libc::c_uint)
            as usize];
        if outcnt == 0x8000 as libc::c_int as libc::c_uint {
            flush_window();
        }
        valid -= len;
    }
    flush_window();
    if orig_len != (bytes_out & 0xffffffff as libc::c_uint as libc::c_long) as ulg {
        gzip_error(
            b"invalid compressed data--length error\0" as *const u8
                as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
