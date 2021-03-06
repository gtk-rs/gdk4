// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk_sys;
use glib;
use glib::translate::*;
use glib::GString;
use std::fmt;
use std::mem;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ContentFormats(Shared<gdk_sys::GdkContentFormats>);

    match fn {
        ref => |ptr| gdk_sys::gdk_content_formats_ref(ptr),
        unref => |ptr| gdk_sys::gdk_content_formats_unref(ptr),
        get_type => || gdk_sys::gdk_content_formats_get_type(),
    }
}

impl ContentFormats {
    pub fn new(mime_types: &[&str]) -> ContentFormats {
        assert_initialized_main_thread!();
        let n_mime_types = mime_types.len() as u32;
        unsafe {
            from_glib_full(gdk_sys::gdk_content_formats_new(
                mime_types.to_glib_none().0,
                n_mime_types,
            ))
        }
    }

    pub fn new_for_gtype(type_: glib::types::Type) -> ContentFormats {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gdk_sys::gdk_content_formats_new_for_gtype(type_.to_glib())) }
    }

    pub fn contain_gtype(&self, type_: glib::types::Type) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_content_formats_contain_gtype(
                self.to_glib_none().0,
                type_.to_glib(),
            ))
        }
    }

    pub fn contain_mime_type(&self, mime_type: &str) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_content_formats_contain_mime_type(
                self.to_glib_none().0,
                mime_type.to_glib_none().0,
            ))
        }
    }

    pub fn get_mime_types(&self) -> (Vec<GString>, usize) {
        unsafe {
            let mut n_mime_types = mem::MaybeUninit::uninit();
            let ret =
                FromGlibPtrContainer::from_glib_none(gdk_sys::gdk_content_formats_get_mime_types(
                    self.to_glib_none().0,
                    n_mime_types.as_mut_ptr(),
                ));
            let n_mime_types = n_mime_types.assume_init();
            (ret, n_mime_types)
        }
    }

    pub fn match_(&self, second: &ContentFormats) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_content_formats_match(
                self.to_glib_none().0,
                second.to_glib_none().0,
            ))
        }
    }

    pub fn match_gtype(&self, second: &ContentFormats) -> glib::types::Type {
        unsafe {
            from_glib(gdk_sys::gdk_content_formats_match_gtype(
                self.to_glib_none().0,
                second.to_glib_none().0,
            ))
        }
    }

    pub fn match_mime_type(&self, second: &ContentFormats) -> Option<GString> {
        unsafe {
            from_glib_none(gdk_sys::gdk_content_formats_match_mime_type(
                self.to_glib_none().0,
                second.to_glib_none().0,
            ))
        }
    }

    pub fn print(&self, string: &mut glib::String) {
        unsafe {
            gdk_sys::gdk_content_formats_print(self.to_glib_none().0, string.to_glib_none_mut().0);
        }
    }

    fn to_string(&self) -> GString {
        unsafe {
            from_glib_full(gdk_sys::gdk_content_formats_to_string(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn union(&self, second: &ContentFormats) -> Option<ContentFormats> {
        unsafe {
            from_glib_full(gdk_sys::gdk_content_formats_union(
                self.to_glib_full(),
                second.to_glib_none().0,
            ))
        }
    }

    pub fn union_deserialize_gtypes(&self) -> Option<ContentFormats> {
        unsafe {
            from_glib_full(gdk_sys::gdk_content_formats_union_deserialize_gtypes(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn union_deserialize_mime_types(&self) -> Option<ContentFormats> {
        unsafe {
            from_glib_full(gdk_sys::gdk_content_formats_union_deserialize_mime_types(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn union_serialize_gtypes(&self) -> Option<ContentFormats> {
        unsafe {
            from_glib_full(gdk_sys::gdk_content_formats_union_serialize_gtypes(
                self.to_glib_full(),
            ))
        }
    }

    pub fn union_serialize_mime_types(&self) -> Option<ContentFormats> {
        unsafe {
            from_glib_full(gdk_sys::gdk_content_formats_union_serialize_mime_types(
                self.to_glib_full(),
            ))
        }
    }
}

impl fmt::Display for ContentFormats {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
