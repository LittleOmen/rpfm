//---------------------------------------------------------------------------//
// Copyright (c) 2017-2020 Ismael Gutiérrez González. All rights reserved.
//
// This file is part of the Rusted PackFile Manager (RPFM) project,
// which can be found here: https://github.com/Frodo45127/rpfm.
//
// This file is licensed under the MIT license, which can be found here:
// https://github.com/Frodo45127/rpfm/blob/master/LICENSE.
//---------------------------------------------------------------------------//

/*!
Module containing the ffi functions used for custom widgets.
!*/

use qt_widgets::QLabel;
use qt_widgets::QMainWindow;
use qt_widgets::{QMessageBox, q_message_box};
use qt_widgets::QTableView;
use qt_widgets::QWidget;

use qt_gui::QPixmap;
use qt_gui::QStandardItemModel;

use qt_core::QAbstractItemModel;
use qt_core::QBox;
use qt_core::QObject;
use qt_core::QRegExp;
use qt_core::QSortFilterProxyModel;
use qt_core::QString;
use qt_core::QStringList;
use qt_core::QPtr;

use cpp_core::Ptr;

use crate::locale::qtr;
use crate::UI_STATE;

/// This function replaces the default editor widget for reference columns with a combobox, so you can select the reference data.
extern "C" { fn new_combobox_item_delegate(table_view: *mut QObject, column: i32, list: *const QStringList, is_editable: bool, max_lenght: i32); }
pub fn new_combobox_item_delegate_safe(table_view: &Ptr<QObject>, column: i32, list: Ptr<QStringList>, is_editable: bool, max_lenght: i32) {
    unsafe { new_combobox_item_delegate(table_view.as_mut_raw_ptr(), column, list.as_raw_ptr(), is_editable, max_lenght) }
}

/// This function changes the default editor widget for I32/64 cells on tables with a numeric one.
extern "C" { fn new_spinbox_item_delegate(table_view: *mut QObject, column: i32, integer_type: i32); }
pub fn new_spinbox_item_delegate_safe(table_view: &Ptr<QObject>, column: i32, integer_type: i32) {
    unsafe { new_spinbox_item_delegate(table_view.as_mut_raw_ptr(), column, integer_type) }
}

/// This function changes the default editor widget for F32 cells on tables with a numeric one.
extern "C" { fn new_doublespinbox_item_delegate(table_view: *mut QObject, column: i32); }
pub fn new_doublespinbox_item_delegate_safe(table_view: &Ptr<QObject>, column: i32) {
    unsafe { new_doublespinbox_item_delegate(table_view.as_mut_raw_ptr(), column) }
}

/// This function changes the default editor widget for String cells, to ensure the provided data is valid for the schema..
extern "C" { fn new_qstring_item_delegate(table_view: *mut QObject, column: i32, max_lenght: i32); }
pub fn new_qstring_item_delegate_safe(table_view: &Ptr<QObject>, column: i32, max_lenght: i32) {
    unsafe { new_qstring_item_delegate(table_view.as_mut_raw_ptr(), column, max_lenght) }
}

/// This function setup the special filter used for the PackFile Contents `TreeView`.
extern "C" { fn new_treeview_filter(parent: *mut QObject) -> *mut QSortFilterProxyModel; }
pub fn new_treeview_filter_safe(parent: QPtr<QObject>) ->  QBox<QSortFilterProxyModel> {
    unsafe { QBox::from_raw(new_treeview_filter(parent.as_mut_raw_ptr())) }
}

/// This function triggers the special filter used for the PackFile Contents `TreeView`. It has to be triggered here to work properly.
extern "C" { fn trigger_treeview_filter(filter: *const QSortFilterProxyModel, pattern: *mut QRegExp); }
pub fn trigger_treeview_filter_safe(filter: &QSortFilterProxyModel, pattern: &Ptr<QRegExp>) {
    unsafe { trigger_treeview_filter(filter, pattern.as_mut_raw_ptr()); }
}

/// This function allow us to create a model compatible with draggable items
extern "C" { fn new_packed_file_model() -> *mut QStandardItemModel; }
pub fn new_packed_file_model_safe() -> QBox<QStandardItemModel> {
    unsafe { QBox::from_raw(new_packed_file_model()) }
}

/// This function allow us to create a custom window.
extern "C" { fn new_q_main_window_custom(are_you_sure: extern fn(*mut QMainWindow, bool) -> bool) -> *mut QMainWindow; }
pub fn new_q_main_window_custom_safe(are_you_sure: extern fn(*mut QMainWindow, bool) -> bool) -> Ptr<QMainWindow> {
    unsafe { Ptr::from_raw(new_q_main_window_custom(are_you_sure)) }
}

//---------------------------------------------------------------------------//
// Freezing Columns stuff.
//---------------------------------------------------------------------------//

