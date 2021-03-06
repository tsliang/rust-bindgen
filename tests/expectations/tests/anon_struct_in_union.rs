/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Copy, Clone)]
pub struct s {
    pub u: s__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union s__bindgen_ty_1 {
    pub field: s__bindgen_ty_1_inner,
    _bindgen_union_align: u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct s__bindgen_ty_1_inner {
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_s__bindgen_ty_1_inner() {
    assert_eq!(
        ::std::mem::size_of::<s__bindgen_ty_1_inner>(),
        4usize,
        concat!("Size of: ", stringify!(s__bindgen_ty_1_inner))
    );
    assert_eq!(
        ::std::mem::align_of::<s__bindgen_ty_1_inner>(),
        4usize,
        concat!("Alignment of ", stringify!(s__bindgen_ty_1_inner))
    );
    assert_eq!(
        unsafe { &(*(0 as *const s__bindgen_ty_1_inner)).b as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(s__bindgen_ty_1_inner),
            "::",
            stringify!(b)
        )
    );
}
#[test]
fn bindgen_test_layout_s__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<s__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(s__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<s__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(s__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(0 as *const s__bindgen_ty_1)).field as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(s__bindgen_ty_1),
            "::",
            stringify!(field)
        )
    );
}
impl Default for s__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[test]
fn bindgen_test_layout_s() {
    assert_eq!(
        ::std::mem::size_of::<s>(),
        4usize,
        concat!("Size of: ", stringify!(s))
    );
    assert_eq!(
        ::std::mem::align_of::<s>(),
        4usize,
        concat!("Alignment of ", stringify!(s))
    );
    assert_eq!(
        unsafe { &(*(0 as *const s)).u as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(s), "::", stringify!(u))
    );
}
impl Default for s {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
