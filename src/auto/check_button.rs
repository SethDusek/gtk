// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Button;
use Container;
use ToggleButton;
use Widget;
use glib::object::Cast;
use glib::translate::*;
use gtk_sys;
use std::fmt;

glib_wrapper! {
    pub struct CheckButton(Object<gtk_sys::GtkCheckButton, gtk_sys::GtkCheckButtonClass, CheckButtonClass>) @extends ToggleButton, Button, Bin, Container, Widget, @implements Buildable, Actionable;

    match fn {
        get_type => || gtk_sys::gtk_check_button_get_type(),
    }
}

impl CheckButton {
    pub fn new() -> CheckButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_check_button_new()).unsafe_cast()
        }
    }

    pub fn new_with_label(label: &str) -> CheckButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_check_button_new_with_label(label.to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn new_with_mnemonic(label: &str) -> CheckButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_check_button_new_with_mnemonic(label.to_glib_none().0)).unsafe_cast()
        }
    }
}

impl Default for CheckButton {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_CHECK_BUTTON: Option<&CheckButton> = None;

impl fmt::Display for CheckButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CheckButton")
    }
}
