extern crate gtk;
extern crate chrono;

pub mod odbc;

use gtk::prelude::*;

struct AppData {
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
    window.set_resizable(false);
    window.set_titlebar(Some(&header));
    window.show_all();
    use chrono::Datelike;
    let data = std::sync::Arc::new(
        std::sync::Mutex::new(
            AppData {
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
    b_calendar.connect_clicked(move | button | {
        let mut data = cloned_data.lock().unwrap();

    });
    gtk::main();
}
