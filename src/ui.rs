

extern crate gtk;
extern crate gdk;


pub fn start_ui() {
    
    use std::fs::OpenOptions;
    use self::gtk::prelude::*;
    use self::gtk;
    use self::gdk;
    use self::gtk::{Builder, Button, MessageDialog, Window};

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }


    let glade_src = include_str!("hertzery.glade");
    let builder = Builder::new_from_string(glade_src);

    let hz_window_main: Option<gtk::Window> = builder.get_object("hz_window_main");
    let top_window = match hz_window_main {  
        Some(window) =>  window, 
        None => {
        println!("Could not get 'hz_window_main' from glade builder.");
        return;
        }
    };

    top_window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    
    top_window.show_all();

    gtk::main();
}
