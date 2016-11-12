extern crate piston_window;
extern crate sdl2_window;
extern crate portaudio;




fn main() {
    hertzery::main();
}


mod hertzery {
    use std;
    use portaudio;
    use portaudio as pa;
    
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
        pa_settings: portaudio::stream::DuplexSettings<f32, i32>,
        pa_handle: &'a portaudio::PortAudio,
    }

    impl<'a> IOSettings<'a> {
        fn quickstart(pa: &portaudio::PortAudio) -> IOSettings {

            let sample_rate = 48000.0;
            let chunk_size = 2048; // Gives ~ 43ms chunks

            let einfo = ExtractedInfo::new(&pa);

            let input_params =
                pa::StreamParameters::<f32>::new(einfo.def_input, 1, true, einfo.input_latency);
            let output_params =
                pa::StreamParameters::new(einfo.def_output, 2, true, einfo.output_latency);
                
            let settings =
                pa::DuplexStreamSettings::new(input_params, output_params, sample_rate, chunk_size);

            IOSettings {
                sample_rate: sample_rate,
                pa_settings: settings,
                pa_handle: &pa,
            }
        }
    }



    struct RecordingPath<'a> {
        settings: IOSettings<'a>,
        stream: portaudio::Stream<portaudio::NonBlocking, portaudio::Duplex<f32, i32>>,
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
             -> portaudio::Stream<portaudio::NonBlocking, portaudio::Duplex<f32, i32>> {
            let callback = move |pa::DuplexStreamCallbackArgs { in_buffer,
                                                                out_buffer,
                                                                frames,
                                                                flags,
                                                                time }| {


                let mut volume = 0;

                for n in 0..frames {
                    volume += (in_buffer[n] as i32).abs();
                    out_buffer[n] = n as i32; // in_buffer[n] as i32;

                }

                println!("Got {} volume units over {} frames.", volume, frames);

                pa::Continue
            };

            let stream = pa.open_non_blocking_stream(settings.pa_settings, callback).unwrap();
            stream
        }
    }

    fn run() -> Result<(), pa::Error> {


        use portaudio;
        use piston_window::{clear, rectangle, OpenGL};

        let pa = portaudio::PortAudio::new().unwrap();
        let mut rp = RecordingPath::new(&pa);
        
        //let in_dev_nr = rp.settings.pa_settings.in_params.device;
        let device_info = pa.device_info(pa::DeviceIndex(0)).unwrap();
        //print!("{}", device_info.name);

        try!(rp.stream.start());

        //let title = format!("Portaudio version {:?}", pa.version_text());
        let title = format!("{}", device_info.name);
        let mut window: PistonWindow<Sdl2Window> = WindowSettings::new(title, (640, 480))
                                                        .exit_on_esc(true)
                                                        .build()
                                                        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
        
        while let Some(e) = window.next() {
                        window.draw_2d(&e, |c, g| {
                            clear([0.5, 0.5, 0.5, 1.0], g);
                            rectangle([1.0, 0.0, 0.0, 1.0], // red
                                      [0.0, 0.0, 100.0, 100.0], // rectangle
                                      c.transform,
                                      g);
                    });
        }
    
        
        // While stream is running, idle.
        while let true = try!(rp.stream.is_active()) {
            std::thread::sleep(std::time::Duration::from_millis(16));
            
        }
        
        try!(rp.stream.stop());

        Ok(())

    }
}
