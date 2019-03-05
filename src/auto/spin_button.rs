// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Adjustment;
use Buildable;
use CellEditable;
use Editable;
use Entry;
use Orientable;
use SpinButtonUpdatePolicy;
use SpinType;
use Widget;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib_wrapper! {
    pub struct SpinButton(Object<gtk_sys::GtkSpinButton, gtk_sys::GtkSpinButtonClass, SpinButtonClass>) @extends Entry, Widget, @implements Buildable, CellEditable, Editable, Orientable;

    match fn {
        get_type => || gtk_sys::gtk_spin_button_get_type(),
    }
}

impl SpinButton {
    pub fn new<P: IsA<Adjustment>>(adjustment: Option<&P>, climb_rate: f64, digits: u32) -> SpinButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_spin_button_new(adjustment.map(|p| p.as_ref()).to_glib_none().0, climb_rate, digits)).unsafe_cast()
        }
    }

    pub fn new_with_range(min: f64, max: f64, step: f64) -> SpinButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_spin_button_new_with_range(min, max, step)).unsafe_cast()
        }
    }
}

pub const NONE_SPIN_BUTTON: Option<&SpinButton> = None;

pub trait SpinButtonExt: 'static {
    fn configure<P: IsA<Adjustment>>(&self, adjustment: Option<&P>, climb_rate: f64, digits: u32);

    fn get_adjustment(&self) -> Adjustment;

    fn get_digits(&self) -> u32;

    fn get_increments(&self) -> (f64, f64);

    fn get_numeric(&self) -> bool;

    fn get_range(&self) -> (f64, f64);

    fn get_snap_to_ticks(&self) -> bool;

    fn get_update_policy(&self) -> SpinButtonUpdatePolicy;

    fn get_value(&self) -> f64;

    fn get_value_as_int(&self) -> i32;

    fn get_wrap(&self) -> bool;

    fn set_adjustment<P: IsA<Adjustment>>(&self, adjustment: &P);

    fn set_digits(&self, digits: u32);

    fn set_increments(&self, step: f64, page: f64);

    fn set_numeric(&self, numeric: bool);

    fn set_range(&self, min: f64, max: f64);

    fn set_snap_to_ticks(&self, snap_to_ticks: bool);

    fn set_update_policy(&self, policy: SpinButtonUpdatePolicy);

    fn set_value(&self, value: f64);

    fn set_wrap(&self, wrap: bool);

    fn spin(&self, direction: SpinType, increment: f64);

    fn update(&self);

    fn get_property_climb_rate(&self) -> f64;

    fn set_property_climb_rate(&self, climb_rate: f64);

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_climb_rate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_digits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_numeric_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_snap_to_ticks_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_update_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wrap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SpinButton>> SpinButtonExt for O {
    fn configure<P: IsA<Adjustment>>(&self, adjustment: Option<&P>, climb_rate: f64, digits: u32) {
        unsafe {
            gtk_sys::gtk_spin_button_configure(self.as_ref().to_glib_none().0, adjustment.map(|p| p.as_ref()).to_glib_none().0, climb_rate, digits);
        }
    }

    fn get_adjustment(&self) -> Adjustment {
        unsafe {
            from_glib_none(gtk_sys::gtk_spin_button_get_adjustment(self.as_ref().to_glib_none().0))
        }
    }

    fn get_digits(&self) -> u32 {
        unsafe {
            gtk_sys::gtk_spin_button_get_digits(self.as_ref().to_glib_none().0)
        }
    }

    fn get_increments(&self) -> (f64, f64) {
        unsafe {
            let mut step = mem::uninitialized();
            let mut page = mem::uninitialized();
            gtk_sys::gtk_spin_button_get_increments(self.as_ref().to_glib_none().0, &mut step, &mut page);
            (step, page)
        }
    }

    fn get_numeric(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_spin_button_get_numeric(self.as_ref().to_glib_none().0))
        }
    }

    fn get_range(&self) -> (f64, f64) {
        unsafe {
            let mut min = mem::uninitialized();
            let mut max = mem::uninitialized();
            gtk_sys::gtk_spin_button_get_range(self.as_ref().to_glib_none().0, &mut min, &mut max);
            (min, max)
        }
    }

    fn get_snap_to_ticks(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_spin_button_get_snap_to_ticks(self.as_ref().to_glib_none().0))
        }
    }

    fn get_update_policy(&self) -> SpinButtonUpdatePolicy {
        unsafe {
            from_glib(gtk_sys::gtk_spin_button_get_update_policy(self.as_ref().to_glib_none().0))
        }
    }

    fn get_value(&self) -> f64 {
        unsafe {
            gtk_sys::gtk_spin_button_get_value(self.as_ref().to_glib_none().0)
        }
    }

    fn get_value_as_int(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_spin_button_get_value_as_int(self.as_ref().to_glib_none().0)
        }
    }

    fn get_wrap(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_spin_button_get_wrap(self.as_ref().to_glib_none().0))
        }
    }

    fn set_adjustment<P: IsA<Adjustment>>(&self, adjustment: &P) {
        unsafe {
            gtk_sys::gtk_spin_button_set_adjustment(self.as_ref().to_glib_none().0, adjustment.as_ref().to_glib_none().0);
        }
    }

    fn set_digits(&self, digits: u32) {
        unsafe {
            gtk_sys::gtk_spin_button_set_digits(self.as_ref().to_glib_none().0, digits);
        }
    }

    fn set_increments(&self, step: f64, page: f64) {
        unsafe {
            gtk_sys::gtk_spin_button_set_increments(self.as_ref().to_glib_none().0, step, page);
        }
    }

    fn set_numeric(&self, numeric: bool) {
        unsafe {
            gtk_sys::gtk_spin_button_set_numeric(self.as_ref().to_glib_none().0, numeric.to_glib());
        }
    }

    fn set_range(&self, min: f64, max: f64) {
        unsafe {
            gtk_sys::gtk_spin_button_set_range(self.as_ref().to_glib_none().0, min, max);
        }
    }

    fn set_snap_to_ticks(&self, snap_to_ticks: bool) {
        unsafe {
            gtk_sys::gtk_spin_button_set_snap_to_ticks(self.as_ref().to_glib_none().0, snap_to_ticks.to_glib());
        }
    }

    fn set_update_policy(&self, policy: SpinButtonUpdatePolicy) {
        unsafe {
            gtk_sys::gtk_spin_button_set_update_policy(self.as_ref().to_glib_none().0, policy.to_glib());
        }
    }

    fn set_value(&self, value: f64) {
        unsafe {
            gtk_sys::gtk_spin_button_set_value(self.as_ref().to_glib_none().0, value);
        }
    }

    fn set_wrap(&self, wrap: bool) {
        unsafe {
            gtk_sys::gtk_spin_button_set_wrap(self.as_ref().to_glib_none().0, wrap.to_glib());
        }
    }

    fn spin(&self, direction: SpinType, increment: f64) {
        unsafe {
            gtk_sys::gtk_spin_button_spin(self.as_ref().to_glib_none().0, direction.to_glib(), increment);
        }
    }

    fn update(&self) {
        unsafe {
            gtk_sys::gtk_spin_button_update(self.as_ref().to_glib_none().0);
        }
    }

    fn get_property_climb_rate(&self) -> f64 {
        unsafe {
            let mut value = Value::from_type(<f64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"climb-rate\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_climb_rate(&self, climb_rate: f64) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"climb-rate\0".as_ptr() as *const _, Value::from(&climb_rate).to_glib_none().0);
        }
    }

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::adjustment\0".as_ptr() as *const _,
                Some(transmute(notify_adjustment_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_climb_rate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::climb-rate\0".as_ptr() as *const _,
                Some(transmute(notify_climb_rate_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_digits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::digits\0".as_ptr() as *const _,
                Some(transmute(notify_digits_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_numeric_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::numeric\0".as_ptr() as *const _,
                Some(transmute(notify_numeric_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_snap_to_ticks_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::snap-to-ticks\0".as_ptr() as *const _,
                Some(transmute(notify_snap_to_ticks_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_update_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::update-policy\0".as_ptr() as *const _,
                Some(transmute(notify_update_policy_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::value\0".as_ptr() as *const _,
                Some(transmute(notify_value_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_wrap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::wrap\0".as_ptr() as *const _,
                Some(transmute(notify_wrap_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_adjustment_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkSpinButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<SpinButton> {
    let f: &F = &*(f as *const F);
    f(&SpinButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_climb_rate_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkSpinButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<SpinButton> {
    let f: &F = &*(f as *const F);
    f(&SpinButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_digits_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkSpinButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<SpinButton> {
    let f: &F = &*(f as *const F);
    f(&SpinButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_numeric_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkSpinButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<SpinButton> {
    let f: &F = &*(f as *const F);
    f(&SpinButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_snap_to_ticks_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkSpinButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<SpinButton> {
    let f: &F = &*(f as *const F);
    f(&SpinButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_update_policy_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkSpinButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<SpinButton> {
    let f: &F = &*(f as *const F);
    f(&SpinButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_value_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkSpinButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<SpinButton> {
    let f: &F = &*(f as *const F);
    f(&SpinButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_wrap_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkSpinButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<SpinButton> {
    let f: &F = &*(f as *const F);
    f(&SpinButton::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for SpinButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SpinButton")
    }
}
