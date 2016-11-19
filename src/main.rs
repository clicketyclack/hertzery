/*! 
Hertzery. A mod.

*/


extern crate piston_window;
extern crate sdl2_window;
extern crate portaudio;
extern crate time;

pub mod dsp_waveforms;
pub mod audiodev_config;

fn main() {
    hertzery::main();
}


mod hertzery {
    use std;
    use portaudio;
    use portaudio as pa;
    use time;
    use audiodev_config::*;
    
    use piston_window::{PistonWindow, WindowSettings, OpenGL};
    use sdl2_window::Sdl2Window;

    
    pub fn main() {
        run().unwrap()
    }


    struct RecordingPath<'a> {
        settings: IOSettings<'a>,
        stream: portaudio::Stream<portaudio::NonBlocking, portaudio::Duplex<f32, f32>>,
    }



    impl<'a> RecordingPath<'a> {
        fn new(pa: &portaudio::PortAudio) -> RecordingPath {
            // Return a recording path with an input stream.
            let settings = IOSettings::quickstart(pa);

            let stream = RecordingPath::open_channel(&settings, &pa);
            RecordingPath {
                settings: settings,
                stream: stream,
            }

        }

        fn open_channel
            (settings: &IOSettings,
             pa: &portaudio::PortAudio)
             -> portaudio::Stream<portaudio::NonBlocking, portaudio::Duplex<f32, f32>> {
                 
            // Safeguard: stop stream after 3 seconds, as we don't have stopping control over it yet.      
            let deadline = time::get_time() + time::Duration::seconds(5);      
            let callback = move |pa::DuplexStreamCallbackArgs { in_buffer,
                                                                out_buffer,
                                                                frames,
                                                                flags,
                                                                time }| {

                let mut volume = 0.0f32;

                for n in 0..frames {
                    let duration = 2048.0 / 48000.0;
                    volume += (in_buffer[n] as f32).abs();
                    let f = 440.0;
                    let t = duration * ((n as f32) / 2048.0);
                    let amplitude = 0.1;
                    out_buffer[n] = amplitude * (f * t * 2.0 * std::f32::consts::PI).sin(); // in_buffer[n] as f32;
                }

                let time_left = deadline - time::get_time();
                println!("Got {} volume units over {} frames. Some frames are {}, {}. {} left.", volume, frames, in_buffer[4], in_buffer[6], time_left);

                if (time::get_time() < deadline) {
                    pa::Continue
                } else {
                    pa::Complete
                }
            };

            let stream = pa.open_non_blocking_stream(settings.pa_settings, callback).unwrap();
            stream
        }
    }



    fn run() -> Result<(), pa::Error> {


        use portaudio;
        use piston_window::{clear, rectangle, Window};

        let pa = portaudio::PortAudio::new().unwrap();
        let mut rp = RecordingPath::new(&pa);
        
        let device = rp.settings.input_device;
        let device_info = pa.device_info(device).unwrap();
        
        try!(rp.stream.start());

        let title = format!("{}", device_info.name);
        
        let mut window: PistonWindow<Sdl2Window> = WindowSettings::new(title, (640, 480))
                                                        .build()
                                                        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
        
            
        // While stream is running, idle.
        while let true = try!(rp.stream.is_active()) {
		
    		if let Some(e) = window.poll_event() {
                            window.draw_2d(&e, |c, g| {
                                clear([0.5, 0.5, 0.5, 1.0], g);
                                rectangle([1.0, 0.0, 0.0, 1.0], // red
                                          [0.0, 0.0, 100.0, 100.0], // rectangle
                                          c.transform,
                                          g);
                            });
                            window.swap_buffers();
                            
                            println!("Draw event!");
                            
    		}
    		
            std::thread::sleep(std::time::Duration::from_millis(16));
            
        }
        
        try!(rp.stream.stop());

        Ok(())

    }
}
