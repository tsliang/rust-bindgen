/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Copy, Clone)]
pub union U4 {
    pub _bitfield_1: u8,
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout_U4() {
    assert_eq!(
        ::std::mem::size_of::<U4>(),
        4usize,
        concat!("Size of: ", stringify!(U4))
    );
    assert_eq!(
        ::std::mem::align_of::<U4>(),
        4usize,
        concat!("Alignment of ", stringify!(U4))
    );
}
impl Default for U4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl U4 {
    #[inline]
    pub fn derp(&self) -> ::std::os::raw::c_uint {
        let mut unit_field_val: u8 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &self._bitfield_1 as *const _ as *const u8,
                &mut unit_field_val as *mut u8 as *mut u8,
                1usize,
            )
        };
        let mask = 0x1 as u8;
        let val = (unit_field_val & mask) >> 0usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_derp(&mut self, val: ::std::os::raw::c_uint) {
        let mask = 0x1 as u8;
        let val = val as u32 as u8;
        let mut unit_field_val: u8 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &self._bitfield_1 as *const _ as *const u8,
                &mut unit_field_val as *mut u8 as *mut u8,
                1usize,
            )
        };
        unit_field_val &= !mask;
        unit_field_val |= (val << 0usize) & mask;
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &unit_field_val as *const _ as *const u8,
                &mut self._bitfield_1 as *mut _ as *mut u8,
                1usize,
            );
        }
    }
    #[inline]
    pub fn new_bitfield_1(derp: ::std::os::raw::c_uint) -> u8 {
        (0 | ((derp as u32 as u8) << 0usize) & (0x1 as u8))
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union B {
    pub _bitfield_1: u32,
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout_B() {
    assert_eq!(
        ::std::mem::size_of::<B>(),
        4usize,
        concat!("Size of: ", stringify!(B))
    );
    assert_eq!(
        ::std::mem::align_of::<B>(),
        4usize,
        concat!("Alignment of ", stringify!(B))
    );
}
impl Default for B {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl B {
    #[inline]
    pub fn foo(&self) -> ::std::os::raw::c_uint {
        let mut unit_field_val: u32 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &self._bitfield_1 as *const _ as *const u8,
                &mut unit_field_val as *mut u32 as *mut u8,
                4usize,
            )
        };
        let mask = 0x7fffffff as u32;
        let val = (unit_field_val & mask) >> 0usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_foo(&mut self, val: ::std::os::raw::c_uint) {
        let mask = 0x7fffffff as u32;
        let val = val as u32 as u32;
        let mut unit_field_val: u32 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &self._bitfield_1 as *const _ as *const u8,
                &mut unit_field_val as *mut u32 as *mut u8,
                4usize,
            )
        };
        unit_field_val &= !mask;
        unit_field_val |= (val << 0usize) & mask;
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &unit_field_val as *const _ as *const u8,
                &mut self._bitfield_1 as *mut _ as *mut u8,
                4usize,
            );
        }
    }
    #[inline]
    pub fn bar(&self) -> ::std::os::raw::c_uchar {
        let mut unit_field_val: u32 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &self._bitfield_1 as *const _ as *const u8,
                &mut unit_field_val as *mut u32 as *mut u8,
                4usize,
            )
        };
        let mask = 0x80000000 as u32;
        let val = (unit_field_val & mask) >> 31usize;
        unsafe { ::std::mem::transmute(val as u8) }
    }
    #[inline]
    pub fn set_bar(&mut self, val: ::std::os::raw::c_uchar) {
        let mask = 0x80000000 as u32;
        let val = val as u8 as u32;
        let mut unit_field_val: u32 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &self._bitfield_1 as *const _ as *const u8,
                &mut unit_field_val as *mut u32 as *mut u8,
                4usize,
            )
        };
        unit_field_val &= !mask;
        unit_field_val |= (val << 31usize) & mask;
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &unit_field_val as *const _ as *const u8,
                &mut self._bitfield_1 as *mut _ as *mut u8,
                4usize,
            );
        }
    }
    #[inline]
    pub fn new_bitfield_1(foo: ::std::os::raw::c_uint, bar: ::std::os::raw::c_uchar) -> u32 {
        ((0 | ((foo as u32 as u32) << 0usize) & (0x7fffffff as u32))
            | ((bar as u8 as u32) << 31usize) & (0x80000000 as u32))
    }
}
