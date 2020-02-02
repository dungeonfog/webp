#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

/* automatically generated by rust-bindgen */

pub const _VCRT_COMPILER_PREPROCESSOR: u32 = 1;
pub const _SAL_VERSION: u32 = 20;
pub const __SAL_H_VERSION: u32 = 180000000;
pub const _USE_DECLSPECS_FOR_SAL: u32 = 0;
pub const _USE_ATTRIBUTES_FOR_SAL: u32 = 0;
pub const _CRT_PACKING: u32 = 8;
pub const _HAS_EXCEPTIONS: u32 = 1;
pub const _HAS_CXX17: u32 = 0;
pub const _HAS_CXX20: u32 = 0;
pub const _HAS_NODISCARD: u32 = 0;
pub const _ARGMAX: u32 = 100;
pub const _CRT_INT_MAX: u32 = 2147483647;
pub const _CRT_FUNCTIONS_REQUIRED: u32 = 1;
pub const _CRT_HAS_CXX17: u32 = 0;
pub const _ARM_WINAPI_PARTITION_DESKTOP_SDK_AVAILABLE: u32 = 1;
pub const _CRT_BUILD_DESKTOP_APP: u32 = 1;
pub const _CRT_INTERNAL_NONSTDC_NAMES: u32 = 1;
pub const __STDC_SECURE_LIB__: u32 = 200411;
pub const __GOT_SECURE_LIB__: u32 = 200411;
pub const __STDC_WANT_SECURE_LIB__: u32 = 1;
pub const _SECURECRT_FILL_BUFFER_PATTERN: u32 = 254;
pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES: u32 = 0;
pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES_COUNT: u32 = 0;
pub const _CRT_SECURE_CPP_OVERLOAD_SECURE_NAMES: u32 = 1;
pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES_MEMORY: u32 = 0;
pub const _CRT_SECURE_CPP_OVERLOAD_SECURE_NAMES_MEMORY: u32 = 0;
pub const WEBP_DECODER_ABI_VERSION: u32 = 521;
pub type va_list = *mut ::std::os::raw::c_char;
extern "C" {
    pub fn __va_start(arg1: *mut *mut ::std::os::raw::c_char, ...);
}
pub type __vcrt_bool = bool;
pub type wchar_t = ::std::os::raw::c_ushort;
extern "C" {
    pub fn __security_init_cookie();
}
extern "C" {
    pub fn __security_check_cookie(_StackCookie: usize);
}
extern "C" {
    pub fn __report_gsfailure(_StackCookie: usize);
}
extern "C" {
    pub static mut __security_cookie: usize;
}
pub type __crt_bool = bool;
extern "C" {
    pub fn _invalid_parameter_noinfo();
}
extern "C" {
    pub fn _invalid_parameter_noinfo_noreturn();
}
extern "C" {
    pub fn _invoke_watson(
        _Expression: *const wchar_t,
        _FunctionName: *const wchar_t,
        _FileName: *const wchar_t,
        _LineNo: ::std::os::raw::c_uint,
        _Reserved: usize,
    );
}
pub type errno_t = ::std::os::raw::c_int;
pub type wint_t = ::std::os::raw::c_ushort;
pub type wctype_t = ::std::os::raw::c_ushort;
pub type __time32_t = ::std::os::raw::c_long;
pub type __time64_t = ::std::os::raw::c_longlong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_data_public {
    pub _locale_pctype: *const ::std::os::raw::c_ushort,
    pub _locale_mb_cur_max: ::std::os::raw::c_int,
    pub _locale_lc_codepage: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout___crt_locale_data_public() {
    assert_eq!(
        ::std::mem::size_of::<__crt_locale_data_public>(),
        16usize,
        concat!("Size of: ", stringify!(__crt_locale_data_public))
    );
    assert_eq!(
        ::std::mem::align_of::<__crt_locale_data_public>(),
        8usize,
        concat!("Alignment of ", stringify!(__crt_locale_data_public))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__crt_locale_data_public>()))._locale_pctype as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_data_public),
            "::",
            stringify!(_locale_pctype)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__crt_locale_data_public>()))._locale_mb_cur_max as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_data_public),
            "::",
            stringify!(_locale_mb_cur_max)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__crt_locale_data_public>()))._locale_lc_codepage as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_data_public),
            "::",
            stringify!(_locale_lc_codepage)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_pointers {
    pub locinfo: *mut __crt_locale_data,
    pub mbcinfo: *mut __crt_multibyte_data,
}
#[test]
fn bindgen_test_layout___crt_locale_pointers() {
    assert_eq!(
        ::std::mem::size_of::<__crt_locale_pointers>(),
        16usize,
        concat!("Size of: ", stringify!(__crt_locale_pointers))
    );
    assert_eq!(
        ::std::mem::align_of::<__crt_locale_pointers>(),
        8usize,
        concat!("Alignment of ", stringify!(__crt_locale_pointers))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__crt_locale_pointers>())).locinfo as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_pointers),
            "::",
            stringify!(locinfo)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__crt_locale_pointers>())).mbcinfo as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_pointers),
            "::",
            stringify!(mbcinfo)
        )
    );
}
pub type _locale_t = *mut __crt_locale_pointers;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Mbstatet {
    pub _Wchar: ::std::os::raw::c_ulong,
    pub _Byte: ::std::os::raw::c_ushort,
    pub _State: ::std::os::raw::c_ushort,
}
#[test]
fn bindgen_test_layout__Mbstatet() {
    assert_eq!(
        ::std::mem::size_of::<_Mbstatet>(),
        8usize,
        concat!("Size of: ", stringify!(_Mbstatet))
    );
    assert_eq!(
        ::std::mem::align_of::<_Mbstatet>(),
        4usize,
        concat!("Alignment of ", stringify!(_Mbstatet))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_Mbstatet>()))._Wchar as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_Wchar)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_Mbstatet>()))._Byte as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_Byte)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_Mbstatet>()))._State as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_State)
        )
    );
}
pub type mbstate_t = _Mbstatet;
pub type time_t = __time64_t;
pub type rsize_t = usize;
extern "C" {
    pub fn _errno() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn _set_errno(_Value: ::std::os::raw::c_int) -> errno_t;
}
extern "C" {
    pub fn _get_errno(_Value: *mut ::std::os::raw::c_int) -> errno_t;
}
extern "C" {
    pub fn __threadid() -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn __threadhandle() -> usize;
}
extern "C" {
    pub fn WebPMalloc(size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn WebPFree(ptr: *mut ::std::os::raw::c_void);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WebPIDecoder {
    _unused: [u8; 0],
}
extern "C" {
    pub fn WebPGetDecoderVersion() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPGetInfo(
        data: *const u8,
        data_size: usize,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPDecodeRGBA(
        data: *const u8,
        data_size: usize,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPDecodeARGB(
        data: *const u8,
        data_size: usize,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPDecodeBGRA(
        data: *const u8,
        data_size: usize,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPDecodeRGB(
        data: *const u8,
        data_size: usize,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPDecodeBGR(
        data: *const u8,
        data_size: usize,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPDecodeYUV(
        data: *const u8,
        data_size: usize,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
        u: *mut *mut u8,
        v: *mut *mut u8,
        stride: *mut ::std::os::raw::c_int,
        uv_stride: *mut ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPDecodeRGBAInto(
        data: *const u8,
        data_size: usize,
        output_buffer: *mut u8,
        output_buffer_size: usize,
        output_stride: ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPDecodeARGBInto(
        data: *const u8,
        data_size: usize,
        output_buffer: *mut u8,
        output_buffer_size: usize,
        output_stride: ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPDecodeBGRAInto(
        data: *const u8,
        data_size: usize,
        output_buffer: *mut u8,
        output_buffer_size: usize,
        output_stride: ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPDecodeRGBInto(
        data: *const u8,
        data_size: usize,
        output_buffer: *mut u8,
        output_buffer_size: usize,
        output_stride: ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPDecodeBGRInto(
        data: *const u8,
        data_size: usize,
        output_buffer: *mut u8,
        output_buffer_size: usize,
        output_stride: ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPDecodeYUVInto(
        data: *const u8,
        data_size: usize,
        luma: *mut u8,
        luma_size: usize,
        luma_stride: ::std::os::raw::c_int,
        u: *mut u8,
        u_size: usize,
        u_stride: ::std::os::raw::c_int,
        v: *mut u8,
        v_size: usize,
        v_stride: ::std::os::raw::c_int,
    ) -> *mut u8;
}
pub const WEBP_CSP_MODE_MODE_RGB: WEBP_CSP_MODE = 0;
pub const WEBP_CSP_MODE_MODE_RGBA: WEBP_CSP_MODE = 1;
pub const WEBP_CSP_MODE_MODE_BGR: WEBP_CSP_MODE = 2;
pub const WEBP_CSP_MODE_MODE_BGRA: WEBP_CSP_MODE = 3;
pub const WEBP_CSP_MODE_MODE_ARGB: WEBP_CSP_MODE = 4;
pub const WEBP_CSP_MODE_MODE_RGBA_4444: WEBP_CSP_MODE = 5;
pub const WEBP_CSP_MODE_MODE_RGB_565: WEBP_CSP_MODE = 6;
pub const WEBP_CSP_MODE_MODE_rgbA: WEBP_CSP_MODE = 7;
pub const WEBP_CSP_MODE_MODE_bgrA: WEBP_CSP_MODE = 8;
pub const WEBP_CSP_MODE_MODE_Argb: WEBP_CSP_MODE = 9;
pub const WEBP_CSP_MODE_MODE_rgbA_4444: WEBP_CSP_MODE = 10;
pub const WEBP_CSP_MODE_MODE_YUV: WEBP_CSP_MODE = 11;
pub const WEBP_CSP_MODE_MODE_YUVA: WEBP_CSP_MODE = 12;
pub const WEBP_CSP_MODE_MODE_LAST: WEBP_CSP_MODE = 13;
pub type WEBP_CSP_MODE = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WebPRGBABuffer {
    pub rgba: *mut u8,
    pub stride: ::std::os::raw::c_int,
    pub size: usize,
}
#[test]
fn bindgen_test_layout_WebPRGBABuffer() {
    assert_eq!(
        ::std::mem::size_of::<WebPRGBABuffer>(),
        24usize,
        concat!("Size of: ", stringify!(WebPRGBABuffer))
    );
    assert_eq!(
        ::std::mem::align_of::<WebPRGBABuffer>(),
        8usize,
        concat!("Alignment of ", stringify!(WebPRGBABuffer))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPRGBABuffer>())).rgba as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPRGBABuffer),
            "::",
            stringify!(rgba)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPRGBABuffer>())).stride as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPRGBABuffer),
            "::",
            stringify!(stride)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPRGBABuffer>())).size as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPRGBABuffer),
            "::",
            stringify!(size)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WebPYUVABuffer {
    pub y: *mut u8,
    pub u: *mut u8,
    pub v: *mut u8,
    pub a: *mut u8,
    pub y_stride: ::std::os::raw::c_int,
    pub u_stride: ::std::os::raw::c_int,
    pub v_stride: ::std::os::raw::c_int,
    pub a_stride: ::std::os::raw::c_int,
    pub y_size: usize,
    pub u_size: usize,
    pub v_size: usize,
    pub a_size: usize,
}
#[test]
fn bindgen_test_layout_WebPYUVABuffer() {
    assert_eq!(
        ::std::mem::size_of::<WebPYUVABuffer>(),
        80usize,
        concat!("Size of: ", stringify!(WebPYUVABuffer))
    );
    assert_eq!(
        ::std::mem::align_of::<WebPYUVABuffer>(),
        8usize,
        concat!("Alignment of ", stringify!(WebPYUVABuffer))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPYUVABuffer>())).y as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPYUVABuffer),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPYUVABuffer>())).u as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPYUVABuffer),
            "::",
            stringify!(u)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPYUVABuffer>())).v as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPYUVABuffer),
            "::",
            stringify!(v)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPYUVABuffer>())).a as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPYUVABuffer),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPYUVABuffer>())).y_stride as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPYUVABuffer),
            "::",
            stringify!(y_stride)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPYUVABuffer>())).u_stride as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPYUVABuffer),
            "::",
            stringify!(u_stride)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPYUVABuffer>())).v_stride as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPYUVABuffer),
            "::",
            stringify!(v_stride)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPYUVABuffer>())).a_stride as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPYUVABuffer),
            "::",
            stringify!(a_stride)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPYUVABuffer>())).y_size as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPYUVABuffer),
            "::",
            stringify!(y_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPYUVABuffer>())).u_size as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPYUVABuffer),
            "::",
            stringify!(u_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPYUVABuffer>())).v_size as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPYUVABuffer),
            "::",
            stringify!(v_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPYUVABuffer>())).a_size as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPYUVABuffer),
            "::",
            stringify!(a_size)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WebPDecBuffer {
    pub colorspace: WEBP_CSP_MODE,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub is_external_memory: ::std::os::raw::c_int,
    pub u: WebPDecBuffer__bindgen_ty_1,
    pub pad: [u32; 4usize],
    pub private_memory: *mut u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union WebPDecBuffer__bindgen_ty_1 {
    pub RGBA: WebPRGBABuffer,
    pub YUVA: WebPYUVABuffer,
    _bindgen_union_align: [u64; 10usize],
}
#[test]
fn bindgen_test_layout_WebPDecBuffer__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<WebPDecBuffer__bindgen_ty_1>(),
        80usize,
        concat!("Size of: ", stringify!(WebPDecBuffer__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<WebPDecBuffer__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(WebPDecBuffer__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WebPDecBuffer__bindgen_ty_1>())).RGBA as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecBuffer__bindgen_ty_1),
            "::",
            stringify!(RGBA)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WebPDecBuffer__bindgen_ty_1>())).YUVA as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecBuffer__bindgen_ty_1),
            "::",
            stringify!(YUVA)
        )
    );
}
#[test]
fn bindgen_test_layout_WebPDecBuffer() {
    assert_eq!(
        ::std::mem::size_of::<WebPDecBuffer>(),
        120usize,
        concat!("Size of: ", stringify!(WebPDecBuffer))
    );
    assert_eq!(
        ::std::mem::align_of::<WebPDecBuffer>(),
        8usize,
        concat!("Alignment of ", stringify!(WebPDecBuffer))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPDecBuffer>())).colorspace as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecBuffer),
            "::",
            stringify!(colorspace)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPDecBuffer>())).width as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecBuffer),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPDecBuffer>())).height as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecBuffer),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WebPDecBuffer>())).is_external_memory as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecBuffer),
            "::",
            stringify!(is_external_memory)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPDecBuffer>())).u as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecBuffer),
            "::",
            stringify!(u)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPDecBuffer>())).pad as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecBuffer),
            "::",
            stringify!(pad)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPDecBuffer>())).private_memory as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecBuffer),
            "::",
            stringify!(private_memory)
        )
    );
}
extern "C" {
    pub fn WebPInitDecBufferInternal(
        arg1: *mut WebPDecBuffer,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPFreeDecBuffer(buffer: *mut WebPDecBuffer);
}
pub const VP8StatusCode_VP8_STATUS_OK: VP8StatusCode = 0;
pub const VP8StatusCode_VP8_STATUS_OUT_OF_MEMORY: VP8StatusCode = 1;
pub const VP8StatusCode_VP8_STATUS_INVALID_PARAM: VP8StatusCode = 2;
pub const VP8StatusCode_VP8_STATUS_BITSTREAM_ERROR: VP8StatusCode = 3;
pub const VP8StatusCode_VP8_STATUS_UNSUPPORTED_FEATURE: VP8StatusCode = 4;
pub const VP8StatusCode_VP8_STATUS_SUSPENDED: VP8StatusCode = 5;
pub const VP8StatusCode_VP8_STATUS_USER_ABORT: VP8StatusCode = 6;
pub const VP8StatusCode_VP8_STATUS_NOT_ENOUGH_DATA: VP8StatusCode = 7;
pub type VP8StatusCode = i32;
extern "C" {
    pub fn WebPINewDecoder(output_buffer: *mut WebPDecBuffer) -> *mut WebPIDecoder;
}
extern "C" {
    pub fn WebPINewRGB(
        csp: WEBP_CSP_MODE,
        output_buffer: *mut u8,
        output_buffer_size: usize,
        output_stride: ::std::os::raw::c_int,
    ) -> *mut WebPIDecoder;
}
extern "C" {
    pub fn WebPINewYUVA(
        luma: *mut u8,
        luma_size: usize,
        luma_stride: ::std::os::raw::c_int,
        u: *mut u8,
        u_size: usize,
        u_stride: ::std::os::raw::c_int,
        v: *mut u8,
        v_size: usize,
        v_stride: ::std::os::raw::c_int,
        a: *mut u8,
        a_size: usize,
        a_stride: ::std::os::raw::c_int,
    ) -> *mut WebPIDecoder;
}
extern "C" {
    pub fn WebPINewYUV(
        luma: *mut u8,
        luma_size: usize,
        luma_stride: ::std::os::raw::c_int,
        u: *mut u8,
        u_size: usize,
        u_stride: ::std::os::raw::c_int,
        v: *mut u8,
        v_size: usize,
        v_stride: ::std::os::raw::c_int,
    ) -> *mut WebPIDecoder;
}
extern "C" {
    pub fn WebPIDelete(idec: *mut WebPIDecoder);
}
extern "C" {
    pub fn WebPIAppend(idec: *mut WebPIDecoder, data: *const u8, data_size: usize)
        -> VP8StatusCode;
}
extern "C" {
    pub fn WebPIUpdate(idec: *mut WebPIDecoder, data: *const u8, data_size: usize)
        -> VP8StatusCode;
}
extern "C" {
    pub fn WebPIDecGetRGB(
        idec: *const WebPIDecoder,
        last_y: *mut ::std::os::raw::c_int,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
        stride: *mut ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPIDecGetYUVA(
        idec: *const WebPIDecoder,
        last_y: *mut ::std::os::raw::c_int,
        u: *mut *mut u8,
        v: *mut *mut u8,
        a: *mut *mut u8,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
        stride: *mut ::std::os::raw::c_int,
        uv_stride: *mut ::std::os::raw::c_int,
        a_stride: *mut ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPIDecodedArea(
        idec: *const WebPIDecoder,
        left: *mut ::std::os::raw::c_int,
        top: *mut ::std::os::raw::c_int,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
    ) -> *const WebPDecBuffer;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WebPBitstreamFeatures {
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub has_alpha: ::std::os::raw::c_int,
    pub has_animation: ::std::os::raw::c_int,
    pub format: ::std::os::raw::c_int,
    pub pad: [u32; 5usize],
}
#[test]
fn bindgen_test_layout_WebPBitstreamFeatures() {
    assert_eq!(
        ::std::mem::size_of::<WebPBitstreamFeatures>(),
        40usize,
        concat!("Size of: ", stringify!(WebPBitstreamFeatures))
    );
    assert_eq!(
        ::std::mem::align_of::<WebPBitstreamFeatures>(),
        4usize,
        concat!("Alignment of ", stringify!(WebPBitstreamFeatures))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPBitstreamFeatures>())).width as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPBitstreamFeatures),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPBitstreamFeatures>())).height as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPBitstreamFeatures),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPBitstreamFeatures>())).has_alpha as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPBitstreamFeatures),
            "::",
            stringify!(has_alpha)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WebPBitstreamFeatures>())).has_animation as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPBitstreamFeatures),
            "::",
            stringify!(has_animation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPBitstreamFeatures>())).format as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPBitstreamFeatures),
            "::",
            stringify!(format)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPBitstreamFeatures>())).pad as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPBitstreamFeatures),
            "::",
            stringify!(pad)
        )
    );
}
extern "C" {
    pub fn WebPGetFeaturesInternal(
        arg1: *const u8,
        arg2: usize,
        arg3: *mut WebPBitstreamFeatures,
        arg4: ::std::os::raw::c_int,
    ) -> VP8StatusCode;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WebPDecoderOptions {
    pub bypass_filtering: ::std::os::raw::c_int,
    pub no_fancy_upsampling: ::std::os::raw::c_int,
    pub use_cropping: ::std::os::raw::c_int,
    pub crop_left: ::std::os::raw::c_int,
    pub crop_top: ::std::os::raw::c_int,
    pub crop_width: ::std::os::raw::c_int,
    pub crop_height: ::std::os::raw::c_int,
    pub use_scaling: ::std::os::raw::c_int,
    pub scaled_width: ::std::os::raw::c_int,
    pub scaled_height: ::std::os::raw::c_int,
    pub use_threads: ::std::os::raw::c_int,
    pub dithering_strength: ::std::os::raw::c_int,
    pub flip: ::std::os::raw::c_int,
    pub alpha_dithering_strength: ::std::os::raw::c_int,
    pub pad: [u32; 5usize],
}
#[test]
fn bindgen_test_layout_WebPDecoderOptions() {
    assert_eq!(
        ::std::mem::size_of::<WebPDecoderOptions>(),
        76usize,
        concat!("Size of: ", stringify!(WebPDecoderOptions))
    );
    assert_eq!(
        ::std::mem::align_of::<WebPDecoderOptions>(),
        4usize,
        concat!("Alignment of ", stringify!(WebPDecoderOptions))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WebPDecoderOptions>())).bypass_filtering as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecoderOptions),
            "::",
            stringify!(bypass_filtering)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WebPDecoderOptions>())).no_fancy_upsampling as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecoderOptions),
            "::",
            stringify!(no_fancy_upsampling)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPDecoderOptions>())).use_cropping as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecoderOptions),
            "::",
            stringify!(use_cropping)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPDecoderOptions>())).crop_left as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecoderOptions),
            "::",
            stringify!(crop_left)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPDecoderOptions>())).crop_top as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecoderOptions),
            "::",
            stringify!(crop_top)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPDecoderOptions>())).crop_width as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecoderOptions),
            "::",
            stringify!(crop_width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPDecoderOptions>())).crop_height as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecoderOptions),
            "::",
            stringify!(crop_height)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPDecoderOptions>())).use_scaling as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecoderOptions),
            "::",
            stringify!(use_scaling)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPDecoderOptions>())).scaled_width as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecoderOptions),
            "::",
            stringify!(scaled_width)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WebPDecoderOptions>())).scaled_height as *const _ as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecoderOptions),
            "::",
            stringify!(scaled_height)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPDecoderOptions>())).use_threads as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecoderOptions),
            "::",
            stringify!(use_threads)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WebPDecoderOptions>())).dithering_strength as *const _ as usize
        },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecoderOptions),
            "::",
            stringify!(dithering_strength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPDecoderOptions>())).flip as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecoderOptions),
            "::",
            stringify!(flip)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WebPDecoderOptions>())).alpha_dithering_strength as *const _
                as usize
        },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecoderOptions),
            "::",
            stringify!(alpha_dithering_strength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPDecoderOptions>())).pad as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecoderOptions),
            "::",
            stringify!(pad)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WebPDecoderConfig {
    pub input: WebPBitstreamFeatures,
    pub output: WebPDecBuffer,
    pub options: WebPDecoderOptions,
}
#[test]
fn bindgen_test_layout_WebPDecoderConfig() {
    assert_eq!(
        ::std::mem::size_of::<WebPDecoderConfig>(),
        240usize,
        concat!("Size of: ", stringify!(WebPDecoderConfig))
    );
    assert_eq!(
        ::std::mem::align_of::<WebPDecoderConfig>(),
        8usize,
        concat!("Alignment of ", stringify!(WebPDecoderConfig))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPDecoderConfig>())).input as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecoderConfig),
            "::",
            stringify!(input)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPDecoderConfig>())).output as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecoderConfig),
            "::",
            stringify!(output)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WebPDecoderConfig>())).options as *const _ as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(WebPDecoderConfig),
            "::",
            stringify!(options)
        )
    );
}
extern "C" {
    pub fn WebPInitDecoderConfigInternal(
        arg1: *mut WebPDecoderConfig,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPIDecode(
        data: *const u8,
        data_size: usize,
        config: *mut WebPDecoderConfig,
    ) -> *mut WebPIDecoder;
}
extern "C" {
    pub fn WebPDecode(
        data: *const u8,
        data_size: usize,
        config: *mut WebPDecoderConfig,
    ) -> VP8StatusCode;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_data {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_multibyte_data {
    pub _address: u8,
}