/// This function allows you to create a table capable of freezing columns.
extern "C" { fn new_tableview_frozen(parent: *mut QWidget) -> *mut QTableView; }
extern "C" { fn get_frozen_view(table_view: *mut QTableView) -> *mut QTableView; }
pub fn new_tableview_frozen_safe(parent: &Ptr<QWidget>) -> (QBox<QTableView>, QBox<QTableView>) {
    let table_view_normal = unsafe { new_tableview_frozen(parent.as_mut_raw_ptr()) };
    let table_view_frozen = unsafe { get_frozen_view(table_view_normal) };
    unsafe { (QBox::from_raw(table_view_normal), QBox::from_raw(table_view_frozen)) }
}

/// This function allows you to load data to a table capable of freezing columns.
extern "C" { fn set_data_model(table: *mut QTableView, model: *mut QAbstractItemModel); }
pub fn set_frozen_data_model_safe(table: &Ptr<QTableView>, model: &Ptr<QAbstractItemModel>) {
    unsafe { set_data_model(table.as_mut_raw_ptr(), model.as_mut_raw_ptr()) };
}

/// This function allows you to freeze/unfreeze a column.
extern "C" { fn toggle_freezer(table: *mut QTableView, column: i32); }
pub fn toggle_freezer_safe(table: &QBox<QTableView>, column: i32) {
    unsafe { toggle_freezer(table.as_mut_raw_ptr(), column) };
}

//---------------------------------------------------------------------------//
// KTextEditor stuff.
//---------------------------------------------------------------------------//

/// This function allow us to create a complete KTextEditor.
extern "C" { fn new_text_editor(parent: *mut QWidget) -> *mut QWidget; }
pub fn new_text_editor_safe(parent: &QBox<QWidget>) -> QBox<QWidget> {
    unsafe { QBox::from_raw(new_text_editor(parent.as_mut_raw_ptr())) }
}

/// This function allow us to get the text from the provided KTextEditor.
extern "C" { fn get_text(document: *mut QWidget) -> *mut QString; }
pub fn get_text_safe(document: &QBox<QWidget>) -> Ptr<QString> {
    unsafe { Ptr::from_raw(get_text(document.as_mut_raw_ptr())) }
}

/// This function allow us to set the text of  the provided KTextEditor.
extern "C" { fn set_text(document: *mut QWidget, string: *mut QString, highlighting_mode: *mut QString); }
pub fn set_text_safe(document: &QBox<QWidget>, string: &Ptr<QString>, highlighting_mode: &Ptr<QString>) {
    unsafe { set_text(document.as_mut_raw_ptr(), string.as_mut_raw_ptr(), highlighting_mode.as_mut_raw_ptr()) }
}

/// This function triggers the config dialog for the KTextEditor.
extern "C" { fn open_text_editor_config(parent: *mut QWidget); }
pub fn open_text_editor_config_safe(parent: &Ptr<QWidget>) {
    unsafe { open_text_editor_config(parent.as_mut_raw_ptr()) }
}

//---------------------------------------------------------------------------//
// Image stuff.
//---------------------------------------------------------------------------//

/// This function allow us to create a QLabel whose QPixmap gets resized with the resize events of the label.
extern "C" { fn new_resizable_label(parent: *mut QWidget, pixmap: *mut QPixmap) -> *mut QLabel; }
pub fn new_resizable_label_safe(parent: &Ptr<QWidget>, pixmap: &Ptr<QPixmap>) -> QPtr<QLabel> {
    unsafe { QPtr::from_raw(new_resizable_label(parent.as_mut_raw_ptr(), pixmap.as_mut_raw_ptr())) }
}

extern "C" { fn set_pixmap_on_resizable_label(label: *mut QLabel, pixmap: *mut QPixmap); }
pub fn set_pixmap_on_resizable_label_safe(label: &Ptr<QLabel>, pixmap: &Ptr<QPixmap>) {
    unsafe { set_pixmap_on_resizable_label(label.as_mut_raw_ptr(), pixmap.as_mut_raw_ptr()); }
}


//---------------------------------------------------------------------------//
// Special functions.
//---------------------------------------------------------------------------//

/// This function allow us to create a dialog when trying to close the main window.
pub extern fn are_you_sure(main_window: *mut QMainWindow, is_delete_my_mod: bool) -> bool {
    let title = qtr("rpfm_title");
    let message = if is_delete_my_mod { qtr("delete_mymod_0") }
    else if UI_STATE.get_is_modified() { qtr("delete_mymod_1") }

    // In any other situation... just return true and forget about the dialog.
    else { return true };

    // Create the dialog and run it (Yes => 3, No => 4).
    unsafe { QMessageBox::from_2_q_string_icon3_int_q_widget(
        &title,
        &message,
        q_message_box::Icon::Warning,
        65536, // No
        16384, // Yes
        1, // By default, select yes.
        main_window,
    ).exec() == 3 }
}
