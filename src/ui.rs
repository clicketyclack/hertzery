



pub fn start_ui() {
    extern crate gtk;
    use std::fs::OpenOptions;
    use gtk;
    use gtk::prelude::*;
    use gtk::{Builder, Button, MessageDialog, Window};

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }


    let f = OpenOptions::new().read(true).open("builder_basics.glade", "r");
    let glade_src = f.read();
    f.close();

    // let glade_src = include_str!("builder_basics.glade");
    let builder = Builder::new_from_string(glade_src);

    let window: Window = builder.get_object("window1").unwrap();
    let bigbutton: Button = builder.get_object("button1").unwrap();
    let dialog: MessageDialog = builder.get_object("messagedialog1").unwrap();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    bigbutton.connect_clicked(move |_| {
        dialog.run();
        dialog.hide();
    });

    window.show_all();

    gtk::main();
}
