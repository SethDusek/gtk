// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use CellEditable;
use Editable;
use Entry;
use Widget;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use gdk;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib;
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib::object::ObjectExt;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct SearchEntry(Object<gtk_sys::GtkSearchEntry, gtk_sys::GtkSearchEntryClass, SearchEntryClass>) @extends Entry, Widget, @implements Buildable, CellEditable, Editable;

    match fn {
        get_type => || gtk_sys::gtk_search_entry_get_type(),
    }
}

impl SearchEntry {
    pub fn new() -> SearchEntry {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_search_entry_new()).unsafe_cast()
        }
    }
}

impl Default for SearchEntry {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SEARCH_ENTRY: Option<&SearchEntry> = None;

pub trait SearchEntryExt: 'static {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn handle_event(&self, event: &gdk::Event) -> bool;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_next_match<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn emit_next_match(&self);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_previous_match<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn emit_previous_match(&self);

    fn connect_search_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_stop_search<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn emit_stop_search(&self);
}

impl<O: IsA<SearchEntry>> SearchEntryExt for O {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn handle_event(&self, event: &gdk::Event) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_search_entry_handle_event(self.as_ref().to_glib_none().0, mut_override(event.to_glib_none().0)))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_next_match<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"next-match\0".as_ptr() as *const _,
                Some(transmute(next_match_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn emit_next_match(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("next-match", &[]).unwrap() };
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_previous_match<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"previous-match\0".as_ptr() as *const _,
                Some(transmute(previous_match_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn emit_previous_match(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("previous-match", &[]).unwrap() };
    }

    fn connect_search_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"search-changed\0".as_ptr() as *const _,
                Some(transmute(search_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_stop_search<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"stop-search\0".as_ptr() as *const _,
                Some(transmute(stop_search_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn emit_stop_search(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("stop-search", &[]).unwrap() };
    }
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn next_match_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkSearchEntry, f: glib_sys::gpointer)
where P: IsA<SearchEntry> {
    let f: &F = &*(f as *const F);
    f(&SearchEntry::from_glib_borrow(this).unsafe_cast())
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn previous_match_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkSearchEntry, f: glib_sys::gpointer)
where P: IsA<SearchEntry> {
    let f: &F = &*(f as *const F);
    f(&SearchEntry::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn search_changed_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkSearchEntry, f: glib_sys::gpointer)
where P: IsA<SearchEntry> {
    let f: &F = &*(f as *const F);
    f(&SearchEntry::from_glib_borrow(this).unsafe_cast())
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn stop_search_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkSearchEntry, f: glib_sys::gpointer)
where P: IsA<SearchEntry> {
    let f: &F = &*(f as *const F);
    f(&SearchEntry::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for SearchEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SearchEntry")
    }
}
