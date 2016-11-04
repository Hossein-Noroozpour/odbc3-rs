extern crate gtk;
extern crate chrono;

pub mod odbc;

use gtk::prelude::*;

struct AppData {
    window: gtk::Window,
    calendar_button: gtk::Button,
    drivers_combo: gtk::ComboBoxText,
    servers_combo: gtk::ComboBoxText,
    databases_combo: gtk::ComboBoxText,
    usernames_combo: gtk::ComboBoxText,
    file_label: gtk::Label,
    env: odbc::Environment,
    date_start_year: u16,
    date_start_month: u8,
    date_start_day: u8,
    date_end_year: u16,
    date_end_month: u8,
    date_end_day: u8,
    servers: Vec<String>,
    databases: Vec<String>,
    usernames: Vec<String>,
    file_address: String,
    query: String,
}

fn main() {
    if gtk::init().is_err() {
        panic!("Gtk didn't started successfully");
    }

    let now = chrono::Local::now();

    let mut env = odbc::Environment::new().expect("Error in getting environment!");

    let b_calendar = gtk::Button::new_with_label("From: 0000-00-00 to: Now");

    let header = gtk::HeaderBar::new();
    header.set_title(Some("DB exporter"));
    header.set_subtitle(Some("Hossein Noroozpour"));
    header.set_show_close_button(true);
    header.pack_end(&b_calendar);

    let l_driver = gtk::Label::new(Some("Driver: "));
    l_driver.set_halign(gtk::Align::Start);

    let c_drivers = gtk::ComboBoxText::new();

    let b_driver_refresh = gtk::Button::new_with_label("Refresh drivers list");

    let l_server = gtk::Label::new(Some("Server address: "));
    l_server.set_halign(gtk::Align::Start);

    let c_servers = gtk::ComboBoxText::new();

    let b_manage_servers = gtk::Button::new_with_label("Manage servers");

    let l_database = gtk::Label::new(Some("Database name: "));
    l_database.set_halign(gtk::Align::Start);

    let c_databases = gtk::ComboBoxText::new();

    let b_manage_databases = gtk::Button::new_with_label("Manage databases");

    let l_username = gtk::Label::new(Some("Username: "));
    l_username.set_halign(gtk::Align::Start);

    let c_usernames = gtk::ComboBoxText::new();

    let b_manage_usernames = gtk::Button::new_with_label("Manage usernames");

    let l_file_address = gtk::Label::new(Some("File address: "));
    l_file_address.set_halign(gtk::Align::Start);

    let l_file = gtk::Label::new(None);

    let b_choose_file = gtk::Button::new_with_label("Choose export file");

    let b_set_query = gtk::Button::new_with_label("Set query");
    let b_set_equivalents = gtk::Button::new_with_label("Set equivalents");
    let b_import_settings = gtk::Button::new_with_label("Import settings");
    let b_export_settings = gtk::Button::new_with_label("Export settings");
    let b_execute = gtk::Button::new_with_label("Execute");

    let grid = gtk::Grid::new();
    grid.set_row_spacing(5);
    grid.set_column_spacing(5);
    grid.set_border_width(5);
    grid.attach(&l_driver, 0, 0, 1, 1);
    grid.attach(&c_drivers, 1, 0, 3, 1);
    grid.attach(&b_driver_refresh, 4, 0, 1, 1);
    grid.attach(&l_server, 0, 1, 1, 1);
    grid.attach(&c_servers, 1, 1, 3, 1);
    grid.attach(&b_manage_servers, 4, 1, 1, 1);
    grid.attach(&l_database, 0, 2, 1, 1);
    grid.attach(&c_databases, 1, 2, 3, 1);
    grid.attach(&b_manage_databases, 4, 2, 1, 1);
    grid.attach(&l_username, 0, 3, 1, 1);
    grid.attach(&c_usernames, 1, 3, 3, 1);
    grid.attach(&b_manage_usernames, 4, 3, 1, 1);
    grid.attach(&l_file_address, 0, 4, 1, 1);
    grid.attach(&l_file, 1, 4, 3, 1);
    grid.attach(&b_choose_file, 4, 4, 1, 1);
    grid.attach(&b_set_query, 0, 5, 1, 1);
    grid.attach(&b_set_equivalents, 1, 5, 1, 1);
    grid.attach(&b_import_settings, 2, 5, 1, 1);
    grid.attach(&b_export_settings, 3, 5, 1, 1);
    grid.attach(&b_execute, 4, 5, 1, 1);

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("DB Exporter");
    window.connect_delete_event(|_, _|{
        gtk::main_quit();
        Inhibit(false)
    });
    window.add(&grid);
    window.set_position(gtk::WindowPosition::Center);
    window.set_resizable(false);
    window.set_titlebar(Some(&header));
    window.show_all();

    use chrono::Datelike;
    let data = std::sync::Arc::new(
        std::sync::Mutex::new(
            AppData {
                window: window,
                calendar_button: b_calendar,
                drivers_combo: c_drivers,
                servers_combo: c_servers,
                databases_combo: c_databases,
                usernames_combo: c_usernames,
                file_label: l_file,
                env: env,
                date_start_year: 0,
                date_start_month: 0,
                date_start_day: 0,
                date_end_year: now.year() as u16,
                date_end_month: now.month() as u8,
                date_end_day: now.day() as u8,
                servers: Vec::new(),
                databases: Vec::new(),
                usernames: Vec::new(),
                file_address: String::new(),
                query: String::new(),
            }
        )
    );
    let cloned_data = data.clone();
    b_driver_refresh.connect_clicked(move |_| {
        let mut data = cloned_data.lock().unwrap();
        data.drivers_combo.remove_all();
        let drivers_info = data.env.get_drivers_info().expect("Error in getting driver info!");
        for driver in drivers_info {
            let s = driver.to_string();
            data.drivers_combo.append_text(&s);
        }
        data.drivers_combo.set_active(0);
        //        let sql_server = env.get_sql_server().unwrap();
    });
    let cloned_data = data.clone();
    data.lock().unwrap().calendar_button.connect_clicked(move | button | {
        let mut data = cloned_data.lock().unwrap();

        let l_from = gtk::Label::new(Some("From"));

        let cal_from = gtk::Calendar::new();

        let l_till = gtk::Label::new(Some("Till"));

        let cal_till = gtk::Calendar::new();

        let b_cancel = gtk::Button::new_with_label("Cancel");

        let b_save = gtk::Button::new_with_label("Save");

        let grid = gtk::Grid::new();
        grid.set_row_spacing(5);
        grid.set_column_spacing(5);
        grid.set_border_width(5);
        grid.attach(&l_from, 0, 0, 1, 1);
        grid.attach(&cal_from, 0, 1, 1, 1);
        grid.attach(&l_till, 1, 0, 1, 1);
        grid.attach(&cal_till, 1, 1, 1, 1);
        grid.attach(&b_cancel, 0, 2, 1, 1);
        grid.attach(&b_save, 1, 2, 1, 1);

        let dialog = gtk::Window::new(gtk::WindowType::Toplevel);
        dialog.set_title("Set from and till dates");
        dialog.set_position(gtk::WindowPosition::Center);
        dialog.set_modal(true);
        dialog.add(&grid);
        dialog.set_resizable(false);
        dialog.show_all();
        data.window.set_sensitive(false);

        struct DialogData {
            data: std::sync::Arc<std::sync::Mutex<AppData>>,
            dialog: gtk::Window,
            from: gtk::Calendar,
            till: gtk::Calendar,
        }

        let data = std::sync::Arc::new(std::sync::Mutex::new(
            DialogData {
                data: cloned_data.clone(),
                dialog: dialog,
                from: cal_from,
                till: cal_till,
            }
        ));

        let cloned_data = data.clone();
        b_cancel.connect_clicked(move |_| {
            let mut data = cloned_data.lock().unwrap();
            let mut data2 = data.data.lock().unwrap();
            data2.window.set_sensitive(true);
            data.dialog.destroy();
        });

        let cloned_data = data.clone();
        b_save.connect_clicked(move |_| {
            let mut data = cloned_data.lock().unwrap();
            let mut data2 = data.data.lock().unwrap();
            let (year, month, day) = data.from.get_date();
            data2.date_start_year = year as u16;
            data2.date_start_month = month as u8;
            data2.date_start_day = day as u8;
            let (year, month, day) = data.till.get_date();
            data2.date_end_year = year as u16;
            data2.date_end_month = month as u8;
            data2.date_end_day = day as u8;
            data2.window.set_sensitive(true);
            data2.calendar_button.set_label(
                &format!(
                    "From: {:04}-{:02}-{:02} to: {:04}-{:02}-{:02}",
                    data2.date_start_year, data2.date_start_month, data2.date_start_day,
                    data2.date_end_year, data2.date_end_month, data2.date_end_day
                ));
            data.dialog.destroy();
        });

        let cloned_data = data.clone();
        let data = data.lock().unwrap();
        data.dialog.connect_delete_event(move |_, _| {
            let mut data = cloned_data.lock().unwrap();
            let mut data2 = data.data.lock().unwrap();
            data2.window.set_sensitive(true);
            Inhibit(false)
        });
    });

    enum ListType {
        Servers,
        Databases,
        Usernames,
    }

    fn list_manager(cloned_data: &std::sync::Arc<std::sync::Mutex<AppData> >,
            list_type: ListType) {
        let mut data = cloned_data.lock().unwrap();
        data.window.set_sensitive(false);

        let list = gtk::ListBox::new();
        list.set_size_request(200, 200);
        list.set_hexpand(true);
        list.set_vexpand(true);

        let b_add = gtk::Button::new_with_label("Add");
        b_add.set_hexpand(false);
        b_add.set_vexpand(false);

        let b_edit = gtk::Button::new_with_label("Edit");
        b_edit.set_hexpand(false);
        b_edit.set_vexpand(false);

        let b_remove = gtk::Button::new_with_label("Remove");
        b_remove.set_hexpand(false);
        b_remove.set_vexpand(false);

        let g_buttons = gtk::Grid::new();
        g_buttons.attach(&b_add, 0, 0, 1, 1);
        g_buttons.attach(&b_edit, 0, 1, 1, 1);
        g_buttons.attach(&b_remove, 0, 2, 1, 1);
        g_buttons.set_hexpand(false);
        g_buttons.set_vexpand(false);

        let grid = gtk::Grid::new();
        grid.set_row_spacing(5);
        grid.set_column_spacing(5);
        grid.set_border_width(5);
        grid.attach(&list, 0, 0, 1, 4);
        grid.attach(&g_buttons, 1, 0, 1, 1);

        let dialog = gtk::Window::new(gtk::WindowType::Toplevel);
        dialog.set_title( match list_type {
            ListType::Servers => "Manage servers",
            ListType::Databases => "Manage databases",
            ListType::Usernames => "Manage usernames",
        });
        dialog.set_position(gtk::WindowPosition::Center);
        dialog.set_modal(true);
        dialog.add(&grid);
        dialog.show_all();

        fn refresh_list(items: &Vec<String>, list: &gtk::ListBox) {
            loop {
                match list.get_row_at_index(0) {
                    Some(w) => list.remove(&w),
                    None => break,
                }
            }
            for item in items {
                let w = gtk::Label::new(Some(&item));
                w.set_halign(gtk::Align::Start);
                let r = gtk::ListBoxRow::new();
                r.add(&w);
                list.prepend(&r);
            }
            list.show_all();
        };

        refresh_list(match list_type {
            ListType::Servers => &data.servers,
            ListType::Databases => &data.databases,
            ListType::Usernames => &data.usernames,
        }, &list);

        struct DialogData {
            data: std::sync::Arc<std::sync::Mutex<AppData>>,
            dialog: gtk::Window,
            list: gtk::ListBox,
            list_type: ListType,
        }

        let data = std::sync::Arc::new(std::sync::Mutex::new(DialogData {
            data: cloned_data.clone(),
            dialog: dialog,
            list: list,
            list_type: list_type,
        }));

        let cloned_data = data.clone();
        data.lock().unwrap().dialog.connect_delete_event(move |_, _|{
            let data = cloned_data.lock().unwrap();
            let mut data2 = data.data.lock().unwrap();
            let combo = match data.list_type {
                ListType::Servers => &data2.servers_combo,
                ListType::Databases => &data2.databases_combo,
                ListType::Usernames => &data2.usernames_combo,
            };
            combo.remove_all();
            let mut items = match data.list_type {
                ListType::Servers => &data2.servers,
                ListType::Databases => &data2.databases,
                ListType::Usernames => &data2.usernames,
            };
            for item in items {
                combo.append_text(&item);
            }
            combo.set_active(0);
            data2.window.set_sensitive(true);
            Inhibit(false)
        });

        fn set_dialog(action_name: &String, previous_value: &String) -> String {
            let entry = gtk::Entry::new();
            entry.set_text(previous_value.as_str());
            entry.set_hexpand(true);
            entry.set_vexpand(false);

            let button = gtk::Button::new_with_label(action_name.as_str());
            button.set_hexpand(false);
            button.set_vexpand(false);
            button.set_valign(gtk::Align::Center);
            button.set_halign(gtk::Align::Center);

            let grid = gtk::Grid::new();
            grid.set_row_spacing(5);
            grid.set_column_spacing(5);
            grid.set_border_width(5);
            grid.attach(&entry, 0, 0, 1, 1);
            grid.attach(&button, 1, 0, 1, 1);
            grid.set_hexpand(true);
            grid.set_vexpand(false);

            let dialog = gtk::Window::new(gtk::WindowType::Toplevel);
            dialog.set_title(&format!("Try to {}", action_name));
            dialog.set_position(gtk::WindowPosition::Center);
            dialog.set_modal(true);
            dialog.set_vexpand(false);
            dialog.add(&grid);
            dialog.show_all();

            dialog.connect_delete_event(|_, _| {
                gtk::main_quit();
                Inhibit(false)
            });
            let value = std::sync::Arc::new(std::sync::Mutex::new((*previous_value).clone()));
            let col_val = value.clone();
            button.connect_clicked(move |_| {
                let mut value = col_val.lock().unwrap();
                *value = match entry.get_text() {
                    Some(s) => s.to_string(),
                    None => "".to_string(),
                };
                if value.len() == 0 {
                    let dialog = gtk::MessageDialog::new(
                        Some(&dialog),
                        gtk::DIALOG_MODAL,
                        gtk::MessageType::Error,
                        gtk::ButtonsType::Close,
                        "Please check your input befor submit!"
                    );
                    dialog.run();
                    dialog.destroy();
                } else {
                    dialog.destroy();
                    gtk::main_quit();
                }
            });

            gtk::main();

            return (*value.lock().unwrap()).clone();
        }

        let cloned_data = data.clone();
        b_add.connect_clicked(move |_| {
            let mut data = cloned_data.lock().unwrap();
            let mut data2 = data.data.lock().unwrap();
            let mut e: String = "".to_string();
            let a: String = "Add".to_string();
            let mut items = match data.list_type {
                ListType::Servers => &mut data2.servers,
                ListType::Databases => &mut data2.databases,
                ListType::Usernames => &mut data2.usernames,
            };
            let s = set_dialog(&a, &e);
            let s = s.trim();
            if s.len() != 0 {
                items.push(s.to_string());
                refresh_list(&items, &data.list);
            }
        });

        fn list_get_index(parent: &gtk::Window, list: &gtk::ListBox, items: &Vec<String>)
                -> Option<usize> {
            let index = match list.get_selected_row() {
                Some(r) => r.get_index(),
                None => {
                    let dialog = gtk::MessageDialog::new(
                        Some(parent),
                        gtk::DIALOG_MODAL,
                        gtk::MessageType::Error,
                        gtk::ButtonsType::Close,
                        "Please select an existing item from list!"
                    );
                    dialog.run();
                    dialog.destroy();
                    return None;
                }
            } + 1;
            let index = (items.len() as i32 - index) as usize;
            return Some(index);
        }

        let cloned_data = data.clone();
        b_edit.connect_clicked(move |_| {
            let mut data = cloned_data.lock().unwrap();
            let mut data2 = data.data.lock().unwrap();
            let mut items = match data.list_type {
                ListType::Servers => &mut data2.servers,
                ListType::Databases => &mut data2.databases,
                ListType::Usernames => &mut data2.usernames,
            };
            let index = match list_get_index(&data.dialog, &data.list, &items) {
                Some(i) => i,
                None => return,
            };
            let mut e: String = items[index].clone();
            let a: String = "Edit".to_string();
            items[index] = set_dialog(&a, &e);
            refresh_list(&items, &data.list);
        });

        let cloned_data = data.clone();
        b_remove.connect_clicked(move |_| {
            let mut data = cloned_data.lock().unwrap();
            let mut data2 = data.data.lock().unwrap();
            let mut items = match data.list_type {
                ListType::Servers => &mut data2.servers,
                ListType::Databases => &mut data2.databases,
                ListType::Usernames => &mut data2.usernames,
            };
            let index = match list_get_index(&data.dialog, &data.list, &items) {
                Some(i) => i,
                None => return,
            };
            items.remove(index);
            refresh_list(&items, &data.list);
        });
    }

    let cloned_data = data.clone();
    b_manage_servers.connect_clicked(move |_| {
        list_manager(&cloned_data, ListType::Servers);
    });

    let cloned_data = data.clone();
    b_manage_databases.connect_clicked(move |_| {
        list_manager(&cloned_data, ListType::Databases);
    });

    let cloned_data = data.clone();
    b_manage_usernames.connect_clicked(move |_| {
        list_manager(&cloned_data, ListType::Usernames);
    });

    let cloned_data = data.clone();
    b_choose_file.connect_clicked(move |_| {
        let mut data = cloned_data.lock().unwrap();
        let dialog = gtk::FileChooserDialog::new(
            Some("Export a XML file"), Some(&data.window), gtk::FileChooserAction::Save);
        dialog.add_buttons(&[
            ("Save", gtk::ResponseType::Ok.into()),
            ("Cancel", gtk::ResponseType::Cancel.into())
        ]);
        let filter = gtk::FileFilter::new();
        filter.add_pattern("*.xml");

        dialog.set_filter(&filter);

        dialog.set_select_multiple(false);
        if dialog.run() == gtk::ResponseType::Ok.into() {
            let p_file = dialog.get_filename().expect("Unexpected behavior!");
            let str_file = p_file.to_str().expect("Unexpected behavior!");
            let s_file = str_file.to_string();
            data.file_label.set_text(str_file);
            data.file_address = s_file;
        }
        dialog.destroy();
    });

    let cloned_data = data.clone();
    b_set_query.connect_clicked(move |_| {
        let entry = gtk::TextView::new();
        entry.set_hexpand(true);
        entry.set_vexpand(true);
        let cloned_data = cloned_data.clone();
        entry.get_buffer().unwrap().connect_changed(move |b| {
            let query = b.get_text(&b.get_start_iter(), &b.get_end_iter(), false).unwrap();
            cloned_data.lock().unwrap().query = query;
        });

        let grid = gtk::Grid::new();
        grid.set_row_spacing(5);
        grid.set_column_spacing(5);
        grid.set_border_width(5);
        grid.attach(&entry, 0, 0, 1, 1);
        grid.set_hexpand(true);
        grid.set_vexpand(true);

        let dialog = gtk::Window::new(gtk::WindowType::Toplevel);
        dialog.set_position(gtk::WindowPosition::Center);
        dialog.set_title("Write your query.");
        dialog.set_modal(true);
        dialog.set_size_request(400, 200);
        dialog.add(&grid);
        dialog.show_all();
        dialog.connect_delete_event(|_, _|{
            gtk::main_quit();
            Inhibit(false)
        });

        gtk::main();
    });

    let cloned_data = data.clone();
    b_execute.connect_clicked(move |_| {
        let mut data = cloned_data.lock().unwrap();
        let driver_index = data.drivers_combo.get_active() as usize;
        let drivers = data.env.get_drivers_info().expect("Error in getting driver info!");
        let driver = drivers[driver_index].get_name();
        let servers_index = data.servers_combo.get_active() as usize;
        let server = data.servers[servers_index].clone();
        let database_index = data.databases_combo.get_active() as usize;
        let database = data.databases[database_index].clone();
        let username_index = data.usernames_combo.get_active() as usize;
        let username = data.usernames[username_index].clone();
        let connection_string = format!("DRIVER={{{}}};SERVER={};DATABASE={};UID={};PWD={};", driver, server, database, username, "12345");
        /// Work sample: "DRIVER={SQL Server};SERVER=ITS-H-NOROUZPOU\\SQLEXPRESS;DATABASE=Eris;UID=hossein-noroozpour;PWD=12345;APP=RustDBExporter"
        let sql = odbc::Database::new(& mut data.env, &connection_string).unwrap();
        println!("{:?}", connection_string);
    });

    gtk::main();
}
