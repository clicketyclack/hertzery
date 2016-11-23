

extern crate gtk;
extern crate gdk;
extern crate cairo;
use program::ReportGlobals;

pub fn start_ui<'a>(report_globals : &'a ReportGlobals) {
    
    use std::fs::OpenOptions;
    
    use std::{thread, time};

    use self::gtk::prelude::*;
    use self::gtk;
    use self::gdk;
    use self::cairo;    
    use self::gtk::{Builder, Button, MessageDialog, Window, DrawingArea};

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
    
    let drawing_area: gtk::DrawingArea = builder.get_object("hz_tuner_drawing").unwrap();
    let rg = report_globals.current_volume.clone();

    drawing_area.connect_draw(move |widget, cairo_context| {
            { // Scope because we don't want to keep the lock during the sleep.

                let mut data = rg.lock().unwrap();
                let radius = (*data).abs() as f64;
                println!("Drawing with radius {}!", radius);
                cairo_context.set_source_rgb(0.9,0.4,0.1);
                cairo_context.rectangle(20.3,20.3,155.5,155.5);
                cairo_context.fill();
    
                cairo_context.set_source_rgb(0.3,0.3,0.8);
                cairo_context.arc(120.1,70.6,radius, 0.1, 10.9);
                cairo_context.fill();
            }
            
            DrawingArea::queue_draw_area(widget,0,2,345,377);
            //gdk::Window::invalidate_rect(widget,0,0);
            
            // TODO: Seperate timer here.
            let ten_millis = time::Duration::from_millis(10);
            thread::sleep(ten_millis);
        
            Inhibit(false)
        });

    top_window.set_default_size(800,600);    

    top_window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    
    top_window.show_all();

    gtk::main();
}
