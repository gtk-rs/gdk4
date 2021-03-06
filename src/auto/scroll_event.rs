// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk_sys;
use glib::translate::*;
use std::fmt;
use std::mem;
use Event;
use ScrollDirection;

glib_wrapper! {
    pub struct ScrollEvent(Object<gdk_sys::GdkScrollEvent, ScrollEventClass>) @extends Event;

    match fn {
        get_type => || gdk_sys::gdk_scroll_event_get_type(),
    }
}

impl ScrollEvent {
    pub fn get_deltas(&self) -> (f64, f64) {
        unsafe {
            let mut delta_x = mem::MaybeUninit::uninit();
            let mut delta_y = mem::MaybeUninit::uninit();
            gdk_sys::gdk_scroll_event_get_deltas(
                self.to_glib_none().0,
                delta_x.as_mut_ptr(),
                delta_y.as_mut_ptr(),
            );
            let delta_x = delta_x.assume_init();
            let delta_y = delta_y.assume_init();
            (delta_x, delta_y)
        }
    }

    pub fn get_direction(&self) -> ScrollDirection {
        unsafe {
            from_glib(gdk_sys::gdk_scroll_event_get_direction(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn is_stop(&self) -> bool {
        unsafe { from_glib(gdk_sys::gdk_scroll_event_is_stop(self.to_glib_none().0)) }
    }
}

impl fmt::Display for ScrollEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScrollEvent")
    }
}
