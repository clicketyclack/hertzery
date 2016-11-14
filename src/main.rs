extern crate piston_window;
extern crate sdl2_window;
extern crate portaudio;
extern crate time;



fn main() {
    hertzery::main();
}


mod hertzery {
    use std;
    use portaudio;
    use portaudio as pa;
    use time;
    
    use piston_window::{PistonWindow, WindowSettings, OpenGL};
    use sdl2_window::Sdl2Window;

    pub fn main() {
        run().unwrap()
    }

    struct ExtractedInfo<'a> {
        def_input: portaudio::DeviceIndex,
        def_output: portaudio::DeviceIndex,
        input_latency: f64,
        output_latency: f64,
        pa: &'a portaudio::PortAudio,
    }

    impl<'a> ExtractedInfo<'a> {
        fn new(pa: &portaudio::PortAudio) -> ExtractedInfo {
            let def_input = pa.default_input_device().unwrap();
            let def_output = pa.default_output_device().unwrap();

            let input_info = pa.device_info(def_input).unwrap();
            let output_info = pa.device_info(def_output).unwrap();

            let input_latency = input_info.default_low_input_latency;
            let output_latency = output_info.default_low_output_latency;

            ExtractedInfo {
                def_input: def_input,
                def_output: def_output,
                input_latency: input_latency,
                output_latency: output_latency,
                pa: &pa,
            }
        }
    }


    struct IOSettings<'a> {
        sample_rate: f64,
        pa_settings: portaudio::stream::DuplexSettings<f32, f32>,
        pa_handle: &'a portaudio::PortAudio,
        chunk_size: u32,
        is_interleaved: bool,
        input_device : portaudio::DeviceIndex,
        output_device : portaudio::DeviceIndex,
    }

    impl<'a> IOSettings<'a> {
        fn quickstart(pa: &portaudio::PortAudio) -> IOSettings {

            IOSettings::print_devices(pa);

            let sample_rate = 48000.0;
            let chunk_size = 2048; // Gives ~ 43ms chunks
            let is_interleaved = true;
            let einfo = ExtractedInfo::new(&pa);
            let input_device = einfo.def_input;
            let output_device = einfo.def_output;

            let input_params =
                pa::StreamParameters::<f32>::new(input_device, 1, is_interleaved, einfo.input_latency);
            let output_params =
                pa::StreamParameters::new(output_device, 2, is_interleaved, einfo.output_latency);
                
            let settings =
                pa::DuplexStreamSettings::new(input_params, output_params, sample_rate, chunk_size);

            IOSettings {
                sample_rate: sample_rate,
                pa_settings: settings,
                pa_handle: &pa,
                chunk_size : chunk_size,
                is_interleaved : is_interleaved,
                input_device : input_device,
                output_device : output_device,
            }
        }
        
        fn print_devices(pa: &portaudio::PortAudio) -> () {
            if let Ok(mut iter) = pa.devices() {
                while let Some(d) = iter.next() {
                    let dinfo = d.unwrap();
                    let idx = dinfo.0;
                    let name = dinfo.1.name;
                    println!("Found device {:?}, {}", idx, name);
                    
                }
            }
        }
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
