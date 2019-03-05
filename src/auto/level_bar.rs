// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use LevelBarMode;
use Orientable;
use Widget;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gtk_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib_wrapper! {
    pub struct LevelBar(Object<gtk_sys::GtkLevelBar, gtk_sys::GtkLevelBarClass, LevelBarClass>) @extends Widget, @implements Buildable, Orientable;

    match fn {
        get_type => || gtk_sys::gtk_level_bar_get_type(),
    }
}

impl LevelBar {
    pub fn new() -> LevelBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_level_bar_new()).unsafe_cast()
        }
    }

    pub fn new_for_interval(min_value: f64, max_value: f64) -> LevelBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_level_bar_new_for_interval(min_value, max_value)).unsafe_cast()
        }
    }
}

impl Default for LevelBar {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_LEVEL_BAR: Option<&LevelBar> = None;

pub trait LevelBarExt: 'static {
    fn add_offset_value(&self, name: &str, value: f64);

    fn get_inverted(&self) -> bool;

    fn get_max_value(&self) -> f64;

    fn get_min_value(&self) -> f64;

    fn get_mode(&self) -> LevelBarMode;

    fn get_offset_value(&self, name: Option<&str>) -> Option<f64>;

    fn get_value(&self) -> f64;

    fn remove_offset_value(&self, name: Option<&str>);

    fn set_inverted(&self, inverted: bool);

    fn set_max_value(&self, value: f64);

    fn set_min_value(&self, value: f64);

    fn set_mode(&self, mode: LevelBarMode);

    fn set_value(&self, value: f64);

    fn connect_offset_changed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_max_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_min_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<LevelBar>> LevelBarExt for O {
    fn add_offset_value(&self, name: &str, value: f64) {
        unsafe {
            gtk_sys::gtk_level_bar_add_offset_value(self.as_ref().to_glib_none().0, name.to_glib_none().0, value);
        }
    }

    fn get_inverted(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_level_bar_get_inverted(self.as_ref().to_glib_none().0))
        }
    }

    fn get_max_value(&self) -> f64 {
        unsafe {
            gtk_sys::gtk_level_bar_get_max_value(self.as_ref().to_glib_none().0)
        }
    }

    fn get_min_value(&self) -> f64 {
        unsafe {
            gtk_sys::gtk_level_bar_get_min_value(self.as_ref().to_glib_none().0)
        }
    }

    fn get_mode(&self) -> LevelBarMode {
        unsafe {
            from_glib(gtk_sys::gtk_level_bar_get_mode(self.as_ref().to_glib_none().0))
        }
    }

    fn get_offset_value(&self, name: Option<&str>) -> Option<f64> {
        unsafe {
            let mut value = mem::uninitialized();
            let ret = from_glib(gtk_sys::gtk_level_bar_get_offset_value(self.as_ref().to_glib_none().0, name.to_glib_none().0, &mut value));
            if ret { Some(value) } else { None }
        }
    }

    fn get_value(&self) -> f64 {
        unsafe {
            gtk_sys::gtk_level_bar_get_value(self.as_ref().to_glib_none().0)
        }
    }

    fn remove_offset_value(&self, name: Option<&str>) {
        unsafe {
            gtk_sys::gtk_level_bar_remove_offset_value(self.as_ref().to_glib_none().0, name.to_glib_none().0);
        }
    }

    fn set_inverted(&self, inverted: bool) {
        unsafe {
            gtk_sys::gtk_level_bar_set_inverted(self.as_ref().to_glib_none().0, inverted.to_glib());
        }
    }

    fn set_max_value(&self, value: f64) {
        unsafe {
            gtk_sys::gtk_level_bar_set_max_value(self.as_ref().to_glib_none().0, value);
        }
    }

    fn set_min_value(&self, value: f64) {
        unsafe {
            gtk_sys::gtk_level_bar_set_min_value(self.as_ref().to_glib_none().0, value);
        }
    }

    fn set_mode(&self, mode: LevelBarMode) {
        unsafe {
            gtk_sys::gtk_level_bar_set_mode(self.as_ref().to_glib_none().0, mode.to_glib());
        }
    }

    fn set_value(&self, value: f64) {
        unsafe {
            gtk_sys::gtk_level_bar_set_value(self.as_ref().to_glib_none().0, value);
        }
    }

    fn connect_offset_changed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"offset-changed\0".as_ptr() as *const _,
                Some(transmute(offset_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::inverted\0".as_ptr() as *const _,
                Some(transmute(notify_inverted_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_max_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::max-value\0".as_ptr() as *const _,
                Some(transmute(notify_max_value_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_min_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::min-value\0".as_ptr() as *const _,
                Some(transmute(notify_min_value_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::mode\0".as_ptr() as *const _,
                Some(transmute(notify_mode_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::value\0".as_ptr() as *const _,
                Some(transmute(notify_value_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn offset_changed_trampoline<P, F: Fn(&P, &str) + 'static>(this: *mut gtk_sys::GtkLevelBar, name: *mut libc::c_char, f: glib_sys::gpointer)
where P: IsA<LevelBar> {
    let f: &F = &*(f as *const F);
    f(&LevelBar::from_glib_borrow(this).unsafe_cast(), &GString::from_glib_borrow(name))
}

unsafe extern "C" fn notify_inverted_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkLevelBar, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<LevelBar> {
    let f: &F = &*(f as *const F);
    f(&LevelBar::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_max_value_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkLevelBar, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<LevelBar> {
    let f: &F = &*(f as *const F);
    f(&LevelBar::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_min_value_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkLevelBar, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<LevelBar> {
    let f: &F = &*(f as *const F);
    f(&LevelBar::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_mode_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkLevelBar, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<LevelBar> {
    let f: &F = &*(f as *const F);
    f(&LevelBar::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_value_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkLevelBar, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<LevelBar> {
    let f: &F = &*(f as *const F);
    f(&LevelBar::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for LevelBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LevelBar")
    }
}
