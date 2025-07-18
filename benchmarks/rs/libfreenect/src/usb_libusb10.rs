use ::libc;
extern "C" {
    pub type libusb_context;
    pub type libusb_device;
    pub type libusb_device_handle;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn libusb_init(ctx: *mut *mut libusb_context) -> libc::c_int;
    fn libusb_exit(ctx: *mut libusb_context);
    fn libusb_error_name(errcode: libc::c_int) -> *const libc::c_char;
    fn libusb_get_device_list(
        ctx: *mut libusb_context,
        list: *mut *mut *mut libusb_device,
    ) -> ssize_t;
    fn libusb_free_device_list(
        list: *mut *mut libusb_device,
        unref_devices: libc::c_int,
    );
    fn libusb_get_device_descriptor(
        dev: *mut libusb_device,
        desc: *mut libusb_device_descriptor,
    ) -> libc::c_int;
    fn libusb_get_active_config_descriptor(
        dev: *mut libusb_device,
        config: *mut *mut libusb_config_descriptor,
    ) -> libc::c_int;
    fn libusb_free_config_descriptor(config: *mut libusb_config_descriptor);
    fn libusb_get_bus_number(dev: *mut libusb_device) -> uint8_t;
    fn libusb_open(
        dev: *mut libusb_device,
        dev_handle: *mut *mut libusb_device_handle,
    ) -> libc::c_int;
    fn libusb_close(dev_handle: *mut libusb_device_handle);
    fn libusb_get_parent(dev: *mut libusb_device) -> *mut libusb_device;
    fn libusb_get_max_iso_packet_size(
        dev: *mut libusb_device,
        endpoint: libc::c_uchar,
    ) -> libc::c_int;
    fn libusb_get_device(dev_handle: *mut libusb_device_handle) -> *mut libusb_device;
    fn libusb_claim_interface(
        dev_handle: *mut libusb_device_handle,
        interface_number: libc::c_int,
    ) -> libc::c_int;
    fn libusb_release_interface(
        dev_handle: *mut libusb_device_handle,
        interface_number: libc::c_int,
    ) -> libc::c_int;
    fn libusb_set_interface_alt_setting(
        dev_handle: *mut libusb_device_handle,
        interface_number: libc::c_int,
        alternate_setting: libc::c_int,
    ) -> libc::c_int;
    fn libusb_reset_device(dev_handle: *mut libusb_device_handle) -> libc::c_int;
    fn libusb_kernel_driver_active(
        dev_handle: *mut libusb_device_handle,
        interface_number: libc::c_int,
    ) -> libc::c_int;
    fn libusb_detach_kernel_driver(
        dev_handle: *mut libusb_device_handle,
        interface_number: libc::c_int,
    ) -> libc::c_int;
    fn libusb_attach_kernel_driver(
        dev_handle: *mut libusb_device_handle,
        interface_number: libc::c_int,
    ) -> libc::c_int;
    fn libusb_alloc_transfer(iso_packets: libc::c_int) -> *mut libusb_transfer;
    fn libusb_submit_transfer(transfer: *mut libusb_transfer) -> libc::c_int;
    fn libusb_cancel_transfer(transfer: *mut libusb_transfer) -> libc::c_int;
    fn libusb_free_transfer(transfer: *mut libusb_transfer);
    fn libusb_control_transfer(
        dev_handle: *mut libusb_device_handle,
        request_type: uint8_t,
        bRequest: uint8_t,
        wValue: uint16_t,
        wIndex: uint16_t,
        data: *mut libc::c_uchar,
        wLength: uint16_t,
        timeout: libc::c_uint,
    ) -> libc::c_int;
    fn libusb_bulk_transfer(
        dev_handle: *mut libusb_device_handle,
        endpoint: libc::c_uchar,
        data: *mut libc::c_uchar,
        length: libc::c_int,
        actual_length: *mut libc::c_int,
        timeout: libc::c_uint,
    ) -> libc::c_int;
    fn libusb_get_string_descriptor_ascii(
        dev_handle: *mut libusb_device_handle,
        desc_index: uint8_t,
        data: *mut libc::c_uchar,
        length: libc::c_int,
    ) -> libc::c_int;
    fn libusb_handle_events_timeout(
        ctx: *mut libusb_context,
        tv: *mut timeval,
    ) -> libc::c_int;
    fn libusb_handle_events(ctx: *mut libusb_context) -> libc::c_int;
    fn fn_log(
        ctx: *mut freenect_context,
        level: freenect_loglevel,
        fmt: *const libc::c_char,
        _: ...
    );
    fn fnusb_set_led_alt(
        dev: *mut libusb_device_handle,
        ctx: *mut freenect_context,
        state: freenect_led_options,
    ) -> libc::c_int;
    fn upload_firmware(
        dev: *mut fnusb_dev,
        fw_filename: *mut libc::c_char,
    ) -> libc::c_int;
    fn upload_firmware_from_memory(
        dev: *mut fnusb_dev,
        fw_from_mem: *mut libc::c_uchar,
        fw_size_in_bytes: libc::c_uint,
    ) -> libc::c_int;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type libusb_transfer_type = libc::c_uint;
pub const LIBUSB_TRANSFER_TYPE_BULK_STREAM: libusb_transfer_type = 4;
pub const LIBUSB_TRANSFER_TYPE_INTERRUPT: libusb_transfer_type = 3;
pub const LIBUSB_TRANSFER_TYPE_BULK: libusb_transfer_type = 2;
pub const LIBUSB_TRANSFER_TYPE_ISOCHRONOUS: libusb_transfer_type = 1;
pub const LIBUSB_TRANSFER_TYPE_CONTROL: libusb_transfer_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_device_descriptor {
    pub bLength: uint8_t,
    pub bDescriptorType: uint8_t,
    pub bcdUSB: uint16_t,
    pub bDeviceClass: uint8_t,
    pub bDeviceSubClass: uint8_t,
    pub bDeviceProtocol: uint8_t,
    pub bMaxPacketSize0: uint8_t,
    pub idVendor: uint16_t,
    pub idProduct: uint16_t,
    pub bcdDevice: uint16_t,
    pub iManufacturer: uint8_t,
    pub iProduct: uint8_t,
    pub iSerialNumber: uint8_t,
    pub bNumConfigurations: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_endpoint_descriptor {
    pub bLength: uint8_t,
    pub bDescriptorType: uint8_t,
    pub bEndpointAddress: uint8_t,
    pub bmAttributes: uint8_t,
    pub wMaxPacketSize: uint16_t,
    pub bInterval: uint8_t,
    pub bRefresh: uint8_t,
    pub bSynchAddress: uint8_t,
    pub extra: *const libc::c_uchar,
    pub extra_length: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_interface_descriptor {
    pub bLength: uint8_t,
    pub bDescriptorType: uint8_t,
    pub bInterfaceNumber: uint8_t,
    pub bAlternateSetting: uint8_t,
    pub bNumEndpoints: uint8_t,
    pub bInterfaceClass: uint8_t,
    pub bInterfaceSubClass: uint8_t,
    pub bInterfaceProtocol: uint8_t,
    pub iInterface: uint8_t,
    pub endpoint: *const libusb_endpoint_descriptor,
    pub extra: *const libc::c_uchar,
    pub extra_length: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_interface {
    pub altsetting: *const libusb_interface_descriptor,
    pub num_altsetting: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_config_descriptor {
    pub bLength: uint8_t,
    pub bDescriptorType: uint8_t,
    pub wTotalLength: uint16_t,
    pub bNumInterfaces: uint8_t,
    pub bConfigurationValue: uint8_t,
    pub iConfiguration: uint8_t,
    pub bmAttributes: uint8_t,
    pub MaxPower: uint8_t,
    pub interface: *const libusb_interface,
    pub extra: *const libc::c_uchar,
    pub extra_length: libc::c_int,
}
pub type libusb_error = libc::c_int;
pub const LIBUSB_ERROR_OTHER: libusb_error = -99;
pub const LIBUSB_ERROR_NOT_SUPPORTED: libusb_error = -12;
pub const LIBUSB_ERROR_NO_MEM: libusb_error = -11;
pub const LIBUSB_ERROR_INTERRUPTED: libusb_error = -10;
pub const LIBUSB_ERROR_PIPE: libusb_error = -9;
pub const LIBUSB_ERROR_OVERFLOW: libusb_error = -8;
pub const LIBUSB_ERROR_TIMEOUT: libusb_error = -7;
pub const LIBUSB_ERROR_BUSY: libusb_error = -6;
pub const LIBUSB_ERROR_NOT_FOUND: libusb_error = -5;
pub const LIBUSB_ERROR_NO_DEVICE: libusb_error = -4;
pub const LIBUSB_ERROR_ACCESS: libusb_error = -3;
pub const LIBUSB_ERROR_INVALID_PARAM: libusb_error = -2;
pub const LIBUSB_ERROR_IO: libusb_error = -1;
pub const LIBUSB_SUCCESS: libusb_error = 0;
pub type libusb_transfer_status = libc::c_uint;
pub const LIBUSB_TRANSFER_OVERFLOW: libusb_transfer_status = 6;
pub const LIBUSB_TRANSFER_NO_DEVICE: libusb_transfer_status = 5;
pub const LIBUSB_TRANSFER_STALL: libusb_transfer_status = 4;
pub const LIBUSB_TRANSFER_CANCELLED: libusb_transfer_status = 3;
pub const LIBUSB_TRANSFER_TIMED_OUT: libusb_transfer_status = 2;
pub const LIBUSB_TRANSFER_ERROR: libusb_transfer_status = 1;
pub const LIBUSB_TRANSFER_COMPLETED: libusb_transfer_status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_iso_packet_descriptor {
    pub length: libc::c_uint,
    pub actual_length: libc::c_uint,
    pub status: libusb_transfer_status,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_transfer {
    pub dev_handle: *mut libusb_device_handle,
    pub flags: uint8_t,
    pub endpoint: libc::c_uchar,
    pub type_0: libc::c_uchar,
    pub timeout: libc::c_uint,
    pub status: libusb_transfer_status,
    pub length: libc::c_int,
    pub actual_length: libc::c_int,
    pub callback: libusb_transfer_cb_fn,
    pub user_data: *mut libc::c_void,
    pub buffer: *mut libc::c_uchar,
    pub num_iso_packets: libc::c_int,
    pub iso_packet_desc: [libusb_iso_packet_descriptor; 0],
}
pub type libusb_transfer_cb_fn = Option::<
    unsafe extern "C" fn(*mut libusb_transfer) -> (),
>;
pub type freenect_device_flags = libc::c_uint;
pub const FREENECT_DEVICE_AUDIO: freenect_device_flags = 4;
pub const FREENECT_DEVICE_CAMERA: freenect_device_flags = 2;
pub const FREENECT_DEVICE_MOTOR: freenect_device_flags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct freenect_device_attributes {
    pub next: *mut freenect_device_attributes,
    pub camera_serial: *const libc::c_char,
}
pub type freenect_resolution = libc::c_uint;
pub const FREENECT_RESOLUTION_DUMMY: freenect_resolution = 2147483647;
pub const FREENECT_RESOLUTION_HIGH: freenect_resolution = 2;
pub const FREENECT_RESOLUTION_MEDIUM: freenect_resolution = 1;
pub const FREENECT_RESOLUTION_LOW: freenect_resolution = 0;
pub type freenect_video_format = libc::c_uint;
pub const FREENECT_VIDEO_DUMMY: freenect_video_format = 2147483647;
pub const FREENECT_VIDEO_YUV_RAW: freenect_video_format = 6;
pub const FREENECT_VIDEO_YUV_RGB: freenect_video_format = 5;
pub const FREENECT_VIDEO_IR_10BIT_PACKED: freenect_video_format = 4;
pub const FREENECT_VIDEO_IR_10BIT: freenect_video_format = 3;
pub const FREENECT_VIDEO_IR_8BIT: freenect_video_format = 2;
pub const FREENECT_VIDEO_BAYER: freenect_video_format = 1;
pub const FREENECT_VIDEO_RGB: freenect_video_format = 0;
pub type freenect_depth_format = libc::c_uint;
pub const FREENECT_DEPTH_DUMMY: freenect_depth_format = 2147483647;
pub const FREENECT_DEPTH_MM: freenect_depth_format = 5;
pub const FREENECT_DEPTH_REGISTERED: freenect_depth_format = 4;
pub const FREENECT_DEPTH_10BIT_PACKED: freenect_depth_format = 3;
pub const FREENECT_DEPTH_11BIT_PACKED: freenect_depth_format = 2;
pub const FREENECT_DEPTH_10BIT: freenect_depth_format = 1;
pub const FREENECT_DEPTH_11BIT: freenect_depth_format = 0;
pub type freenect_led_options = libc::c_uint;
pub const LED_BLINK_RED_YELLOW: freenect_led_options = 6;
pub const LED_BLINK_GREEN: freenect_led_options = 4;
pub const LED_YELLOW: freenect_led_options = 3;
pub const LED_RED: freenect_led_options = 2;
pub const LED_GREEN: freenect_led_options = 1;
pub const LED_OFF: freenect_led_options = 0;
pub type freenect_tilt_status_code = libc::c_uint;
pub const TILT_STATUS_MOVING: freenect_tilt_status_code = 4;
pub const TILT_STATUS_LIMIT: freenect_tilt_status_code = 1;
pub const TILT_STATUS_STOPPED: freenect_tilt_status_code = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct freenect_raw_tilt_state {
    pub accelerometer_x: int16_t,
    pub accelerometer_y: int16_t,
    pub accelerometer_z: int16_t,
    pub tilt_angle: int8_t,
    pub tilt_status: freenect_tilt_status_code,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _freenect_context {
    pub log_level: freenect_loglevel,
    pub log_cb: freenect_log_cb,
    pub usb: fnusb_ctx,
    pub enabled_subdevices: freenect_device_flags,
    pub first: *mut freenect_device,
    pub zero_plane_res: libc::c_int,
    pub fn_fw_nui_ptr: *mut libc::c_uchar,
    pub fn_fw_nui_size: libc::c_uint,
    pub fn_fw_k4w_ptr: *mut libc::c_uchar,
    pub fn_fw_k4w_size: libc::c_uint,
}
pub type freenect_device = _freenect_device;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _freenect_device {
    pub parent: *mut freenect_context,
    pub next: *mut freenect_device,
    pub user_data: *mut libc::c_void,
    pub usb_cam: fnusb_dev,
    pub depth_isoc: fnusb_isoc_stream,
    pub video_isoc: fnusb_isoc_stream,
    pub depth_cb: freenect_depth_cb,
    pub video_cb: freenect_video_cb,
    pub depth_chunk_cb: freenect_chunk_cb,
    pub video_chunk_cb: freenect_chunk_cb,
    pub video_format: freenect_video_format,
    pub depth_format: freenect_depth_format,
    pub video_resolution: freenect_resolution,
    pub depth_resolution: freenect_resolution,
    pub cam_inited: libc::c_int,
    pub cam_tag: uint16_t,
    pub depth: packet_stream,
    pub video: packet_stream,
    pub registration: freenect_registration,
    pub usb_audio: fnusb_dev,
    pub audio_out_isoc: fnusb_isoc_stream,
    pub audio_in_isoc: fnusb_isoc_stream,
    pub audio_in_cb: freenect_audio_in_cb,
    pub audio_out_cb: freenect_audio_out_cb,
    pub audio: audio_stream,
    pub audio_tag: uint32_t,
    pub usb_motor: fnusb_dev,
    pub raw_state: freenect_raw_tilt_state,
    pub device_does_motor_control_with_audio: libc::c_int,
    pub motor_control_with_audio_enabled: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fnusb_dev {
    pub parent: *mut freenect_device,
    pub dev: *mut libusb_device_handle,
    pub device_dead: libc::c_int,
    pub VID: libc::c_int,
    pub PID: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audio_stream {
    pub running: libc::c_int,
    pub audio_out_ring: *mut freenect_sample_51,
    pub ring_reader_idx: libc::c_int,
    pub ring_writer_idx: libc::c_int,
    pub out_window: uint16_t,
    pub out_seq: uint8_t,
    pub out_counter_within_window: uint8_t,
    pub out_weird_timestamp: uint16_t,
    pub out_window_parity: uint8_t,
    pub in_window: uint16_t,
    pub last_seen_window: [uint16_t; 10],
    pub in_counter: uint8_t,
    pub mic_buffer: [*mut int32_t; 4],
    pub cancelled_buffer: *mut int16_t,
    pub in_unknown: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct freenect_sample_51 {
    pub left: int16_t,
    pub right: int16_t,
    pub center: int16_t,
    pub lfe: int16_t,
    pub surround_left: int16_t,
    pub surround_right: int16_t,
}
pub type freenect_audio_out_cb = Option::<
    unsafe extern "C" fn(
        *mut freenect_device,
        *mut freenect_sample_51,
        *mut libc::c_int,
    ) -> (),
>;
pub type freenect_audio_in_cb = Option::<
    unsafe extern "C" fn(
        *mut freenect_device,
        libc::c_int,
        *mut int32_t,
        *mut int32_t,
        *mut int32_t,
        *mut int32_t,
        *mut int16_t,
        *mut libc::c_void,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fnusb_isoc_stream {
    pub parent: *mut fnusb_dev,
    pub xfers: *mut *mut libusb_transfer,
    pub buffer: *mut uint8_t,
    pub cb: fnusb_iso_cb,
    pub num_xfers: libc::c_int,
    pub pkts: libc::c_int,
    pub len: libc::c_int,
    pub dead: libc::c_int,
    pub dead_xfers: libc::c_int,
}
pub type fnusb_iso_cb = Option::<
    unsafe extern "C" fn(*mut freenect_device, *mut uint8_t, libc::c_int) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct freenect_registration {
    pub reg_info: freenect_reg_info,
    pub reg_pad_info: freenect_reg_pad_info,
    pub zero_plane_info: freenect_zero_plane_info,
    pub const_shift: libc::c_double,
    pub raw_to_mm_shift: *mut uint16_t,
    pub depth_to_rgb_shift: *mut int32_t,
    pub registration_table: *mut [int32_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct freenect_zero_plane_info {
    pub dcmos_emitter_dist: libc::c_float,
    pub dcmos_rcmos_dist: libc::c_float,
    pub reference_distance: libc::c_float,
    pub reference_pixel_size: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct freenect_reg_pad_info {
    pub start_lines: uint16_t,
    pub end_lines: uint16_t,
    pub cropping_lines: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct freenect_reg_info {
    pub dx_center: int32_t,
    pub ax: int32_t,
    pub bx: int32_t,
    pub cx: int32_t,
    pub dx: int32_t,
    pub dx_start: int32_t,
    pub ay: int32_t,
    pub by: int32_t,
    pub cy: int32_t,
    pub dy: int32_t,
    pub dy_start: int32_t,
    pub dx_beta_start: int32_t,
    pub dy_beta_start: int32_t,
    pub rollout_blank: int32_t,
    pub rollout_size: int32_t,
    pub dx_beta_inc: int32_t,
    pub dy_beta_inc: int32_t,
    pub dxdx_start: int32_t,
    pub dxdy_start: int32_t,
    pub dydx_start: int32_t,
    pub dydy_start: int32_t,
    pub dxdxdx_start: int32_t,
    pub dydxdx_start: int32_t,
    pub dxdxdy_start: int32_t,
    pub dydxdy_start: int32_t,
    pub back_comp1: int32_t,
    pub dydydx_start: int32_t,
    pub back_comp2: int32_t,
    pub dydydy_start: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct packet_stream {
    pub running: libc::c_int,
    pub flag: uint8_t,
    pub synced: libc::c_int,
    pub seq: uint8_t,
    pub got_pkts: libc::c_int,
    pub pkt_num: libc::c_int,
    pub pkts_per_frame: libc::c_int,
    pub pkt_size: libc::c_int,
    pub frame_size: libc::c_int,
    pub last_pkt_size: libc::c_int,
    pub valid_pkts: libc::c_int,
    pub lost_pkts: libc::c_uint,
    pub valid_frames: libc::c_int,
    pub variable_length: libc::c_int,
    pub last_timestamp: uint32_t,
    pub timestamp: uint32_t,
    pub split_bufs: libc::c_int,
    pub lib_buf: *mut libc::c_void,
    pub usr_buf: *mut libc::c_void,
    pub raw_buf: *mut uint8_t,
    pub proc_buf: *mut libc::c_void,
}
pub type freenect_chunk_cb = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut libc::c_void,
        libc::c_int,
        libc::c_int,
        *mut libc::c_void,
    ) -> (),
>;
pub type freenect_video_cb = Option::<
    unsafe extern "C" fn(*mut freenect_device, *mut libc::c_void, uint32_t) -> (),
>;
pub type freenect_depth_cb = Option::<
    unsafe extern "C" fn(*mut freenect_device, *mut libc::c_void, uint32_t) -> (),
>;
pub type freenect_context = _freenect_context;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fnusb_ctx {
    pub ctx: *mut libusb_context,
    pub should_free_ctx: libc::c_int,
}
pub type freenect_log_cb = Option::<
    unsafe extern "C" fn(
        *mut freenect_context,
        freenect_loglevel,
        *const libc::c_char,
    ) -> (),
>;
pub type freenect_loglevel = libc::c_uint;
pub const FREENECT_LOG_FLOOD: freenect_loglevel = 7;
pub const FREENECT_LOG_SPEW: freenect_loglevel = 6;
pub const FREENECT_LOG_DEBUG: freenect_loglevel = 5;
pub const FREENECT_LOG_INFO: freenect_loglevel = 4;
pub const FREENECT_LOG_NOTICE: freenect_loglevel = 3;
pub const FREENECT_LOG_WARNING: freenect_loglevel = 2;
pub const FREENECT_LOG_ERROR: freenect_loglevel = 1;
pub const FREENECT_LOG_FATAL: freenect_loglevel = 0;
pub type freenect_usb_context = ();
#[inline]
unsafe extern "C" fn libusb_fill_iso_transfer(
    mut transfer: *mut libusb_transfer,
    mut dev_handle: *mut libusb_device_handle,
    mut endpoint: libc::c_uchar,
    mut buffer: *mut libc::c_uchar,
    mut length: libc::c_int,
    mut num_iso_packets: libc::c_int,
    mut callback: libusb_transfer_cb_fn,
    mut user_data: *mut libc::c_void,
    mut timeout: libc::c_uint,
) {
    (*transfer).dev_handle = dev_handle;
    (*transfer).endpoint = endpoint;
    (*transfer)
        .type_0 = LIBUSB_TRANSFER_TYPE_ISOCHRONOUS as libc::c_int as libc::c_uchar;
    (*transfer).timeout = timeout;
    (*transfer).buffer = buffer;
    (*transfer).length = length;
    (*transfer).num_iso_packets = num_iso_packets;
    (*transfer).user_data = user_data;
    (*transfer).callback = callback;
}
#[inline]
unsafe extern "C" fn libusb_set_iso_packet_lengths(
    mut transfer: *mut libusb_transfer,
    mut length: libc::c_uint,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*transfer).num_iso_packets {
        (*((*transfer).iso_packet_desc).as_mut_ptr().offset(i as isize)).length = length;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn fnusb_num_devices(
    mut ctx: *mut freenect_context,
) -> libc::c_int {
    let mut devs: *mut *mut libusb_device = 0 as *mut *mut libusb_device;
    let mut count: ssize_t = libusb_get_device_list((*ctx).usb.ctx, &mut devs);
    if count < 0 as libc::c_int as libc::c_long {
        return if count
            >= (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_long
        {
            count as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
    }
    let mut number_found: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut desc: libusb_device_descriptor = libusb_device_descriptor {
        bLength: 0,
        bDescriptorType: 0,
        bcdUSB: 0,
        bDeviceClass: 0,
        bDeviceSubClass: 0,
        bDeviceProtocol: 0,
        bMaxPacketSize0: 0,
        idVendor: 0,
        idProduct: 0,
        bcdDevice: 0,
        iManufacturer: 0,
        iProduct: 0,
        iSerialNumber: 0,
        bNumConfigurations: 0,
    };
    i = 0 as libc::c_int;
    while (i as libc::c_long) < count {
        let mut r: libc::c_int = libusb_get_device_descriptor(
            *devs.offset(i as isize),
            &mut desc,
        );
        if r < 0 as libc::c_int {
            fn_log(
                ctx,
                FREENECT_LOG_WARNING,
                b"Failed to query USB device descriptor.\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else if desc.idVendor as libc::c_int == 0x45e as libc::c_int {
            if desc.idProduct as libc::c_int == 0x2ae as libc::c_int
                || desc.idProduct as libc::c_int == 0x2bf as libc::c_int
            {
                number_found += 1;
                number_found;
            } else if desc.idProduct as libc::c_int == 0x2d9 as libc::c_int {
                fn_log(
                    ctx,
                    FREENECT_LOG_NOTICE,
                    b"Skipping Kinect v2 device (needs https://github.com/OpenKinect/libfreenect2).\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        }
        i += 1;
        i;
    }
    libusb_free_device_list(devs, 1 as libc::c_int);
    return number_found;
}
pub unsafe extern "C" fn fnusb_is_camera(
    mut desc: libusb_device_descriptor,
) -> libc::c_short {
    return (desc.idVendor as libc::c_int == 0x45e as libc::c_int
        && (desc.idProduct as libc::c_int == 0x2ae as libc::c_int
            || desc.idProduct as libc::c_int == 0x2bf as libc::c_int)) as libc::c_int
        as libc::c_short;
}
pub unsafe extern "C" fn fnusb_is_motor(
    mut desc: libusb_device_descriptor,
) -> libc::c_short {
    return (desc.idVendor as libc::c_int == 0x45e as libc::c_int
        && desc.idProduct as libc::c_int == 0x2b0 as libc::c_int) as libc::c_int
        as libc::c_short;
}
pub unsafe extern "C" fn fnusb_is_audio(
    mut desc: libusb_device_descriptor,
) -> libc::c_short {
    return (desc.idVendor as libc::c_int == 0x45e as libc::c_int
        && (desc.idProduct as libc::c_int == 0x2ad as libc::c_int
            || desc.idProduct as libc::c_int == 0x2be as libc::c_int
            || desc.idProduct as libc::c_int == 0x2c3 as libc::c_int
            || desc.idProduct as libc::c_int == 0x2bb as libc::c_int)) as libc::c_int
        as libc::c_short;
}
pub unsafe extern "C" fn fnusb_find_sibling_device(
    mut ctx: *mut freenect_context,
    mut camera: *mut libusb_device,
    mut deviceList: *mut *mut libusb_device,
    mut count: libc::c_int,
    mut predicate: Option::<
        unsafe extern "C" fn(libusb_device_descriptor) -> libc::c_short,
    >,
) -> *mut libusb_device {
    if count <= 0 as libc::c_int {
        return 0 as *mut libusb_device;
    }
    let cameraBusNo: libc::c_int = libusb_get_bus_number(camera) as libc::c_int;
    if cameraBusNo < 0 as libc::c_int {
        return 0 as *mut libusb_device;
    }
    let mut cameraParent: *const libusb_device = libusb_get_parent(camera);
    let mut siblingCount_Total: libc::c_int = 0 as libc::c_int;
    let mut siblingCount_SameBus: libc::c_int = 0 as libc::c_int;
    let mut siblingMatch_Single: *mut libusb_device = 0 as *mut libusb_device;
    let mut siblingMatch_SameBus: *mut libusb_device = 0 as *mut libusb_device;
    let mut i: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < count {
        let mut currentDevice: *mut libusb_device = *deviceList.offset(i as isize);
        let mut desc: libusb_device_descriptor = libusb_device_descriptor {
            bLength: 0,
            bDescriptorType: 0,
            bcdUSB: 0,
            bDeviceClass: 0,
            bDeviceSubClass: 0,
            bDeviceProtocol: 0,
            bMaxPacketSize0: 0,
            idVendor: 0,
            idProduct: 0,
            bcdDevice: 0,
            iManufacturer: 0,
            iProduct: 0,
            iSerialNumber: 0,
            bNumConfigurations: 0,
        };
        let mut res: libc::c_int = libusb_get_device_descriptor(
            currentDevice,
            &mut desc,
        );
        if !(res < 0 as libc::c_int) {
            if !((Some(predicate.unwrap())).unwrap()(desc) == 0) {
                siblingCount_Total += 1;
                siblingCount_Total;
                siblingMatch_Single = if siblingCount_Total == 1 as libc::c_int {
                    currentDevice
                } else {
                    0 as *mut libusb_device
                };
                if !(cameraBusNo != libusb_get_bus_number(currentDevice) as libc::c_int)
                {
                    siblingCount_SameBus += 1;
                    siblingCount_SameBus;
                    siblingMatch_SameBus = if siblingCount_SameBus == 1 as libc::c_int {
                        currentDevice
                    } else {
                        0 as *mut libusb_device
                    };
                    if !(cameraParent.is_null()
                        || cameraParent
                            != libusb_get_parent(currentDevice) as *const libusb_device)
                    {
                        fn_log(
                            ctx,
                            FREENECT_LOG_DEBUG,
                            b"Found sibling device [same parent]\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        return currentDevice;
                    }
                }
            }
        }
        i += 1;
        i;
    }
    if !siblingMatch_SameBus.is_null() {
        fn_log(
            ctx,
            FREENECT_LOG_DEBUG,
            b"Found sibling device [single on same bus]\n\0" as *const u8
                as *const libc::c_char,
        );
        return siblingMatch_SameBus;
    }
    if !siblingMatch_Single.is_null() {
        fn_log(
            ctx,
            FREENECT_LOG_DEBUG,
            b"Found sibling device [single on system]\n\0" as *const u8
                as *const libc::c_char,
        );
        return siblingMatch_Single;
    }
    return 0 as *mut libusb_device;
}
pub unsafe extern "C" fn fnusb_list_device_attributes(
    mut ctx: *mut freenect_context,
    mut attribute_list: *mut *mut freenect_device_attributes,
) -> libc::c_int {
    *attribute_list = 0 as *mut freenect_device_attributes;
    let mut devs: *mut *mut libusb_device = 0 as *mut *mut libusb_device;
    let mut count: ssize_t = libusb_get_device_list((*ctx).usb.ctx, &mut devs);
    if count < 0 as libc::c_int as libc::c_long {
        return if count
            >= (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_long
        {
            count as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
    }
    let mut next_attr: *mut *mut freenect_device_attributes = attribute_list;
    let mut num_cams: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_long) < count {
        let mut camera_device: *mut libusb_device = *devs.offset(i as isize);
        let mut desc: libusb_device_descriptor = libusb_device_descriptor {
            bLength: 0,
            bDescriptorType: 0,
            bcdUSB: 0,
            bDeviceClass: 0,
            bDeviceSubClass: 0,
            bDeviceProtocol: 0,
            bMaxPacketSize0: 0,
            idVendor: 0,
            idProduct: 0,
            bcdDevice: 0,
            iManufacturer: 0,
            iProduct: 0,
            iSerialNumber: 0,
            bNumConfigurations: 0,
        };
        let mut res: libc::c_int = libusb_get_device_descriptor(
            camera_device,
            &mut desc,
        );
        if !(res < 0 as libc::c_int) {
            if desc.idVendor as libc::c_int == 0x45e as libc::c_int
                && (desc.idProduct as libc::c_int == 0x2ae as libc::c_int
                    || desc.idProduct as libc::c_int == 0x2bf as libc::c_int)
            {
                if !(desc.iSerialNumber as libc::c_int == 0 as libc::c_int) {
                    let mut camera_handle: *mut libusb_device_handle = 0
                        as *mut libusb_device_handle;
                    res = libusb_open(camera_device, &mut camera_handle);
                    if !(res != 0 as libc::c_int) {
                        let mut serial: [libc::c_uchar; 256] = [0; 256];
                        res = libusb_get_string_descriptor_ascii(
                            camera_handle,
                            desc.iSerialNumber,
                            serial.as_mut_ptr(),
                            256 as libc::c_int,
                        );
                        libusb_close(camera_handle);
                        if !(res < 0 as libc::c_int) {
                            let K4W_1473_SERIAL: *const libc::c_char = b"0000000000000000\0"
                                as *const u8 as *const libc::c_char;
                            if strncmp(
                                serial.as_mut_ptr() as *const libc::c_char,
                                K4W_1473_SERIAL,
                                16 as libc::c_int as libc::c_ulong,
                            ) == 0 as libc::c_int
                            {
                                let mut audio_device: *mut libusb_device = fnusb_find_sibling_device(
                                    ctx,
                                    camera_device,
                                    devs,
                                    count as libc::c_int,
                                    Some(
                                        fnusb_is_audio
                                            as unsafe extern "C" fn(
                                                libusb_device_descriptor,
                                            ) -> libc::c_short,
                                    ),
                                );
                                if !audio_device.is_null() {
                                    let mut audio_desc: libusb_device_descriptor = libusb_device_descriptor {
                                        bLength: 0,
                                        bDescriptorType: 0,
                                        bcdUSB: 0,
                                        bDeviceClass: 0,
                                        bDeviceSubClass: 0,
                                        bDeviceProtocol: 0,
                                        bMaxPacketSize0: 0,
                                        idVendor: 0,
                                        idProduct: 0,
                                        bcdDevice: 0,
                                        iManufacturer: 0,
                                        iProduct: 0,
                                        iSerialNumber: 0,
                                        bNumConfigurations: 0,
                                    };
                                    res = libusb_get_device_descriptor(
                                        audio_device,
                                        &mut audio_desc,
                                    );
                                    if res != 0 as libc::c_int {
                                        fn_log(
                                            ctx,
                                            FREENECT_LOG_WARNING,
                                            b"Failed to get audio serial descriptors of K4W or 1473 device: %s\n\0"
                                                as *const u8 as *const libc::c_char,
                                            libusb_error_name(res),
                                        );
                                    } else {
                                        let mut audio_handle: *mut libusb_device_handle = 0
                                            as *mut libusb_device_handle;
                                        res = libusb_open(audio_device, &mut audio_handle);
                                        if res != 0 as libc::c_int {
                                            fn_log(
                                                ctx,
                                                FREENECT_LOG_WARNING,
                                                b"Failed to open audio device for serial of K4W or 1473 device: %s\n\0"
                                                    as *const u8 as *const libc::c_char,
                                                libusb_error_name(res),
                                            );
                                        } else {
                                            res = libusb_get_string_descriptor_ascii(
                                                audio_handle,
                                                audio_desc.iSerialNumber,
                                                serial.as_mut_ptr(),
                                                256 as libc::c_int,
                                            );
                                            libusb_close(audio_handle);
                                            if res != 0 as libc::c_int {
                                                fn_log(
                                                    ctx,
                                                    FREENECT_LOG_WARNING,
                                                    b"Failed to get audio serial of K4W or 1473 device: %s\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                    libusb_error_name(res),
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                            let mut current_attr: *mut freenect_device_attributes = malloc(
                                ::std::mem::size_of::<freenect_device_attributes>()
                                    as libc::c_ulong,
                            ) as *mut freenect_device_attributes;
                            memset(
                                current_attr as *mut libc::c_void,
                                0 as libc::c_int,
                                ::std::mem::size_of::<freenect_device_attributes>()
                                    as libc::c_ulong,
                            );
                            (*current_attr)
                                .camera_serial = strdup(
                                serial.as_mut_ptr() as *mut libc::c_char,
                            );
                            *next_attr = current_attr;
                            next_attr = &mut (*current_attr).next;
                            num_cams += 1;
                            num_cams;
                        }
                    }
                }
            }
        }
        i += 1;
        i;
    }
    libusb_free_device_list(devs, 1 as libc::c_int);
    return num_cams;
}
pub unsafe extern "C" fn fnusb_init(
    mut ctx: *mut fnusb_ctx,
    mut usb_ctx: *mut libc::c_void,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    if usb_ctx.is_null() {
        res = libusb_init(&mut (*ctx).ctx);
        if res >= 0 as libc::c_int {
            (*ctx).should_free_ctx = 1 as libc::c_int;
            return 0 as libc::c_int;
        } else {
            (*ctx).should_free_ctx = 0 as libc::c_int;
            (*ctx).ctx = 0 as *mut libusb_context;
            return res;
        }
    } else {
        (*ctx).ctx = usb_ctx as *mut libusb_context;
        (*ctx).should_free_ctx = 0 as libc::c_int;
        return 0 as libc::c_int;
    };
}
pub unsafe extern "C" fn fnusb_shutdown(mut ctx: *mut fnusb_ctx) -> libc::c_int {
    if (*ctx).should_free_ctx != 0 {
        libusb_exit((*ctx).ctx);
        (*ctx).ctx = 0 as *mut libusb_context;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fnusb_process_events(mut ctx: *mut fnusb_ctx) -> libc::c_int {
    return libusb_handle_events((*ctx).ctx);
}
pub unsafe extern "C" fn fnusb_process_events_timeout(
    mut ctx: *mut fnusb_ctx,
    mut timeout: *mut timeval,
) -> libc::c_int {
    return libusb_handle_events_timeout((*ctx).ctx, timeout);
}
pub unsafe extern "C" fn fnusb_claim_camera(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    let mut ret: libc::c_int = 0 as libc::c_int;
    ret = libusb_kernel_driver_active((*dev).usb_cam.dev, 0 as libc::c_int);
    if ret == 1 as libc::c_int {
        ret = libusb_detach_kernel_driver((*dev).usb_cam.dev, 0 as libc::c_int);
        if ret < 0 as libc::c_int {
            fn_log(
                ctx,
                FREENECT_LOG_ERROR,
                b"Failed to detach camera kernel driver: %s\n\0" as *const u8
                    as *const libc::c_char,
                libusb_error_name(ret),
            );
            libusb_close((*dev).usb_cam.dev);
            (*dev).usb_cam.dev = 0 as *mut libusb_device_handle;
            return ret;
        }
    }
    ret = libusb_claim_interface((*dev).usb_cam.dev, 0 as libc::c_int);
    if ret < 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"Failed to claim camera interface: %s\n\0" as *const u8
                as *const libc::c_char,
            libusb_error_name(ret),
        );
        libusb_close((*dev).usb_cam.dev);
        (*dev).usb_cam.dev = 0 as *mut libusb_device_handle;
        return ret;
    }
    if (*dev).usb_cam.PID == 0x2bf as libc::c_int {
        ret = libusb_set_interface_alt_setting(
            (*dev).usb_cam.dev,
            0 as libc::c_int,
            1 as libc::c_int,
        );
        if ret != 0 as libc::c_int {
            fn_log(
                ctx,
                FREENECT_LOG_ERROR,
                b"Failed to set alternate interface #1 for K4W: %s\n\0" as *const u8
                    as *const libc::c_char,
                libusb_error_name(ret),
            );
            libusb_close((*dev).usb_cam.dev);
            (*dev).usb_cam.dev = 0 as *mut libusb_device_handle;
            return ret;
        }
    }
    return ret;
}
pub unsafe extern "C" fn fnusb_keep_alive_led(
    mut ctx: *mut freenect_context,
    mut audio: *mut libusb_device,
) -> libc::c_int {
    if audio.is_null() {
        return -(1 as libc::c_int);
    }
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut audioHandle: *mut libusb_device_handle = 0 as *mut libusb_device_handle;
    res = libusb_open(audio, &mut audioHandle);
    if res < 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"Failed to set the LED of K4W or 1473 device: %s\n\0" as *const u8
                as *const libc::c_char,
            libusb_error_name(res),
        );
        return res;
    }
    libusb_reset_device(audioHandle);
    libusb_close(audioHandle);
    res = libusb_open(audio, &mut audioHandle);
    if res < 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"Failed to set the LED of K4W or 1473 device: %s\n\0" as *const u8
                as *const libc::c_char,
            libusb_error_name(res),
        );
        return res;
    }
    res = libusb_claim_interface(audioHandle, 0 as libc::c_int);
    if res < 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"Unable to claim interface %s\n\0" as *const u8 as *const libc::c_char,
            libusb_error_name(res),
        );
    } else {
        fnusb_set_led_alt(audioHandle, ctx, LED_GREEN);
        libusb_release_interface(audioHandle, 0 as libc::c_int);
    }
    libusb_close(audioHandle);
    return res;
}
pub unsafe extern "C" fn fnusb_open_subdevices(
    mut dev: *mut freenect_device,
    mut index: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ctx: *mut freenect_context = (*dev).parent;
    (*dev).device_does_motor_control_with_audio = 0 as libc::c_int;
    (*dev).motor_control_with_audio_enabled = 0 as libc::c_int;
    (*dev).usb_cam.parent = dev;
    (*dev).usb_cam.dev = 0 as *mut libusb_device_handle;
    (*dev).usb_motor.parent = dev;
    (*dev).usb_motor.dev = 0 as *mut libusb_device_handle;
    (*dev).usb_audio.parent = dev;
    (*dev).usb_audio.dev = 0 as *mut libusb_device_handle;
    let mut devs: *mut *mut libusb_device = 0 as *mut *mut libusb_device;
    let mut count: ssize_t = libusb_get_device_list((*(*dev).parent).usb.ctx, &mut devs);
    if count < 0 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int);
    }
    let mut camera: *mut libusb_device = 0 as *mut libusb_device;
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut nr_cam: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    loop {
        if !((i as libc::c_long) < count) {
            current_block = 1118134448028020070;
            break;
        }
        let mut desc: libusb_device_descriptor = libusb_device_descriptor {
            bLength: 0,
            bDescriptorType: 0,
            bcdUSB: 0,
            bDeviceClass: 0,
            bDeviceSubClass: 0,
            bDeviceProtocol: 0,
            bMaxPacketSize0: 0,
            idVendor: 0,
            idProduct: 0,
            bcdDevice: 0,
            iManufacturer: 0,
            iProduct: 0,
            iSerialNumber: 0,
            bNumConfigurations: 0,
        };
        res = libusb_get_device_descriptor(*devs.offset(i as isize), &mut desc);
        if !(res < 0 as libc::c_int) {
            if !(fnusb_is_camera(desc) == 0) {
                if nr_cam != index {
                    nr_cam += 1;
                    nr_cam;
                } else {
                    camera = *devs.offset(i as isize);
                    if !((*ctx).enabled_subdevices as libc::c_uint
                        & FREENECT_DEVICE_CAMERA as libc::c_int as libc::c_uint != 0)
                    {
                        current_block = 1118134448028020070;
                        break;
                    }
                    res = libusb_open(camera, &mut (*dev).usb_cam.dev);
                    if res < 0 as libc::c_int || ((*dev).usb_cam.dev).is_null() {
                        fn_log(
                            ctx,
                            FREENECT_LOG_ERROR,
                            b"Could not open camera: %s\n\0" as *const u8
                                as *const libc::c_char,
                            libusb_error_name(res),
                        );
                        (*dev).usb_cam.dev = 0 as *mut libusb_device_handle;
                        current_block = 2610323664595272286;
                        break;
                    } else {
                        if desc.idProduct as libc::c_int == 0x2bf as libc::c_int
                            || desc.bcdDevice as libc::c_int != 267 as libc::c_int
                        {
                            let mut requested_devices: freenect_device_flags = (*ctx)
                                .enabled_subdevices;
                            (*ctx)
                                .enabled_subdevices = ((*ctx).enabled_subdevices
                                as libc::c_uint
                                & !(FREENECT_DEVICE_MOTOR as libc::c_int) as libc::c_uint)
                                as freenect_device_flags;
                            (*ctx).zero_plane_res = 334 as libc::c_int;
                            (*dev)
                                .device_does_motor_control_with_audio = 1 as libc::c_int;
                            let mut audio: *mut libusb_device = fnusb_find_sibling_device(
                                ctx,
                                camera,
                                devs,
                                count as libc::c_int,
                                Some(
                                    fnusb_is_audio
                                        as unsafe extern "C" fn(
                                            libusb_device_descriptor,
                                        ) -> libc::c_short,
                                ),
                            );
                            fnusb_keep_alive_led(ctx, audio);
                            if requested_devices as libc::c_uint
                                & FREENECT_DEVICE_MOTOR as libc::c_int as libc::c_uint != 0
                                && requested_devices as libc::c_uint
                                    & FREENECT_DEVICE_AUDIO as libc::c_int as libc::c_uint
                                    == 0 as libc::c_int as libc::c_uint
                            {
                                (*ctx)
                                    .enabled_subdevices = ((*ctx).enabled_subdevices
                                    as libc::c_uint
                                    | FREENECT_DEVICE_AUDIO as libc::c_int as libc::c_uint)
                                    as freenect_device_flags;
                            }
                        } else {
                            (*ctx).zero_plane_res = 322 as libc::c_int;
                        }
                        (*dev).usb_cam.VID = desc.idVendor as libc::c_int;
                        (*dev).usb_cam.PID = desc.idProduct as libc::c_int;
                        res = fnusb_claim_camera(dev);
                        if res < 0 as libc::c_int {
                            current_block = 2610323664595272286;
                            break;
                        } else {
                            current_block = 1118134448028020070;
                            break;
                        }
                    }
                }
            }
        }
        i += 1;
        i;
    }
    match current_block {
        1118134448028020070 => {
            if res < 0 as libc::c_int || camera.is_null() {
                res = -(1 as libc::c_int);
                current_block = 2610323664595272286;
            } else {
                if (*ctx).enabled_subdevices as libc::c_uint
                    & FREENECT_DEVICE_MOTOR as libc::c_int as libc::c_uint != 0
                {
                    let mut motor: *mut libusb_device = fnusb_find_sibling_device(
                        ctx,
                        camera,
                        devs,
                        count as libc::c_int,
                        Some(
                            fnusb_is_motor
                                as unsafe extern "C" fn(
                                    libusb_device_descriptor,
                                ) -> libc::c_short,
                        ),
                    );
                    if motor.is_null() {
                        fn_log(
                            ctx,
                            FREENECT_LOG_ERROR,
                            b"Could not find device sibling\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        res = -(1 as libc::c_int);
                        current_block = 2610323664595272286;
                    } else {
                        let mut desc_0: libusb_device_descriptor = libusb_device_descriptor {
                            bLength: 0,
                            bDescriptorType: 0,
                            bcdUSB: 0,
                            bDeviceClass: 0,
                            bDeviceSubClass: 0,
                            bDeviceProtocol: 0,
                            bMaxPacketSize0: 0,
                            idVendor: 0,
                            idProduct: 0,
                            bcdDevice: 0,
                            iManufacturer: 0,
                            iProduct: 0,
                            iSerialNumber: 0,
                            bNumConfigurations: 0,
                        };
                        res = libusb_get_device_descriptor(motor, &mut desc_0);
                        if res < 0 as libc::c_int {
                            fn_log(
                                ctx,
                                FREENECT_LOG_ERROR,
                                b"Could not query device: %s\n\0" as *const u8
                                    as *const libc::c_char,
                                libusb_error_name(res),
                            );
                            current_block = 2610323664595272286;
                        } else {
                            res = libusb_open(motor, &mut (*dev).usb_motor.dev);
                            if res < 0 as libc::c_int || ((*dev).usb_motor.dev).is_null()
                            {
                                fn_log(
                                    ctx,
                                    FREENECT_LOG_ERROR,
                                    b"Could not open device: %s\n\0" as *const u8
                                        as *const libc::c_char,
                                    libusb_error_name(res),
                                );
                                (*dev).usb_motor.dev = 0 as *mut libusb_device_handle;
                                current_block = 2610323664595272286;
                            } else {
                                res = libusb_claim_interface(
                                    (*dev).usb_motor.dev,
                                    0 as libc::c_int,
                                );
                                if res < 0 as libc::c_int {
                                    fn_log(
                                        ctx,
                                        FREENECT_LOG_ERROR,
                                        b"Could not claim interface: %s\n\0" as *const u8
                                            as *const libc::c_char,
                                        libusb_error_name(res),
                                    );
                                    libusb_close((*dev).usb_motor.dev);
                                    (*dev).usb_motor.dev = 0 as *mut libusb_device_handle;
                                    current_block = 2610323664595272286;
                                } else {
                                    (*dev).usb_motor.VID = desc_0.idVendor as libc::c_int;
                                    (*dev).usb_motor.PID = desc_0.idProduct as libc::c_int;
                                    current_block = 7990025728955927862;
                                }
                            }
                        }
                    }
                } else {
                    current_block = 7990025728955927862;
                }
                match current_block {
                    2610323664595272286 => {}
                    _ => {
                        if (*ctx).enabled_subdevices as libc::c_uint
                            & FREENECT_DEVICE_AUDIO as libc::c_int as libc::c_uint != 0
                        {
                            let mut audio_0: *mut libusb_device = fnusb_find_sibling_device(
                                ctx,
                                camera,
                                devs,
                                count as libc::c_int,
                                Some(
                                    fnusb_is_audio
                                        as unsafe extern "C" fn(
                                            libusb_device_descriptor,
                                        ) -> libc::c_short,
                                ),
                            );
                            if audio_0.is_null() {
                                fn_log(
                                    ctx,
                                    FREENECT_LOG_ERROR,
                                    b"Could not find device sibling\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                res = -(1 as libc::c_int);
                                current_block = 2610323664595272286;
                            } else {
                                let mut desc_1: libusb_device_descriptor = libusb_device_descriptor {
                                    bLength: 0,
                                    bDescriptorType: 0,
                                    bcdUSB: 0,
                                    bDeviceClass: 0,
                                    bDeviceSubClass: 0,
                                    bDeviceProtocol: 0,
                                    bMaxPacketSize0: 0,
                                    idVendor: 0,
                                    idProduct: 0,
                                    bcdDevice: 0,
                                    iManufacturer: 0,
                                    iProduct: 0,
                                    iSerialNumber: 0,
                                    bNumConfigurations: 0,
                                };
                                res = libusb_get_device_descriptor(audio_0, &mut desc_1);
                                if res < 0 as libc::c_int {
                                    fn_log(
                                        ctx,
                                        FREENECT_LOG_ERROR,
                                        b"Could not query device: %s\n\0" as *const u8
                                            as *const libc::c_char,
                                        libusb_error_name(res),
                                    );
                                    current_block = 2610323664595272286;
                                } else {
                                    res = libusb_open(audio_0, &mut (*dev).usb_audio.dev);
                                    if res < 0 as libc::c_int
                                        || ((*dev).usb_audio.dev).is_null()
                                    {
                                        fn_log(
                                            ctx,
                                            FREENECT_LOG_ERROR,
                                            b"Could not open device: %s\n\0" as *const u8
                                                as *const libc::c_char,
                                            libusb_error_name(res),
                                        );
                                        (*dev).usb_audio.dev = 0 as *mut libusb_device_handle;
                                        current_block = 2610323664595272286;
                                    } else {
                                        res = libusb_claim_interface(
                                            (*dev).usb_audio.dev,
                                            0 as libc::c_int,
                                        );
                                        if res < 0 as libc::c_int {
                                            fn_log(
                                                ctx,
                                                FREENECT_LOG_ERROR,
                                                b"Could not claim interface: %s\n\0" as *const u8
                                                    as *const libc::c_char,
                                                libusb_error_name(res),
                                            );
                                            libusb_close((*dev).usb_audio.dev);
                                            (*dev).usb_audio.dev = 0 as *mut libusb_device_handle;
                                            current_block = 2610323664595272286;
                                        } else {
                                            (*dev).usb_audio.VID = desc_1.idVendor as libc::c_int;
                                            (*dev).usb_audio.PID = desc_1.idProduct as libc::c_int;
                                            let mut num_interfaces: libc::c_int = fnusb_num_interfaces(
                                                &mut (*dev).usb_audio,
                                            );
                                            if num_interfaces >= 2 as libc::c_int {
                                                if (*dev).device_does_motor_control_with_audio != 0 {
                                                    (*dev).motor_control_with_audio_enabled = 1 as libc::c_int;
                                                }
                                                current_block = 16314074004867283505;
                                            } else {
                                                let mut string_desc: [libc::c_uchar; 256] = [0; 256];
                                                res = libusb_get_string_descriptor_ascii(
                                                    (*dev).usb_audio.dev,
                                                    desc_1.iSerialNumber,
                                                    string_desc.as_mut_ptr(),
                                                    256 as libc::c_int,
                                                );
                                                if res < 0 as libc::c_int {
                                                    fn_log(
                                                        ctx,
                                                        FREENECT_LOG_ERROR,
                                                        b"Failed to retrieve serial number for audio device in bootloader state\n\0"
                                                            as *const u8 as *const libc::c_char,
                                                    );
                                                    current_block = 2610323664595272286;
                                                } else {
                                                    let mut audio_serial: *mut libc::c_char = strdup(
                                                        string_desc.as_mut_ptr() as *mut libc::c_char,
                                                    );
                                                    fn_log(
                                                        ctx,
                                                        FREENECT_LOG_SPEW,
                                                        b"Uploading firmware to audio device in bootloader state.\n\0"
                                                            as *const u8 as *const libc::c_char,
                                                    );
                                                    if desc_1.idProduct as libc::c_int == 0x2ad as libc::c_int
                                                        && !((*ctx).fn_fw_nui_ptr).is_null()
                                                        && (*ctx).fn_fw_nui_size > 0 as libc::c_int as libc::c_uint
                                                    {
                                                        fn_log(
                                                            ctx,
                                                            FREENECT_LOG_SPEW,
                                                            b"loading firmware from memory\n\0" as *const u8
                                                                as *const libc::c_char,
                                                        );
                                                        res = upload_firmware_from_memory(
                                                            &mut (*dev).usb_audio,
                                                            (*ctx).fn_fw_nui_ptr,
                                                            (*ctx).fn_fw_nui_size,
                                                        );
                                                    } else if desc_1.idProduct as libc::c_int
                                                        == 0x2be as libc::c_int && !((*ctx).fn_fw_k4w_ptr).is_null()
                                                        && (*ctx).fn_fw_k4w_size > 0 as libc::c_int as libc::c_uint
                                                    {
                                                        fn_log(
                                                            ctx,
                                                            FREENECT_LOG_SPEW,
                                                            b"loading firmware from memory\n\0" as *const u8
                                                                as *const libc::c_char,
                                                        );
                                                        res = upload_firmware_from_memory(
                                                            &mut (*dev).usb_audio,
                                                            (*ctx).fn_fw_k4w_ptr,
                                                            (*ctx).fn_fw_k4w_size,
                                                        );
                                                    } else {
                                                        res = upload_firmware(
                                                            &mut (*dev).usb_audio,
                                                            b"audios.bin\0" as *const u8 as *const libc::c_char
                                                                as *mut libc::c_char,
                                                        );
                                                    }
                                                    if res < 0 as libc::c_int {
                                                        fn_log(
                                                            ctx,
                                                            FREENECT_LOG_ERROR,
                                                            b"upload_firmware failed: %d\n\0" as *const u8
                                                                as *const libc::c_char,
                                                            res,
                                                        );
                                                        free(audio_serial as *mut libc::c_void);
                                                        current_block = 2610323664595272286;
                                                    } else {
                                                        libusb_close((*dev).usb_audio.dev);
                                                        (*dev).usb_audio.dev = 0 as *mut libusb_device_handle;
                                                        let mut loops: libc::c_int = 0 as libc::c_int;
                                                        loops = 0 as libc::c_int;
                                                        while loops < 10 as libc::c_int {
                                                            fn_log(
                                                                ctx,
                                                                FREENECT_LOG_SPEW,
                                                                b"Try %d: Looking for new audio device matching serial %s\n\0"
                                                                    as *const u8 as *const libc::c_char,
                                                                loops,
                                                                audio_serial,
                                                            );
                                                            let mut new_dev_list: *mut *mut libusb_device = 0
                                                                as *mut *mut libusb_device;
                                                            let mut dev_index: libc::c_int = 0;
                                                            let mut num_new_devs: ssize_t = libusb_get_device_list(
                                                                (*ctx).usb.ctx,
                                                                &mut new_dev_list,
                                                            );
                                                            dev_index = 0 as libc::c_int;
                                                            while (dev_index as libc::c_long) < num_new_devs {
                                                                let mut new_dev_desc: libusb_device_descriptor = libusb_device_descriptor {
                                                                    bLength: 0,
                                                                    bDescriptorType: 0,
                                                                    bcdUSB: 0,
                                                                    bDeviceClass: 0,
                                                                    bDeviceSubClass: 0,
                                                                    bDeviceProtocol: 0,
                                                                    bMaxPacketSize0: 0,
                                                                    idVendor: 0,
                                                                    idProduct: 0,
                                                                    bcdDevice: 0,
                                                                    iManufacturer: 0,
                                                                    iProduct: 0,
                                                                    iSerialNumber: 0,
                                                                    bNumConfigurations: 0,
                                                                };
                                                                let mut r: libc::c_int = 0;
                                                                r = libusb_get_device_descriptor(
                                                                    *new_dev_list.offset(dev_index as isize),
                                                                    &mut new_dev_desc,
                                                                );
                                                                if !(r < 0 as libc::c_int) {
                                                                    if fnusb_is_audio(new_dev_desc) != 0 {
                                                                        fn_log(
                                                                            ctx,
                                                                            FREENECT_LOG_SPEW,
                                                                            b"Matched VID/PID!\n\0" as *const u8 as *const libc::c_char,
                                                                        );
                                                                        let mut new_dev_handle: *mut libusb_device_handle = 0
                                                                            as *mut libusb_device_handle;
                                                                        r = libusb_open(
                                                                            *new_dev_list.offset(dev_index as isize),
                                                                            &mut new_dev_handle,
                                                                        );
                                                                        if !(r < 0 as libc::c_int) {
                                                                            r = libusb_get_string_descriptor_ascii(
                                                                                new_dev_handle,
                                                                                new_dev_desc.iSerialNumber,
                                                                                string_desc.as_mut_ptr(),
                                                                                256 as libc::c_int,
                                                                            );
                                                                            if r < 0 as libc::c_int {
                                                                                fn_log(
                                                                                    ctx,
                                                                                    FREENECT_LOG_SPEW,
                                                                                    b"Lost new audio device while fetching serial number.\n\0"
                                                                                        as *const u8 as *const libc::c_char,
                                                                                );
                                                                                libusb_close(new_dev_handle);
                                                                            } else if r as libc::c_ulong == strlen(audio_serial)
                                                                                && strcmp(
                                                                                    string_desc.as_mut_ptr() as *mut libc::c_char,
                                                                                    audio_serial,
                                                                                ) == 0 as libc::c_int
                                                                            {
                                                                                r = libusb_claim_interface(
                                                                                    new_dev_handle,
                                                                                    0 as libc::c_int,
                                                                                );
                                                                                if r != 0 as libc::c_int {
                                                                                    fn_log(
                                                                                        ctx,
                                                                                        FREENECT_LOG_SPEW,
                                                                                        b"Device with serial %s reappeared but couldn't claim interface 0\n\0"
                                                                                            as *const u8 as *const libc::c_char,
                                                                                        audio_serial,
                                                                                    );
                                                                                    libusb_close(new_dev_handle);
                                                                                } else {
                                                                                    (*dev).usb_audio.dev = new_dev_handle;
                                                                                    num_interfaces = fnusb_num_interfaces(
                                                                                        &mut (*dev).usb_audio,
                                                                                    );
                                                                                    if num_interfaces >= 2 as libc::c_int {
                                                                                        if (*dev).device_does_motor_control_with_audio != 0 {
                                                                                            (*dev).motor_control_with_audio_enabled = 1 as libc::c_int;
                                                                                        }
                                                                                        break;
                                                                                    } else {
                                                                                        fn_log(
                                                                                            ctx,
                                                                                            FREENECT_LOG_SPEW,
                                                                                            b"Opened audio with matching serial but too few interfaces.\n\0"
                                                                                                as *const u8 as *const libc::c_char,
                                                                                        );
                                                                                        (*dev).usb_audio.dev = 0 as *mut libusb_device_handle;
                                                                                        libusb_close(new_dev_handle);
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                fn_log(
                                                                                    ctx,
                                                                                    FREENECT_LOG_SPEW,
                                                                                    b"Got serial %s, expected serial %s\n\0" as *const u8
                                                                                        as *const libc::c_char,
                                                                                    string_desc.as_mut_ptr() as *mut libc::c_char,
                                                                                    audio_serial,
                                                                                );
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                dev_index += 1;
                                                                dev_index;
                                                            }
                                                            libusb_free_device_list(new_dev_list, 1 as libc::c_int);
                                                            if !((*dev).usb_audio.dev).is_null() {
                                                                break;
                                                            }
                                                            sleep(1 as libc::c_int as libc::c_uint);
                                                            loops += 1;
                                                            loops;
                                                        }
                                                        free(audio_serial as *mut libc::c_void);
                                                        current_block = 16314074004867283505;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            current_block = 16314074004867283505;
                        }
                        match current_block {
                            2610323664595272286 => {}
                            _ => {
                                if (!((*dev).usb_cam.dev).is_null()
                                    || (*ctx).enabled_subdevices as libc::c_uint
                                        & FREENECT_DEVICE_CAMERA as libc::c_int as libc::c_uint
                                        == 0)
                                    && (!((*dev).usb_motor.dev).is_null()
                                        || (*ctx).enabled_subdevices as libc::c_uint
                                            & FREENECT_DEVICE_MOTOR as libc::c_int as libc::c_uint == 0)
                                    && (!((*dev).usb_audio.dev).is_null()
                                        || (*ctx).enabled_subdevices as libc::c_uint
                                            & FREENECT_DEVICE_AUDIO as libc::c_int as libc::c_uint == 0)
                                {
                                    current_block = 12691247646256966570;
                                } else {
                                    current_block = 2610323664595272286;
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        2610323664595272286 => {
            fnusb_close_subdevices(dev);
        }
        _ => {}
    }
    libusb_free_device_list(devs, 1 as libc::c_int);
    return res;
}
pub unsafe extern "C" fn fnusb_close_subdevices(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    if !((*dev).usb_cam.dev).is_null() {
        libusb_release_interface((*dev).usb_cam.dev, 0 as libc::c_int);
        libusb_attach_kernel_driver((*dev).usb_cam.dev, 0 as libc::c_int);
        libusb_close((*dev).usb_cam.dev);
        (*dev).usb_cam.dev = 0 as *mut libusb_device_handle;
    }
    if !((*dev).usb_motor.dev).is_null() {
        libusb_release_interface((*dev).usb_motor.dev, 0 as libc::c_int);
        libusb_close((*dev).usb_motor.dev);
        (*dev).usb_motor.dev = 0 as *mut libusb_device_handle;
    }
    if !((*dev).usb_audio.dev).is_null() {
        libusb_release_interface((*dev).usb_audio.dev, 0 as libc::c_int);
        libusb_close((*dev).usb_audio.dev);
        (*dev).usb_audio.dev = 0 as *mut libusb_device_handle;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn iso_callback(mut xfer: *mut libusb_transfer) {
    let mut i: libc::c_int = 0;
    let mut strm: *mut fnusb_isoc_stream = (*xfer).user_data as *mut fnusb_isoc_stream;
    let mut ctx: *mut freenect_context = (*(*(*strm).parent).parent).parent;
    if (*strm).dead != 0 {
        (*strm).dead_xfers += 1;
        (*strm).dead_xfers;
        fn_log(
            ctx,
            FREENECT_LOG_SPEW,
            b"EP %02x transfer complete, %d left\n\0" as *const u8
                as *const libc::c_char,
            (*xfer).endpoint as libc::c_int,
            (*strm).num_xfers - (*strm).dead_xfers,
        );
        return;
    }
    match (*xfer).status as libc::c_uint {
        0 => {
            let mut buf: *mut uint8_t = (*xfer).buffer as *mut uint8_t;
            i = 0 as libc::c_int;
            while i < (*strm).pkts {
                ((*strm).cb)
                    .unwrap()(
                    (*(*strm).parent).parent,
                    buf,
                    (*((*xfer).iso_packet_desc).as_mut_ptr().offset(i as isize))
                        .actual_length as libc::c_int,
                );
                buf = buf.offset((*strm).len as isize);
                i += 1;
                i;
            }
            let mut res: libc::c_int = 0;
            res = libusb_submit_transfer(xfer);
            if res != 0 as libc::c_int {
                fn_log(
                    ctx,
                    FREENECT_LOG_ERROR,
                    b"iso_callback(): failed to resubmit transfer after successful completion: %s\n\0"
                        as *const u8 as *const libc::c_char,
                    libusb_error_name(res),
                );
                (*strm).dead_xfers += 1;
                (*strm).dead_xfers;
                if res == LIBUSB_ERROR_NO_DEVICE as libc::c_int {
                    (*(*strm).parent).device_dead = 1 as libc::c_int;
                }
            }
        }
        5 => {
            if (*(*strm).parent).device_dead == 0 {
                fn_log(
                    ctx,
                    FREENECT_LOG_ERROR,
                    b"USB device disappeared, cancelling stream %02x :(\n\0" as *const u8
                        as *const libc::c_char,
                    (*xfer).endpoint as libc::c_int,
                );
            }
            (*strm).dead_xfers += 1;
            (*strm).dead_xfers;
            (*(*strm).parent).device_dead = 1 as libc::c_int;
        }
        3 => {
            if (*strm).dead != 0 {
                fn_log(
                    ctx,
                    FREENECT_LOG_SPEW,
                    b"EP %02x transfer cancelled\n\0" as *const u8
                        as *const libc::c_char,
                    (*xfer).endpoint as libc::c_int,
                );
            } else {
                if (*(*strm).parent).device_dead == 0 {
                    fn_log(
                        ctx,
                        FREENECT_LOG_ERROR,
                        b"Got cancelled transfer, but we didn't request it - device disconnected?\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                (*(*strm).parent).device_dead = 1 as libc::c_int;
            }
            (*strm).dead_xfers += 1;
            (*strm).dead_xfers;
        }
        _ => {
            fn_log(
                ctx,
                FREENECT_LOG_WARNING,
                b"Isochronous transfer error: %d\n\0" as *const u8
                    as *const libc::c_char,
                (*xfer).status as libc::c_uint,
            );
            let mut res_0: libc::c_int = 0;
            res_0 = libusb_submit_transfer(xfer);
            if res_0 != 0 as libc::c_int {
                fn_log(
                    ctx,
                    FREENECT_LOG_ERROR,
                    b"Isochronous transfer resubmission failed after unknown error: %s\n\0"
                        as *const u8 as *const libc::c_char,
                    libusb_error_name(res_0),
                );
                (*strm).dead_xfers += 1;
                (*strm).dead_xfers;
                if res_0 == LIBUSB_ERROR_NO_DEVICE as libc::c_int {
                    (*(*strm).parent).device_dead = 1 as libc::c_int;
                }
            }
        }
    };
}
pub unsafe extern "C" fn fnusb_get_max_iso_packet_size(
    mut dev: *mut fnusb_dev,
    mut endpoint: libc::c_uchar,
    mut default_size: libc::c_int,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*(*dev).parent).parent;
    let mut size: libc::c_int = libusb_get_max_iso_packet_size(
        libusb_get_device((*dev).dev),
        endpoint,
    );
    if size <= 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_WARNING,
            b"libusb_get_max_iso_packet_size() returned %d; using default %d\n\0"
                as *const u8 as *const libc::c_char,
            size,
            default_size,
        );
        size = default_size;
    }
    return size;
}
pub unsafe extern "C" fn fnusb_start_iso(
    mut dev: *mut fnusb_dev,
    mut strm: *mut fnusb_isoc_stream,
    mut cb: fnusb_iso_cb,
    mut endpoint: libc::c_uchar,
    mut xfers: libc::c_int,
    mut pkts: libc::c_int,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*(*dev).parent).parent;
    (*strm).parent = dev;
    (*strm).cb = cb;
    (*strm).num_xfers = xfers;
    (*strm).pkts = pkts;
    (*strm).len = len;
    (*strm).buffer = malloc((xfers * pkts * len) as libc::c_ulong) as *mut uint8_t;
    (*strm)
        .xfers = malloc(
        (::std::mem::size_of::<*mut libusb_transfer>() as libc::c_ulong)
            .wrapping_mul(xfers as libc::c_ulong),
    ) as *mut *mut libusb_transfer;
    (*strm).dead = 0 as libc::c_int;
    (*strm).dead_xfers = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut bufp: *mut uint8_t = (*strm).buffer;
    i = 0 as libc::c_int;
    while i < xfers {
        fn_log(
            ctx,
            FREENECT_LOG_SPEW,
            b"Creating endpoint %02x transfer #%d\n\0" as *const u8
                as *const libc::c_char,
            endpoint as libc::c_int,
            i,
        );
        let ref mut fresh0 = *((*strm).xfers).offset(i as isize);
        *fresh0 = libusb_alloc_transfer(pkts);
        if (*((*strm).xfers).offset(i as isize)).is_null() {
            fn_log(
                ctx,
                FREENECT_LOG_WARNING,
                b"Failed to allocate transfer\n\0" as *const u8 as *const libc::c_char,
            );
            (*strm).dead_xfers += 1;
            (*strm).dead_xfers;
        } else {
            libusb_fill_iso_transfer(
                *((*strm).xfers).offset(i as isize),
                (*dev).dev,
                endpoint,
                bufp,
                pkts * len,
                pkts,
                Some(iso_callback as unsafe extern "C" fn(*mut libusb_transfer) -> ()),
                strm as *mut libc::c_void,
                0 as libc::c_int as libc::c_uint,
            );
            libusb_set_iso_packet_lengths(
                *((*strm).xfers).offset(i as isize),
                len as libc::c_uint,
            );
            let mut ret: libc::c_int = libusb_submit_transfer(
                *((*strm).xfers).offset(i as isize),
            );
            if ret < 0 as libc::c_int {
                fn_log(
                    ctx,
                    FREENECT_LOG_WARNING,
                    b"Failed to submit isochronous transfer %d: %s\n\0" as *const u8
                        as *const libc::c_char,
                    i,
                    libusb_error_name(ret),
                );
                (*strm).dead_xfers += 1;
                (*strm).dead_xfers;
            }
        }
        bufp = bufp.offset((pkts * len) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fnusb_stop_iso(
    mut dev: *mut fnusb_dev,
    mut strm: *mut fnusb_isoc_stream,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*(*dev).parent).parent;
    let mut i: libc::c_int = 0;
    fn_log(
        ctx,
        FREENECT_LOG_FLOOD,
        b"fnusb_stop_iso() called\n\0" as *const u8 as *const libc::c_char,
    );
    (*strm).dead = 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*strm).num_xfers {
        libusb_cancel_transfer(*((*strm).xfers).offset(i as isize));
        i += 1;
        i;
    }
    fn_log(
        ctx,
        FREENECT_LOG_FLOOD,
        b"fnusb_stop_iso() cancelled all transfers\n\0" as *const u8
            as *const libc::c_char,
    );
    while (*strm).dead_xfers < (*strm).num_xfers {
        fn_log(
            ctx,
            FREENECT_LOG_FLOOD,
            b"fnusb_stop_iso() dead = %d\tnum = %d\n\0" as *const u8
                as *const libc::c_char,
            (*strm).dead_xfers,
            (*strm).num_xfers,
        );
        libusb_handle_events((*ctx).usb.ctx);
    }
    i = 0 as libc::c_int;
    while i < (*strm).num_xfers {
        libusb_free_transfer(*((*strm).xfers).offset(i as isize));
        i += 1;
        i;
    }
    fn_log(
        ctx,
        FREENECT_LOG_FLOOD,
        b"fnusb_stop_iso() freed all transfers\n\0" as *const u8 as *const libc::c_char,
    );
    free((*strm).buffer as *mut libc::c_void);
    free((*strm).xfers as *mut libc::c_void);
    fn_log(
        ctx,
        FREENECT_LOG_FLOOD,
        b"fnusb_stop_iso() freed buffers and stream\n\0" as *const u8
            as *const libc::c_char,
    );
    fn_log(
        ctx,
        FREENECT_LOG_FLOOD,
        b"fnusb_stop_iso() done\n\0" as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fnusb_control(
    mut dev: *mut fnusb_dev,
    mut bmRequestType: uint8_t,
    mut bRequest: uint8_t,
    mut wValue: uint16_t,
    mut wIndex: uint16_t,
    mut data: *mut uint8_t,
    mut wLength: uint16_t,
) -> libc::c_int {
    return libusb_control_transfer(
        (*dev).dev,
        bmRequestType,
        bRequest,
        wValue,
        wIndex,
        data,
        wLength,
        0 as libc::c_int as libc::c_uint,
    );
}
pub unsafe extern "C" fn fnusb_bulk(
    mut dev: *mut fnusb_dev,
    mut endpoint: uint8_t,
    mut data: *mut uint8_t,
    mut len: libc::c_int,
    mut transferred: *mut libc::c_int,
) -> libc::c_int {
    *transferred = 0 as libc::c_int;
    return libusb_bulk_transfer(
        (*dev).dev,
        endpoint,
        data,
        len,
        transferred,
        0 as libc::c_int as libc::c_uint,
    );
}
pub unsafe extern "C" fn fnusb_num_interfaces(mut dev: *mut fnusb_dev) -> libc::c_int {
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut res: libc::c_int = 0;
    let mut d: *mut libusb_device = libusb_get_device((*dev).dev);
    let mut config: *mut libusb_config_descriptor = 0 as *mut libusb_config_descriptor;
    res = libusb_get_active_config_descriptor(d, &mut config);
    if res < 0 as libc::c_int {
        return res;
    }
    retval = (*config).bNumInterfaces as libc::c_int;
    libusb_free_config_descriptor(config);
    return retval;
}
