// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use EventController;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use PadActionType;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use Window;
use ffi;
use gdk;
use gio;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct PadController(Object<ffi::GtkPadController, ffi::GtkPadControllerClass>): EventController;

    match fn {
        get_type => || ffi::gtk_pad_controller_get_type(),
    }
}

impl PadController {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn new<'a, P: IsA<Window>, Q: IsA<gio::ActionGroup>, R: IsA<gdk::Device> + 'a, S: Into<Option<&'a R>>>(window: &P, group: &Q, pad: S) -> PadController {
        skip_assert_initialized!();
        let pad = pad.into();
        let pad = pad.to_glib_none();
        unsafe {
            from_glib_full(ffi::gtk_pad_controller_new(window.to_glib_none().0, group.to_glib_none().0, pad.0))
        }
    }
}

pub trait PadControllerExt {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_action(&self, type_: PadActionType, index: i32, mode: i32, label: &str, action_name: &str);

    //#[cfg(any(feature = "v3_22", feature = "dox"))]
    //fn set_action_entries(&self, entries: /*Ignored*/&[&PadActionEntry]);

    fn get_property_action_group(&self) -> Option<gio::ActionGroup>;

    fn get_property_pad(&self) -> Option<gdk::Device>;

    fn connect_property_action_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<PadController> + IsA<glib::object::Object>> PadControllerExt for O {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_action(&self, type_: PadActionType, index: i32, mode: i32, label: &str, action_name: &str) {
        unsafe {
            ffi::gtk_pad_controller_set_action(self.to_glib_none().0, type_.to_glib(), index, mode, label.to_glib_none().0, action_name.to_glib_none().0);
        }
    }

    //#[cfg(any(feature = "v3_22", feature = "dox"))]
    //fn set_action_entries(&self, entries: /*Ignored*/&[&PadActionEntry]) {
    //    unsafe { TODO: call ffi::gtk_pad_controller_set_action_entries() }
    //}

    fn get_property_action_group(&self) -> Option<gio::ActionGroup> {
        unsafe {
            let mut value = Value::from_type(<gio::ActionGroup as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "action-group".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_pad(&self) -> Option<gdk::Device> {
        unsafe {
            let mut value = Value::from_type(<gdk::Device as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "pad".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_property_action_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::action-group",
                transmute(notify_action_group_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_pad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::pad",
                transmute(notify_pad_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_action_group_trampoline<P>(this: *mut ffi::GtkPadController, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<PadController> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PadController::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_pad_trampoline<P>(this: *mut ffi::GtkPadController, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<PadController> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PadController::from_glib_borrow(this).downcast_unchecked())
}
