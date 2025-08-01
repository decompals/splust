// Generated by `wit-bindgen` 0.36.0. DO NOT EDIT!
// Options used:
//   * runtime_path: "wit_bindgen_rt"
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_testfunc_cabi<T: Guest>(arg0: *mut u8, arg1: usize) -> *mut u8 {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let len0 = arg1;
    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
    let result1 = T::testfunc(_rt::string_lift(bytes0));
    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    let vec3 = (result1.into_bytes()).into_boxed_slice();
    let ptr3 = vec3.as_ptr().cast::<u8>();
    let len3 = vec3.len();
    ::core::mem::forget(vec3);
    *ptr2.add(4).cast::<usize>() = len3;
    *ptr2.add(0).cast::<*mut u8>() = ptr3.cast_mut();
    ptr2
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_testfunc<T: Guest>(arg0: *mut u8) {
    let l0 = *arg0.add(0).cast::<*mut u8>();
    let l1 = *arg0.add(4).cast::<usize>();
    _rt::cabi_dealloc(l0, l1, 1);
}
pub trait Guest {
    fn testfunc(name: _rt::String) -> _rt::String;
}
#[doc(hidden)]
macro_rules! __export_world_segment_cabi {
    ($ty:ident with_types_in $($path_to_types:tt)*) => {
        const _ : () = { #[export_name = "testfunc"] unsafe extern "C" fn
        export_testfunc(arg0 : * mut u8, arg1 : usize,) -> * mut u8 {
        $($path_to_types)*:: _export_testfunc_cabi::<$ty > (arg0, arg1) } #[export_name =
        "cabi_post_testfunc"] unsafe extern "C" fn _post_return_testfunc(arg0 : * mut
        u8,) { $($path_to_types)*:: __post_return_testfunc::<$ty > (arg0) } };
    };
}
#[doc(hidden)]
pub(crate) use __export_world_segment_cabi;
#[repr(align(4))]
struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 8]);
#[rustfmt::skip]
mod _rt {
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub use alloc_crate::string::String;
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_segment_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*:: __export_world_segment_cabi!($ty with_types_in
        $($path_to_types_root)*);
    };
}
#[doc(inline)]
pub(crate) use __export_segment_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.36.0:splat:segment:segment:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 181] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x078\x01A\x02\x01A\x02\x01\
@\x01\x04names\0s\x04\0\x08testfunc\x01\0\x04\0\x15splat:segment/segment\x04\0\x0b\
\x0d\x01\0\x07segment\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-com\
ponent\x070.220.0\x10wit-bindgen-rust\x060.36.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
