extern crate gtk;
extern crate chrono;

pub mod odbc;

use gtk::prelude::*;

struct AppData {
    window: gtk::Window,
    calendar_button: gtk::Button,
    drivers_combo: gtk::ComboBoxText,
    env: odbc::Environment,
    date_start_year: u16,
    date_start_month: u8,
    date_start_day: u8,
    date_end_year: u16,
    date_end_month: u8,
    date_end_day: u8,
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
                env: env,
                date_start_year: 0,
                date_start_month: 0,
                date_start_day: 0,
                date_end_year: now.year() as u16,
                date_end_month: now.month() as u8,
                date_end_day: now.day() as u8,
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

    let cloned_data = data.clone();
    b_manage_servers.connect_clicked(move |_| {
        let mut data = cloned_data.lock().unwrap();

        let list = gtk::ListBox::new();
        list.set_size_request(200, 400);

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

        let dialog = gtk::Window::new(gtk::WindowType::Toplevel);
        dialog.set_title("Manage servers");
        dialog.set_position(gtk::WindowPosition::Center);
        dialog.set_modal(true);
        dialog.add(&grid);
        dialog.set_resizable(false);
        dialog.show_all();
        data.window.set_sensitive(false);
    });
    gtk::main();
}
