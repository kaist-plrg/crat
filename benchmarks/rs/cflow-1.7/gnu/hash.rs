use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_tuning {
    pub shrink_threshold: libc::c_float,
    pub shrink_factor: libc::c_float,
    pub growth_threshold: libc::c_float,
    pub growth_factor: libc::c_float,
    pub is_n_buckets: bool,
}
pub type Hash_tuning = hash_tuning;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table {
    pub bucket: *mut hash_entry,
    pub bucket_limit: *const hash_entry,
    pub n_buckets: size_t,
    pub n_buckets_used: size_t,
    pub n_entries: size_t,
    pub tuning: *const Hash_tuning,
    pub hasher: Hash_hasher,
    pub comparator: Hash_comparator,
    pub data_freer: Hash_data_freer,
    pub free_entry_list: *mut hash_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_entry {
    pub data: *mut libc::c_void,
    pub next: *mut hash_entry,
}
pub type Hash_data_freer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type Hash_comparator = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_hasher = Option::<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
pub type Hash_table = hash_table;
pub type Hash_processor = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> bool,
>;
#[inline]
unsafe extern "C" fn rotr_sz(mut x: size_t, mut n: libc::c_int) -> size_t {
    return (x >> n
        | x
            << (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_sub(n as libc::c_ulong))
        & 18446744073709551615 as libc::c_ulong;
}
static mut default_tuning: Hash_tuning = {
    let mut init = hash_tuning {
        shrink_threshold: 0.0f32,
        shrink_factor: 1.0f32,
        growth_threshold: 0.8f32,
        growth_factor: 1.414f32,
        is_n_buckets: 0 as libc::c_int != 0,
    };
    init
};
pub unsafe extern "C" fn hash_get_n_buckets(mut table: *const Hash_table) -> size_t {
    return (*table).n_buckets;
}
pub unsafe extern "C" fn hash_get_n_buckets_used(
    mut table: *const Hash_table,
) -> size_t {
    return (*table).n_buckets_used;
}
pub unsafe extern "C" fn hash_get_n_entries(mut table: *const Hash_table) -> size_t {
    return (*table).n_entries;
}
pub unsafe extern "C" fn hash_get_max_bucket_length(
    mut table: *const Hash_table,
) -> size_t {
    let mut bucket: *const hash_entry = 0 as *const hash_entry;
    let mut max_bucket_length: size_t = 0 as libc::c_int as size_t;
    bucket = (*table).bucket;
    while bucket < (*table).bucket_limit {
        if !((*bucket).data).is_null() {
            let mut cursor: *const hash_entry = bucket;
            let mut bucket_length: size_t = 1 as libc::c_int as size_t;
            loop {
                cursor = (*cursor).next;
                if cursor.is_null() {
                    break;
                }
                bucket_length = bucket_length.wrapping_add(1);
                bucket_length;
            }
            if bucket_length > max_bucket_length {
                max_bucket_length = bucket_length;
            }
        }
        bucket = bucket.offset(1);
        bucket;
    }
    return max_bucket_length;
}
pub unsafe extern "C" fn hash_table_ok(mut table: *const Hash_table) -> bool {
    let mut bucket: *const hash_entry = 0 as *const hash_entry;
    let mut n_buckets_used: size_t = 0 as libc::c_int as size_t;
    let mut n_entries: size_t = 0 as libc::c_int as size_t;
    bucket = (*table).bucket;
    while bucket < (*table).bucket_limit {
        if !((*bucket).data).is_null() {
            let mut cursor: *const hash_entry = bucket;
            n_buckets_used = n_buckets_used.wrapping_add(1);
            n_buckets_used;
            n_entries = n_entries.wrapping_add(1);
            n_entries;
            loop {
                cursor = (*cursor).next;
                if cursor.is_null() {
                    break;
                }
                n_entries = n_entries.wrapping_add(1);
                n_entries;
            }
        }
        bucket = bucket.offset(1);
        bucket;
    }
    if n_buckets_used == (*table).n_buckets_used && n_entries == (*table).n_entries {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn hash_print_statistics(
    mut table: *const Hash_table,
    mut stream: *mut FILE,
) {
    let mut n_entries: size_t = hash_get_n_entries(table);
    let mut n_buckets: size_t = hash_get_n_buckets(table);
    let mut n_buckets_used: size_t = hash_get_n_buckets_used(table);
    let mut max_bucket_length: size_t = hash_get_max_bucket_length(table);
    fprintf(
        stream,
        b"# entries:         %lu\n\0" as *const u8 as *const libc::c_char,
        n_entries,
    );
    fprintf(
        stream,
        b"# buckets:         %lu\n\0" as *const u8 as *const libc::c_char,
        n_buckets,
    );
    fprintf(
        stream,
        b"# buckets used:    %lu (%.2f%%)\n\0" as *const u8 as *const libc::c_char,
        n_buckets_used,
        100.0f64 * n_buckets_used as libc::c_double / n_buckets as libc::c_double,
    );
    fprintf(
        stream,
        b"max bucket length: %lu\n\0" as *const u8 as *const libc::c_char,
        max_bucket_length,
    );
}
unsafe extern "C" fn safe_hasher(
    mut table: *const Hash_table,
    mut key: *const libc::c_void,
) -> *mut hash_entry {
    let mut n: size_t = ((*table).hasher).unwrap()(key, (*table).n_buckets);
    if !(n < (*table).n_buckets) {
        abort();
    }
    return ((*table).bucket).offset(n as isize);
}
pub unsafe extern "C" fn hash_lookup(
    mut table: *const Hash_table,
    mut entry: *const libc::c_void,
) -> *mut libc::c_void {
    let mut bucket: *const hash_entry = safe_hasher(table, entry);
    let mut cursor: *const hash_entry = 0 as *const hash_entry;
    if ((*bucket).data).is_null() {
        return 0 as *mut libc::c_void;
    }
    cursor = bucket;
    while !cursor.is_null() {
        if entry == (*cursor).data as *const libc::c_void
            || ((*table).comparator).unwrap()(entry, (*cursor).data) as libc::c_int != 0
        {
            return (*cursor).data;
        }
        cursor = (*cursor).next;
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn hash_get_first(
    mut table: *const Hash_table,
) -> *mut libc::c_void {
    let mut bucket: *const hash_entry = 0 as *const hash_entry;
    if (*table).n_entries == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    bucket = (*table).bucket;
    loop {
        if !(bucket < (*table).bucket_limit) {
            abort();
        } else if !((*bucket).data).is_null() {
            return (*bucket).data
        }
        bucket = bucket.offset(1);
        bucket;
    };
}
pub unsafe extern "C" fn hash_get_next(
    mut table: *const Hash_table,
    mut entry: *const libc::c_void,
) -> *mut libc::c_void {
    let mut bucket: *const hash_entry = safe_hasher(table, entry);
    let mut cursor: *const hash_entry = 0 as *const hash_entry;
    cursor = bucket;
    loop {
        if (*cursor).data == entry as *mut libc::c_void && !((*cursor).next).is_null() {
            return (*(*cursor).next).data;
        }
        cursor = (*cursor).next;
        if cursor.is_null() {
            break;
        }
    }
    loop {
        bucket = bucket.offset(1);
        if !(bucket < (*table).bucket_limit) {
            break;
        }
        if !((*bucket).data).is_null() {
            return (*bucket).data;
        }
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn hash_get_entries(
    mut table: *const Hash_table,
    mut buffer: *mut *mut libc::c_void,
    mut buffer_size: size_t,
) -> size_t {
    let mut counter: size_t = 0 as libc::c_int as size_t;
    let mut bucket: *const hash_entry = 0 as *const hash_entry;
    let mut cursor: *const hash_entry = 0 as *const hash_entry;
    bucket = (*table).bucket;
    while bucket < (*table).bucket_limit {
        if !((*bucket).data).is_null() {
            cursor = bucket;
            while !cursor.is_null() {
                if counter >= buffer_size {
                    return counter;
                }
                let fresh0 = counter;
                counter = counter.wrapping_add(1);
                let ref mut fresh1 = *buffer.offset(fresh0 as isize);
                *fresh1 = (*cursor).data;
                cursor = (*cursor).next;
            }
        }
        bucket = bucket.offset(1);
        bucket;
    }
    return counter;
}
pub unsafe extern "C" fn hash_do_for_each(
    mut table: *const Hash_table,
    mut processor: Hash_processor,
    mut processor_data: *mut libc::c_void,
) -> size_t {
    let mut counter: size_t = 0 as libc::c_int as size_t;
    let mut bucket: *const hash_entry = 0 as *const hash_entry;
    let mut cursor: *const hash_entry = 0 as *const hash_entry;
    bucket = (*table).bucket;
    while bucket < (*table).bucket_limit {
        if !((*bucket).data).is_null() {
            cursor = bucket;
            while !cursor.is_null() {
                if !processor.unwrap()((*cursor).data, processor_data) {
                    return counter;
                }
                counter = counter.wrapping_add(1);
                counter;
                cursor = (*cursor).next;
            }
        }
        bucket = bucket.offset(1);
        bucket;
    }
    return counter;
}
pub unsafe extern "C" fn hash_string(
    mut string: *const libc::c_char,
    mut n_buckets: size_t,
) -> size_t {
    let mut value: size_t = 0 as libc::c_int as size_t;
    let mut ch: libc::c_uchar = 0;
    loop {
        ch = *string as libc::c_uchar;
        if !(ch != 0) {
            break;
        }
        value = value
            .wrapping_mul(31 as libc::c_int as libc::c_ulong)
            .wrapping_add(ch as libc::c_ulong)
            .wrapping_rem(n_buckets);
        string = string.offset(1);
        string;
    }
    return value;
}
unsafe extern "C" fn is_prime(mut candidate: size_t) -> bool {
    let mut divisor: size_t = 3 as libc::c_int as size_t;
    let mut square: size_t = divisor.wrapping_mul(divisor);
    while square < candidate && candidate.wrapping_rem(divisor) != 0 {
        divisor = divisor.wrapping_add(1);
        divisor;
        square = (square as libc::c_ulong)
            .wrapping_add((4 as libc::c_int as libc::c_ulong).wrapping_mul(divisor))
            as size_t as size_t;
        divisor = divisor.wrapping_add(1);
        divisor;
    }
    return if candidate.wrapping_rem(divisor) != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0;
}
unsafe extern "C" fn next_prime(mut candidate: size_t) -> size_t {
    if candidate < 10 as libc::c_int as libc::c_ulong {
        candidate = 10 as libc::c_int as size_t;
    }
    candidate |= 1 as libc::c_int as libc::c_ulong;
    while 18446744073709551615 as libc::c_ulong != candidate && !is_prime(candidate) {
        candidate = (candidate as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
    }
    return candidate;
}
pub unsafe extern "C" fn hash_reset_tuning(mut tuning: *mut Hash_tuning) {
    *tuning = default_tuning;
}
unsafe extern "C" fn raw_hasher(mut data: *const libc::c_void, mut n: size_t) -> size_t {
    let mut val: size_t = rotr_sz(data as size_t, 3 as libc::c_int);
    return val.wrapping_rem(n);
}
unsafe extern "C" fn raw_comparator(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> bool {
    return a == b;
}
unsafe extern "C" fn check_tuning(mut table: *mut Hash_table) -> bool {
    let mut tuning: *const Hash_tuning = (*table).tuning;
    let mut epsilon: libc::c_float = 0.;
    if tuning == &default_tuning as *const Hash_tuning {
        return 1 as libc::c_int != 0;
    }
    epsilon = 0.1f32;
    if epsilon < (*tuning).growth_threshold
        && (*tuning).growth_threshold < 1 as libc::c_int as libc::c_float - epsilon
        && 1 as libc::c_int as libc::c_float + epsilon < (*tuning).growth_factor
        && 0 as libc::c_int as libc::c_float <= (*tuning).shrink_threshold
        && (*tuning).shrink_threshold + epsilon < (*tuning).shrink_factor
        && (*tuning).shrink_factor <= 1 as libc::c_int as libc::c_float
        && (*tuning).shrink_threshold + epsilon < (*tuning).growth_threshold
    {
        return 1 as libc::c_int != 0;
    }
    (*table).tuning = &default_tuning;
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn compute_bucket_size(
    mut candidate: size_t,
    mut tuning: *const Hash_tuning,
) -> size_t {
    if !(*tuning).is_n_buckets {
        let mut new_candidate: libc::c_float = candidate as libc::c_float
            / (*tuning).growth_threshold;
        if 18446744073709551615 as libc::c_ulong as libc::c_float <= new_candidate {
            return 0 as libc::c_int as size_t;
        }
        candidate = new_candidate as size_t;
    }
    candidate = next_prime(candidate);
    if (if (9223372036854775807 as libc::c_long as libc::c_ulong)
        < 18446744073709551615 as libc::c_ulong
    {
        9223372036854775807 as libc::c_long as libc::c_ulong
    } else {
        (18446744073709551615 as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    })
        .wrapping_div(::std::mem::size_of::<*mut hash_entry>() as libc::c_ulong)
        < candidate
    {
        return 0 as libc::c_int as size_t;
    }
    return candidate;
}
pub unsafe extern "C" fn hash_initialize(
    mut candidate: size_t,
    mut tuning: *const Hash_tuning,
    mut hasher: Hash_hasher,
    mut comparator: Hash_comparator,
    mut data_freer: Hash_data_freer,
) -> *mut Hash_table {
    let mut table: *mut Hash_table = 0 as *mut Hash_table;
    if hasher.is_none() {
        hasher = Some(
            raw_hasher as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
        );
    }
    if comparator.is_none() {
        comparator = Some(
            raw_comparator
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
        );
    }
    table = malloc(::std::mem::size_of::<Hash_table>() as libc::c_ulong)
        as *mut Hash_table;
    if table.is_null() {
        return 0 as *mut Hash_table;
    }
    if tuning.is_null() {
        tuning = &default_tuning;
    }
    (*table).tuning = tuning;
    if check_tuning(table) {
        (*table).n_buckets = compute_bucket_size(candidate, tuning);
        if !((*table).n_buckets == 0) {
            (*table)
                .bucket = calloc(
                (*table).n_buckets,
                ::std::mem::size_of::<hash_entry>() as libc::c_ulong,
            ) as *mut hash_entry;
            if !((*table).bucket).is_null() {
                (*table)
                    .bucket_limit = ((*table).bucket)
                    .offset((*table).n_buckets as isize);
                (*table).n_buckets_used = 0 as libc::c_int as size_t;
                (*table).n_entries = 0 as libc::c_int as size_t;
                (*table).hasher = hasher;
                (*table).comparator = comparator;
                (*table).data_freer = data_freer;
                (*table).free_entry_list = 0 as *mut hash_entry;
                return table;
            }
        }
    }
    free(table as *mut libc::c_void);
    return 0 as *mut Hash_table;
}
pub unsafe extern "C" fn hash_clear(mut table: *mut Hash_table) {
    let mut bucket: *mut hash_entry = 0 as *mut hash_entry;
    bucket = (*table).bucket;
    while bucket < (*table).bucket_limit as *mut hash_entry {
        if !((*bucket).data).is_null() {
            let mut cursor: *mut hash_entry = 0 as *mut hash_entry;
            let mut next: *mut hash_entry = 0 as *mut hash_entry;
            cursor = (*bucket).next;
            while !cursor.is_null() {
                if ((*table).data_freer).is_some() {
                    ((*table).data_freer).unwrap()((*cursor).data);
                }
                (*cursor).data = 0 as *mut libc::c_void;
                next = (*cursor).next;
                (*cursor).next = (*table).free_entry_list;
                (*table).free_entry_list = cursor;
                cursor = next;
            }
            if ((*table).data_freer).is_some() {
                ((*table).data_freer).unwrap()((*bucket).data);
            }
            (*bucket).data = 0 as *mut libc::c_void;
            (*bucket).next = 0 as *mut hash_entry;
        }
        bucket = bucket.offset(1);
        bucket;
    }
    (*table).n_buckets_used = 0 as libc::c_int as size_t;
    (*table).n_entries = 0 as libc::c_int as size_t;
}
pub unsafe extern "C" fn hash_free(mut table: *mut Hash_table) {
    let mut bucket: *mut hash_entry = 0 as *mut hash_entry;
    let mut cursor: *mut hash_entry = 0 as *mut hash_entry;
    let mut next: *mut hash_entry = 0 as *mut hash_entry;
    if ((*table).data_freer).is_some() && (*table).n_entries != 0 {
        bucket = (*table).bucket;
        while bucket < (*table).bucket_limit as *mut hash_entry {
            if !((*bucket).data).is_null() {
                cursor = bucket;
                while !cursor.is_null() {
                    ((*table).data_freer).unwrap()((*cursor).data);
                    cursor = (*cursor).next;
                }
            }
            bucket = bucket.offset(1);
            bucket;
        }
    }
    bucket = (*table).bucket;
    while bucket < (*table).bucket_limit as *mut hash_entry {
        cursor = (*bucket).next;
        while !cursor.is_null() {
            next = (*cursor).next;
            free(cursor as *mut libc::c_void);
            cursor = next;
        }
        bucket = bucket.offset(1);
        bucket;
    }
    cursor = (*table).free_entry_list;
    while !cursor.is_null() {
        next = (*cursor).next;
        free(cursor as *mut libc::c_void);
        cursor = next;
    }
    free((*table).bucket as *mut libc::c_void);
    free(table as *mut libc::c_void);
}
unsafe extern "C" fn allocate_entry(mut table: *mut Hash_table) -> *mut hash_entry {
    let mut new: *mut hash_entry = 0 as *mut hash_entry;
    if !((*table).free_entry_list).is_null() {
        new = (*table).free_entry_list;
        (*table).free_entry_list = (*new).next;
    } else {
        new = malloc(::std::mem::size_of::<hash_entry>() as libc::c_ulong)
            as *mut hash_entry;
    }
    return new;
}
unsafe extern "C" fn free_entry(mut table: *mut Hash_table, mut entry: *mut hash_entry) {
    (*entry).data = 0 as *mut libc::c_void;
    (*entry).next = (*table).free_entry_list;
    (*table).free_entry_list = entry;
}
unsafe extern "C" fn hash_find_entry(
    mut table: *mut Hash_table,
    mut entry: *const libc::c_void,
    mut bucket_head: *mut *mut hash_entry,
    mut delete: bool,
) -> *mut libc::c_void {
    let mut bucket: *mut hash_entry = safe_hasher(table, entry);
    let mut cursor: *mut hash_entry = 0 as *mut hash_entry;
    *bucket_head = bucket;
    if ((*bucket).data).is_null() {
        return 0 as *mut libc::c_void;
    }
    if entry == (*bucket).data as *const libc::c_void
        || ((*table).comparator).unwrap()(entry, (*bucket).data) as libc::c_int != 0
    {
        let mut data: *mut libc::c_void = (*bucket).data;
        if delete {
            if !((*bucket).next).is_null() {
                let mut next: *mut hash_entry = (*bucket).next;
                *bucket = *next;
                free_entry(table, next);
            } else {
                (*bucket).data = 0 as *mut libc::c_void;
            }
        }
        return data;
    }
    cursor = bucket;
    while !((*cursor).next).is_null() {
        if entry == (*(*cursor).next).data as *const libc::c_void
            || ((*table).comparator).unwrap()(entry, (*(*cursor).next).data)
                as libc::c_int != 0
        {
            let mut data_0: *mut libc::c_void = (*(*cursor).next).data;
            if delete {
                let mut next_0: *mut hash_entry = (*cursor).next;
                (*cursor).next = (*next_0).next;
                free_entry(table, next_0);
            }
            return data_0;
        }
        cursor = (*cursor).next;
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn transfer_entries(
    mut dst: *mut Hash_table,
    mut src: *mut Hash_table,
    mut safe: bool,
) -> bool {
    let mut bucket: *mut hash_entry = 0 as *mut hash_entry;
    let mut cursor: *mut hash_entry = 0 as *mut hash_entry;
    let mut next: *mut hash_entry = 0 as *mut hash_entry;
    bucket = (*src).bucket;
    while bucket < (*src).bucket_limit as *mut hash_entry {
        if !((*bucket).data).is_null() {
            let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
            let mut new_bucket: *mut hash_entry = 0 as *mut hash_entry;
            cursor = (*bucket).next;
            while !cursor.is_null() {
                data = (*cursor).data;
                new_bucket = safe_hasher(dst, data);
                next = (*cursor).next;
                if !((*new_bucket).data).is_null() {
                    (*cursor).next = (*new_bucket).next;
                    (*new_bucket).next = cursor;
                } else {
                    (*new_bucket).data = data;
                    (*dst).n_buckets_used = ((*dst).n_buckets_used).wrapping_add(1);
                    (*dst).n_buckets_used;
                    free_entry(dst, cursor);
                }
                cursor = next;
            }
            data = (*bucket).data;
            (*bucket).next = 0 as *mut hash_entry;
            if !safe {
                new_bucket = safe_hasher(dst, data);
                if !((*new_bucket).data).is_null() {
                    let mut new_entry: *mut hash_entry = allocate_entry(dst);
                    if new_entry.is_null() {
                        return 0 as libc::c_int != 0;
                    }
                    (*new_entry).data = data;
                    (*new_entry).next = (*new_bucket).next;
                    (*new_bucket).next = new_entry;
                } else {
                    (*new_bucket).data = data;
                    (*dst).n_buckets_used = ((*dst).n_buckets_used).wrapping_add(1);
                    (*dst).n_buckets_used;
                }
                (*bucket).data = 0 as *mut libc::c_void;
                (*src).n_buckets_used = ((*src).n_buckets_used).wrapping_sub(1);
                (*src).n_buckets_used;
            }
        }
        bucket = bucket.offset(1);
        bucket;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn hash_rehash(
    mut table: *mut Hash_table,
    mut candidate: size_t,
) -> bool {
    let mut storage: Hash_table = Hash_table {
        bucket: 0 as *mut hash_entry,
        bucket_limit: 0 as *const hash_entry,
        n_buckets: 0,
        n_buckets_used: 0,
        n_entries: 0,
        tuning: 0 as *const Hash_tuning,
        hasher: None,
        comparator: None,
        data_freer: None,
        free_entry_list: 0 as *mut hash_entry,
    };
    let mut new_table: *mut Hash_table = 0 as *mut Hash_table;
    let mut new_size: size_t = compute_bucket_size(candidate, (*table).tuning);
    if new_size == 0 {
        return 0 as libc::c_int != 0;
    }
    if new_size == (*table).n_buckets {
        return 1 as libc::c_int != 0;
    }
    new_table = &mut storage;
    (*new_table)
        .bucket = calloc(new_size, ::std::mem::size_of::<hash_entry>() as libc::c_ulong)
        as *mut hash_entry;
    if ((*new_table).bucket).is_null() {
        return 0 as libc::c_int != 0;
    }
    (*new_table).n_buckets = new_size;
    (*new_table).bucket_limit = ((*new_table).bucket).offset(new_size as isize);
    (*new_table).n_buckets_used = 0 as libc::c_int as size_t;
    (*new_table).n_entries = 0 as libc::c_int as size_t;
    (*new_table).tuning = (*table).tuning;
    (*new_table).hasher = (*table).hasher;
    (*new_table).comparator = (*table).comparator;
    (*new_table).data_freer = (*table).data_freer;
    (*new_table).free_entry_list = (*table).free_entry_list;
    if transfer_entries(new_table, table, 0 as libc::c_int != 0) {
        free((*table).bucket as *mut libc::c_void);
        (*table).bucket = (*new_table).bucket;
        (*table).bucket_limit = (*new_table).bucket_limit;
        (*table).n_buckets = (*new_table).n_buckets;
        (*table).n_buckets_used = (*new_table).n_buckets_used;
        (*table).free_entry_list = (*new_table).free_entry_list;
        return 1 as libc::c_int != 0;
    }
    (*table).free_entry_list = (*new_table).free_entry_list;
    if !(transfer_entries(table, new_table, 1 as libc::c_int != 0) as libc::c_int != 0
        && transfer_entries(table, new_table, 0 as libc::c_int != 0) as libc::c_int != 0)
    {
        abort();
    }
    free((*new_table).bucket as *mut libc::c_void);
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn hash_insert_if_absent(
    mut table: *mut Hash_table,
    mut entry: *const libc::c_void,
    mut matched_ent: *mut *const libc::c_void,
) -> libc::c_int {
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut bucket: *mut hash_entry = 0 as *mut hash_entry;
    if entry.is_null() {
        abort();
    }
    data = hash_find_entry(table, entry, &mut bucket, 0 as libc::c_int != 0);
    if !data.is_null() {
        if !matched_ent.is_null() {
            *matched_ent = data;
        }
        return 0 as libc::c_int;
    }
    if (*table).n_buckets_used as libc::c_float
        > (*(*table).tuning).growth_threshold * (*table).n_buckets as libc::c_float
    {
        check_tuning(table);
        if (*table).n_buckets_used as libc::c_float
            > (*(*table).tuning).growth_threshold * (*table).n_buckets as libc::c_float
        {
            let mut tuning: *const Hash_tuning = (*table).tuning;
            let mut candidate: libc::c_float = if (*tuning).is_n_buckets as libc::c_int
                != 0
            {
                (*table).n_buckets as libc::c_float * (*tuning).growth_factor
            } else {
                (*table).n_buckets as libc::c_float * (*tuning).growth_factor
                    * (*tuning).growth_threshold
            };
            if 18446744073709551615 as libc::c_ulong as libc::c_float <= candidate {
                return -(1 as libc::c_int);
            }
            if !hash_rehash(table, candidate as size_t) {
                return -(1 as libc::c_int);
            }
            if !(hash_find_entry(table, entry, &mut bucket, 0 as libc::c_int != 0))
                .is_null()
            {
                abort();
            }
        }
    }
    if !((*bucket).data).is_null() {
        let mut new_entry: *mut hash_entry = allocate_entry(table);
        if new_entry.is_null() {
            return -(1 as libc::c_int);
        }
        (*new_entry).data = entry as *mut libc::c_void;
        (*new_entry).next = (*bucket).next;
        (*bucket).next = new_entry;
        (*table).n_entries = ((*table).n_entries).wrapping_add(1);
        (*table).n_entries;
        return 1 as libc::c_int;
    }
    (*bucket).data = entry as *mut libc::c_void;
    (*table).n_entries = ((*table).n_entries).wrapping_add(1);
    (*table).n_entries;
    (*table).n_buckets_used = ((*table).n_buckets_used).wrapping_add(1);
    (*table).n_buckets_used;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn hash_insert(
    mut table: *mut Hash_table,
    mut entry: *const libc::c_void,
) -> *mut libc::c_void {
    let mut matched_ent: *const libc::c_void = 0 as *const libc::c_void;
    let mut err: libc::c_int = hash_insert_if_absent(table, entry, &mut matched_ent);
    return if err == -(1 as libc::c_int) {
        0 as *mut libc::c_void
    } else {
        (if err == 0 as libc::c_int { matched_ent } else { entry }) as *mut libc::c_void
    };
}
pub unsafe extern "C" fn hash_remove(
    mut table: *mut Hash_table,
    mut entry: *const libc::c_void,
) -> *mut libc::c_void {
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut bucket: *mut hash_entry = 0 as *mut hash_entry;
    data = hash_find_entry(table, entry, &mut bucket, 1 as libc::c_int != 0);
    if data.is_null() {
        return 0 as *mut libc::c_void;
    }
    (*table).n_entries = ((*table).n_entries).wrapping_sub(1);
    (*table).n_entries;
    if ((*bucket).data).is_null() {
        (*table).n_buckets_used = ((*table).n_buckets_used).wrapping_sub(1);
        (*table).n_buckets_used;
        if ((*table).n_buckets_used as libc::c_float)
            < (*(*table).tuning).shrink_threshold * (*table).n_buckets as libc::c_float
        {
            check_tuning(table);
            if ((*table).n_buckets_used as libc::c_float)
                < (*(*table).tuning).shrink_threshold
                    * (*table).n_buckets as libc::c_float
            {
                let mut tuning: *const Hash_tuning = (*table).tuning;
                let mut candidate: size_t = (if (*tuning).is_n_buckets as libc::c_int
                    != 0
                {
                    (*table).n_buckets as libc::c_float * (*tuning).shrink_factor
                } else {
                    (*table).n_buckets as libc::c_float * (*tuning).shrink_factor
                        * (*tuning).growth_threshold
                }) as size_t;
                if !hash_rehash(table, candidate) {
                    let mut cursor: *mut hash_entry = (*table).free_entry_list;
                    let mut next: *mut hash_entry = 0 as *mut hash_entry;
                    while !cursor.is_null() {
                        next = (*cursor).next;
                        free(cursor as *mut libc::c_void);
                        cursor = next;
                    }
                    (*table).free_entry_list = 0 as *mut hash_entry;
                }
            }
        }
    }
    return data;
}
pub unsafe extern "C" fn hash_delete(
    mut table: *mut Hash_table,
    mut entry: *const libc::c_void,
) -> *mut libc::c_void {
    return hash_remove(table, entry);
}
