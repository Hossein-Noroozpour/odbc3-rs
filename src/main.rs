extern crate gtk;

pub mod odbc;

use gtk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        panic!("Gtk didn't started successfully");
    }
    let mut env = odbc::Environment::new().expect("Error in getting environment!");
    let drivers_info = env.get_drivers_info().expect("Error in getting driver info!");

    let l_driver = gtk::Label::new(Some("Driver: "));
    l_driver.set_halign(gtk::Align::Start);

    let combo = gtk::ComboBoxText::new();
    for driver in drivers_info {
        let s = driver.to_string();
        combo.append_text(&s);
    }
    combo.set_active(0);
    let sql_server = env.get_sql_server().unwrap();

    let b_driver_refresh = gtk::Button::new_with_label("Refresh list");
    b_driver_refresh.connect_clicked(|_| {

    });

    let grid = gtk::Grid::new();
    grid.set_row_spacing(5);
    grid.set_column_spacing(5);
    grid.set_border_width(5);
    grid.attach(&l_driver, 0, 0, 1, 1);
    grid.attach(&combo, 1, 0, 1, 1);
    grid.attach(&b_driver_refresh, 2, 0, 1, 1);

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Diesel SQL Server Database Driver Test");
    window.connect_delete_event(|_, _|{
        gtk::main_quit();
        gtk::Inhibit(false)
    });
    window.add(&grid);
    window.set_resizable(false);
    window.show_all();

    gtk::main();
}
