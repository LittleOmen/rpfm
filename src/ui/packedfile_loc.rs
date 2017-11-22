// In this file are all the helper functions used by the UI when decoding Loc files.
extern crate gtk;

use gtk::prelude::*;
use gtk::{
    TreeView, TreeSelection, ListStore, ScrolledWindow,
    CellRendererText, TreeViewColumn, CellRendererToggle
};

use ::packedfile::loc::LocData;
use ::packedfile::loc::LocDataEntry;

// Struct PackedFileLocTreeView: contains all the stuff we need to give to the program to show a
// TreeView with the data of a Loc file, allowing us to manipulate it.
#[derive(Clone)]
pub struct PackedFileLocTreeView {
    pub packed_file_tree_view: TreeView,
    pub packed_file_list_store: ListStore,
    pub packed_file_tree_view_selection: TreeSelection,
    pub packed_file_tree_view_cell_key: CellRendererText,
    pub packed_file_tree_view_cell_text: CellRendererText,
    pub packed_file_tree_view_cell_tooltip: CellRendererToggle,
}

// Implementation of "PackedFileLocTreeView".
impl PackedFileLocTreeView{

    // This function creates a new TreeView with "packed_file_data_display" as father and returns a
    // PackedFileLocTreeView with all his data.
    pub fn create_tree_view(packed_file_data_display: &ScrolledWindow) -> PackedFileLocTreeView {

        // First, we create the new ListStore, the new TreeView, and prepare the TreeView to display the data
        let packed_file_tree_view = TreeView::new();
        let packed_file_tree_view_selection = packed_file_tree_view.get_selection();
        let packed_file_list_store = ListStore::new(&[String::static_type(), String::static_type(), String::static_type(), gtk::Type::Bool]);

        packed_file_tree_view.set_model(Some(&packed_file_list_store));
        packed_file_tree_view.set_grid_lines(gtk::TreeViewGridLines::Both);

        let cell_index = CellRendererText::new();
        let cell_key = CellRendererText::new();
        let cell_text = CellRendererText::new();
        let cell_tooltip = CellRendererToggle::new();

        cell_key.set_property_editable(true);
        cell_text.set_property_editable(true);
        cell_tooltip.set_activatable(true);

        let column_index = TreeViewColumn::new();
        let column_key = TreeViewColumn::new();
        let column_text = TreeViewColumn::new();
        let column_tooltip = TreeViewColumn::new();

        column_index.set_title("Index");
        column_key.set_title("Key");
        column_text.set_title("Text");
        column_tooltip.set_title("Tooltip");

        column_index.set_clickable(true);
        column_key.set_clickable(true);
        column_text.set_clickable(true);
        column_tooltip.set_clickable(true);

        column_key.set_resizable(true);
        column_text.set_resizable(true);

        column_index.set_max_width(60);
        column_key.set_min_width(50);
        column_text.set_min_width(50);
        column_tooltip.set_min_width(50);

        column_key.set_fixed_width(500);
        column_text.set_fixed_width(500);
        column_tooltip.set_fixed_width(50);

        column_index.set_sort_column_id(0);
        column_key.set_sort_column_id(1);
        column_text.set_sort_column_id(2);
        column_tooltip.set_sort_column_id(3);

        column_index.pack_start(&cell_index, true);
        column_key.pack_start(&cell_key, true);
        column_text.pack_start(&cell_text, true);
        column_tooltip.pack_start(&cell_tooltip, true);

        column_index.add_attribute(&cell_index, "text", 0);
        column_key.add_attribute(&cell_key, "text", 1);
        column_text.add_attribute(&cell_text, "text", 2);
        column_tooltip.add_attribute(&cell_tooltip, "active", 3);

        packed_file_tree_view.append_column(&column_index);
        packed_file_tree_view.append_column(&column_key);
        packed_file_tree_view.append_column(&column_text);
        packed_file_tree_view.append_column(&column_tooltip);

        packed_file_tree_view.set_enable_search(false);

        packed_file_data_display.add(&packed_file_tree_view);
        packed_file_data_display.show_all();

        PackedFileLocTreeView {
            packed_file_tree_view,
            packed_file_list_store,
            packed_file_tree_view_selection,
            packed_file_tree_view_cell_key: cell_key,
            packed_file_tree_view_cell_text: cell_text,
            packed_file_tree_view_cell_tooltip: cell_tooltip,
        }
    }

    // This function loads the data from a LocData into a TreeView.
    pub fn load_data_to_tree_view(
        packed_file_data: &LocData,
        packed_file_list_store: &ListStore
    ) {
        // First, we delete all the data from the ListStore.
        packed_file_list_store.clear();

        // Then we add every line to the ListStore.
        let mut j = 0;
        for i in &packed_file_data.packed_file_data_entries {
            j += 1;
            packed_file_list_store.insert_with_values(None, &[0, 1, 2, 3], &[&j.to_string(), &i.key, &i.text, &i.tooltip]);
        }
    }

    // This function returns a Vec<LocDataEntry> with all the stuff in the table. We need for it the
    // ListStore, and it'll return a LocData with all the stuff from the table.
    pub fn return_data_from_tree_view(
        packed_file_list_store: &ListStore,
    ) -> LocData {

        let mut packed_file_data_from_tree_view = LocData::new();

        // Only in case we have any line in the ListStore we try to get it. Otherwise we return an
        // empty LocData.
        if let Some(current_line) = packed_file_list_store.get_iter_first() {
            let mut done = false;
            while !done {
                let key = packed_file_list_store.get_value(&current_line, 1).get().unwrap();
                let text = packed_file_list_store.get_value(&current_line, 2).get().unwrap();
                let tooltip = packed_file_list_store.get_value(&current_line, 3).get().unwrap();

                &packed_file_data_from_tree_view.packed_file_data_entries.push(LocDataEntry::new(key, text, tooltip));

                if !packed_file_list_store.iter_next(&current_line) {
                    done = true;
                }
            }
        }
        packed_file_data_from_tree_view
    }
}
