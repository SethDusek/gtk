// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use Container;
use ReliefStyle;
use ToolItem;
use ToolShell;
use Widget;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gtk_sys;
use pango;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct ToolItemGroup(Object<gtk_sys::GtkToolItemGroup, gtk_sys::GtkToolItemGroupClass, ToolItemGroupClass>) @extends Container, Widget, @implements Buildable, ToolShell;

    match fn {
        get_type => || gtk_sys::gtk_tool_item_group_get_type(),
    }
}

impl ToolItemGroup {
    pub fn new(label: &str) -> ToolItemGroup {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_tool_item_group_new(label.to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_TOOL_ITEM_GROUP: Option<&ToolItemGroup> = None;

pub trait ToolItemGroupExt: 'static {
    fn get_collapsed(&self) -> bool;

    fn get_drop_item(&self, x: i32, y: i32) -> Option<ToolItem>;

    fn get_ellipsize(&self) -> pango::EllipsizeMode;

    fn get_header_relief(&self) -> ReliefStyle;

    fn get_item_position<P: IsA<ToolItem>>(&self, item: &P) -> i32;

    fn get_label(&self) -> Option<GString>;

    fn get_label_widget(&self) -> Option<Widget>;

    fn get_n_items(&self) -> u32;

    fn get_nth_item(&self, index: u32) -> Option<ToolItem>;

    fn insert<P: IsA<ToolItem>>(&self, item: &P, position: i32);

    fn set_collapsed(&self, collapsed: bool);

    fn set_ellipsize(&self, ellipsize: pango::EllipsizeMode);

    fn set_header_relief(&self, style: ReliefStyle);

    fn set_item_position<P: IsA<ToolItem>>(&self, item: &P, position: i32);

    fn set_label(&self, label: &str);

    fn set_label_widget<P: IsA<Widget>>(&self, label_widget: &P);

    fn get_item_expand<T: IsA<ToolItem>>(&self, item: &T) -> bool;

    fn set_item_expand<T: IsA<ToolItem>>(&self, item: &T, expand: bool);

    fn get_item_fill<T: IsA<ToolItem>>(&self, item: &T) -> bool;

    fn set_item_fill<T: IsA<ToolItem>>(&self, item: &T, fill: bool);

    fn get_item_homogeneous<T: IsA<ToolItem>>(&self, item: &T) -> bool;

    fn set_item_homogeneous<T: IsA<ToolItem>>(&self, item: &T, homogeneous: bool);

    fn get_item_new_row<T: IsA<ToolItem>>(&self, item: &T) -> bool;

    fn set_item_new_row<T: IsA<ToolItem>>(&self, item: &T, new_row: bool);

    fn connect_property_collapsed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ellipsize_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_header_relief_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ToolItemGroup>> ToolItemGroupExt for O {
    fn get_collapsed(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_tool_item_group_get_collapsed(self.as_ref().to_glib_none().0))
        }
    }

    fn get_drop_item(&self, x: i32, y: i32) -> Option<ToolItem> {
        unsafe {
            from_glib_none(gtk_sys::gtk_tool_item_group_get_drop_item(self.as_ref().to_glib_none().0, x, y))
        }
    }

    fn get_ellipsize(&self) -> pango::EllipsizeMode {
        unsafe {
            from_glib(gtk_sys::gtk_tool_item_group_get_ellipsize(self.as_ref().to_glib_none().0))
        }
    }

    fn get_header_relief(&self) -> ReliefStyle {
        unsafe {
            from_glib(gtk_sys::gtk_tool_item_group_get_header_relief(self.as_ref().to_glib_none().0))
        }
    }

    fn get_item_position<P: IsA<ToolItem>>(&self, item: &P) -> i32 {
        unsafe {
            gtk_sys::gtk_tool_item_group_get_item_position(self.as_ref().to_glib_none().0, item.as_ref().to_glib_none().0)
        }
    }

    fn get_label(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_tool_item_group_get_label(self.as_ref().to_glib_none().0))
        }
    }

    fn get_label_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_tool_item_group_get_label_widget(self.as_ref().to_glib_none().0))
        }
    }

    fn get_n_items(&self) -> u32 {
        unsafe {
            gtk_sys::gtk_tool_item_group_get_n_items(self.as_ref().to_glib_none().0)
        }
    }

    fn get_nth_item(&self, index: u32) -> Option<ToolItem> {
        unsafe {
            from_glib_none(gtk_sys::gtk_tool_item_group_get_nth_item(self.as_ref().to_glib_none().0, index))
        }
    }

    fn insert<P: IsA<ToolItem>>(&self, item: &P, position: i32) {
        unsafe {
            gtk_sys::gtk_tool_item_group_insert(self.as_ref().to_glib_none().0, item.as_ref().to_glib_none().0, position);
        }
    }

    fn set_collapsed(&self, collapsed: bool) {
        unsafe {
            gtk_sys::gtk_tool_item_group_set_collapsed(self.as_ref().to_glib_none().0, collapsed.to_glib());
        }
    }

    fn set_ellipsize(&self, ellipsize: pango::EllipsizeMode) {
        unsafe {
            gtk_sys::gtk_tool_item_group_set_ellipsize(self.as_ref().to_glib_none().0, ellipsize.to_glib());
        }
    }

    fn set_header_relief(&self, style: ReliefStyle) {
        unsafe {
            gtk_sys::gtk_tool_item_group_set_header_relief(self.as_ref().to_glib_none().0, style.to_glib());
        }
    }

    fn set_item_position<P: IsA<ToolItem>>(&self, item: &P, position: i32) {
        unsafe {
            gtk_sys::gtk_tool_item_group_set_item_position(self.as_ref().to_glib_none().0, item.as_ref().to_glib_none().0, position);
        }
    }

    fn set_label(&self, label: &str) {
        unsafe {
            gtk_sys::gtk_tool_item_group_set_label(self.as_ref().to_glib_none().0, label.to_glib_none().0);
        }
    }

    fn set_label_widget<P: IsA<Widget>>(&self, label_widget: &P) {
        unsafe {
            gtk_sys::gtk_tool_item_group_set_label_widget(self.as_ref().to_glib_none().0, label_widget.as_ref().to_glib_none().0);
        }
    }

    fn get_item_expand<T: IsA<ToolItem>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gtk_sys::gtk_container_child_get_property(self.to_glib_none().0 as *mut gtk_sys::GtkContainer, item.to_glib_none().0 as *mut _, b"expand\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_item_expand<T: IsA<ToolItem>>(&self, item: &T, expand: bool) {
        unsafe {
            gtk_sys::gtk_container_child_set_property(self.to_glib_none().0 as *mut gtk_sys::GtkContainer, item.to_glib_none().0 as *mut _, b"expand\0".as_ptr() as *const _, Value::from(&expand).to_glib_none().0);
        }
    }

    fn get_item_fill<T: IsA<ToolItem>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gtk_sys::gtk_container_child_get_property(self.to_glib_none().0 as *mut gtk_sys::GtkContainer, item.to_glib_none().0 as *mut _, b"fill\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_item_fill<T: IsA<ToolItem>>(&self, item: &T, fill: bool) {
        unsafe {
            gtk_sys::gtk_container_child_set_property(self.to_glib_none().0 as *mut gtk_sys::GtkContainer, item.to_glib_none().0 as *mut _, b"fill\0".as_ptr() as *const _, Value::from(&fill).to_glib_none().0);
        }
    }

    fn get_item_homogeneous<T: IsA<ToolItem>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gtk_sys::gtk_container_child_get_property(self.to_glib_none().0 as *mut gtk_sys::GtkContainer, item.to_glib_none().0 as *mut _, b"homogeneous\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_item_homogeneous<T: IsA<ToolItem>>(&self, item: &T, homogeneous: bool) {
        unsafe {
            gtk_sys::gtk_container_child_set_property(self.to_glib_none().0 as *mut gtk_sys::GtkContainer, item.to_glib_none().0 as *mut _, b"homogeneous\0".as_ptr() as *const _, Value::from(&homogeneous).to_glib_none().0);
        }
    }

    fn get_item_new_row<T: IsA<ToolItem>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gtk_sys::gtk_container_child_get_property(self.to_glib_none().0 as *mut gtk_sys::GtkContainer, item.to_glib_none().0 as *mut _, b"new-row\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_item_new_row<T: IsA<ToolItem>>(&self, item: &T, new_row: bool) {
        unsafe {
            gtk_sys::gtk_container_child_set_property(self.to_glib_none().0 as *mut gtk_sys::GtkContainer, item.to_glib_none().0 as *mut _, b"new-row\0".as_ptr() as *const _, Value::from(&new_row).to_glib_none().0);
        }
    }

    fn connect_property_collapsed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::collapsed\0".as_ptr() as *const _,
                Some(transmute(notify_collapsed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_ellipsize_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::ellipsize\0".as_ptr() as *const _,
                Some(transmute(notify_ellipsize_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_header_relief_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::header-relief\0".as_ptr() as *const _,
                Some(transmute(notify_header_relief_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::label\0".as_ptr() as *const _,
                Some(transmute(notify_label_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_label_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::label-widget\0".as_ptr() as *const _,
                Some(transmute(notify_label_widget_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_collapsed_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkToolItemGroup, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<ToolItemGroup> {
    let f: &F = &*(f as *const F);
    f(&ToolItemGroup::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_ellipsize_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkToolItemGroup, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<ToolItemGroup> {
    let f: &F = &*(f as *const F);
    f(&ToolItemGroup::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_header_relief_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkToolItemGroup, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<ToolItemGroup> {
    let f: &F = &*(f as *const F);
    f(&ToolItemGroup::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_label_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkToolItemGroup, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<ToolItemGroup> {
    let f: &F = &*(f as *const F);
    f(&ToolItemGroup::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_label_widget_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkToolItemGroup, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<ToolItemGroup> {
    let f: &F = &*(f as *const F);
    f(&ToolItemGroup::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for ToolItemGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ToolItemGroup")
    }
}
