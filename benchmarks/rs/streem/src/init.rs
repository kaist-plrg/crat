use ::libc;
extern "C" {
    fn strm_number_init(state: *mut strm_state);
    fn strm_array_init(state: *mut strm_state);
    fn strm_string_init(state: *mut strm_state);
    fn strm_latch_init(state: *mut strm_state);
    fn strm_iter_init(state: *mut strm_state);
    fn strm_socket_init(state: *mut strm_state);
    fn strm_csv_init(state: *mut strm_state);
    fn strm_kvs_init(state: *mut strm_state);
    fn strm_time_init(state: *mut strm_state);
    fn strm_math_init(state: *mut strm_state);
    fn strm_graph_init(state: *mut strm_state);
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_state {
    pub env: *mut libc::c_void,
    pub prev: *mut strm_state,
    pub flags: uint32_t,
}
pub unsafe extern "C" fn strm_init(mut state: *mut strm_state) {
    strm_number_init(state);
    strm_array_init(state);
    strm_string_init(state);
    strm_latch_init(state);
    strm_iter_init(state);
    strm_socket_init(state);
    strm_csv_init(state);
    strm_kvs_init(state);
    strm_time_init(state);
    strm_math_init(state);
    strm_graph_init(state);
}
