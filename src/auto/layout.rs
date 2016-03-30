// This file was generated by gir (becf3b4) from gir-files (11e0e6d)
// DO NOT EDIT

use Adjustment;
use Buildable;
use Container;
use Scrollable;
use Widget;
use ffi;
use gdk;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    pub struct Layout(Object<ffi::GtkLayout>): Container, Widget, Buildable, Scrollable;

    match fn {
        get_type => || ffi::gtk_layout_get_type(),
    }
}

impl Layout {
    pub fn new(hadjustment: Option<&Adjustment>, vadjustment: Option<&Adjustment>) -> Layout {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_layout_new(hadjustment.to_glib_none().0, vadjustment.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn get_bin_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_layout_get_bin_window(self.to_glib_none().0))
        }
    }

    pub fn get_hadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_layout_get_hadjustment(self.to_glib_none().0))
        }
    }

    pub fn get_size(&self) -> (u32, u32) {
        unsafe {
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::gtk_layout_get_size(self.to_glib_none().0, &mut width, &mut height);
            (width, height)
        }
    }

    pub fn get_vadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_layout_get_vadjustment(self.to_glib_none().0))
        }
    }

    pub fn move_<T: IsA<Widget>>(&self, child_widget: &T, x: i32, y: i32) {
        unsafe {
            ffi::gtk_layout_move(self.to_glib_none().0, child_widget.to_glib_none().0, x, y);
        }
    }

    pub fn put<T: IsA<Widget>>(&self, child_widget: &T, x: i32, y: i32) {
        unsafe {
            ffi::gtk_layout_put(self.to_glib_none().0, child_widget.to_glib_none().0, x, y);
        }
    }

    pub fn set_hadjustment(&self, adjustment: Option<&Adjustment>) {
        unsafe {
            ffi::gtk_layout_set_hadjustment(self.to_glib_none().0, adjustment.to_glib_none().0);
        }
    }

    pub fn set_size(&self, width: u32, height: u32) {
        unsafe {
            ffi::gtk_layout_set_size(self.to_glib_none().0, width, height);
        }
    }

    pub fn set_vadjustment(&self, adjustment: Option<&Adjustment>) {
        unsafe {
            ffi::gtk_layout_set_vadjustment(self.to_glib_none().0, adjustment.to_glib_none().0);
        }
    }
}