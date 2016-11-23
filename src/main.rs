/*! 
Hertzery. A mod.

*/


extern crate portaudio;
extern crate time;

pub mod dsp_waveforms;
pub mod audiodev_config;
pub mod ui;
pub mod program;

fn main() {
    hertzery::main();
}


mod hertzery {
    use std;
    use portaudio;
    use portaudio as pa;
    use time;
    use audiodev_config::*;
    use dsp_waveforms;
    use ui;
    use program::ReportGlobals;
    
    pub fn main() {
        run().unwrap()
    }


    struct RecordingPath<'a> {
        settings: IOSettings<'a>,
        stream: portaudio::Stream<portaudio::NonBlocking, portaudio::Duplex<f32, f32>>,
    }



    impl<'a> RecordingPath<'a> {
        fn new(pa: &'a portaudio::PortAudio, report_globals : &'a ReportGlobals) -> RecordingPath<'a> {
            // Return a recording path with an input stream.
            let settings = IOSettings::quickstart(pa);

            let stream = RecordingPath::open_channel(&settings, &pa, report_globals);
            RecordingPath {
                settings: settings,
                stream: stream,
            }

        }

        fn open_channel
            (settings: &IOSettings,
                pa: &portaudio::PortAudio,
                report_globals : &ReportGlobals)
             -> portaudio::Stream<portaudio::NonBlocking, portaudio::Duplex<f32, f32>> {
                 
            // Safeguard: stop stream after 3 seconds, as we don't have stopping control over it yet.      
            let deadline = time::get_time() + time::Duration::seconds(6786575);
                  
            let wfc = dsp_waveforms::WaveFormCache::new();
            
            // Alternate between a few frequencies.
            let waveform1 = wfc.get_sine(2048.0/16.0, 2048);
            let waveform2 = wfc.get_sine(2048.0/12.0, 2048);
            let rg = report_globals.current_volume.clone();

            let callback = move |pa::DuplexStreamCallbackArgs { in_buffer,
                                                                out_buffer,
                                                                frames,
                                                                flags,
                                                                time }| {
                let start_time = time::get_time();
                let mut volume = 0.0f32;
                let amplitude = 0.1;

                for n in 0..frames {
                    let t = ((3.141516*2.0)*(n as f32)/(frames as f32)).sin();
                    let a1 = 0.5 + 0.5 * amplitude * t;
                    let a2 = amplitude * (1.0-a1);
                    
                    volume += (in_buffer[n] as f32).abs();
                    
                    out_buffer[n] = a1 * waveform1[n] + a2 * waveform2[n];
                }
                
                let avg_volume_db = dsp_waveforms::ampl2dbfs(volume / (frames as f32));
                
                let mut data = rg.lock().unwrap();
                *data = avg_volume_db;                

                let time_left = deadline - time::get_time();
                let taken_time = time::get_time() - start_time;
                println!("Got {} volume units over {} frames. Some frames are {}, {}. {} left. Callback took {}.", avg_volume_db, frames, in_buffer[4], in_buffer[6], time_left, taken_time);
                
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
        
        let mut rg : ReportGlobals = ReportGlobals::new();
        
        let pa = portaudio::PortAudio::new().unwrap();
        let mut rp = RecordingPath::new(&pa, &rg);
        
        let device = rp.settings.input_device;
        let device_info = pa.device_info(device).unwrap();
        
        try!(rp.stream.start());

        let title = format!("{}", device_info.name);
        /*
        let mut window: PistonWindow<Sdl2Window> = WindowSettings::new(title, (640, 480))
                                                        .build()
                                                        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
        
        */
        
        ui::start_ui(&rg);
          
        // While stream is running, idle.
        while let true = try!(rp.stream.is_active()) {
    		/*
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
    		*/
            std::thread::sleep(std::time::Duration::from_millis(16));
            
        }
        
        try!(rp.stream.stop());

        Ok(())

    }
}
