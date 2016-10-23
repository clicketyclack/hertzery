extern crate piston_window;
extern crate portaudio;

use piston_window::*;

fn main() {

    let mut pa = portaudio::PortAudio::new().unwrap();
    let title = format!("Portaudio version {:?}", pa.version_text());
    let mut window: PistonWindow = WindowSettings::new(title, [512; 2]).build().unwrap();


    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0], // rectangle
                      c.transform,
                      g);
        });
    }
}
