
use std;
use portaudio;
use portaudio as pa;
use time;

pub struct ExtractedInfo<'a> {
    def_input: portaudio::DeviceIndex,
    def_output: portaudio::DeviceIndex,
    input_latency: f64,
    output_latency: f64,
    pa: &'a portaudio::PortAudio,
}


impl<'a> ExtractedInfo<'a> {
    pub fn new(pa: &portaudio::PortAudio) -> ExtractedInfo {
        //let def_input = pa.default_input_device().unwrap();
        let def_input = portaudio::DeviceIndex(0);
        //let def_output = pa.default_output_device().unwrap();
        let def_output = portaudio::DeviceIndex(1);
        
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


pub struct IOSettings<'a> {
    sample_rate: f64,
    pub pa_settings: portaudio::stream::DuplexSettings<f32, f32>,
    pa_handle: &'a portaudio::PortAudio,
    chunk_size: u32,
    is_interleaved: bool,
    pub input_device: portaudio::DeviceIndex,
    pub output_device: portaudio::DeviceIndex,
}



impl<'a> IOSettings<'a> {
    pub fn quickstart(pa: &portaudio::PortAudio) -> IOSettings {

        IOSettings::print_devices(pa);

        let sample_rate = 48000.0;
        let chunk_size = 2048; // 2k gives ~ 43ms chunks
        let is_interleaved = true;
        let einfo = ExtractedInfo::new(&pa);
        let input_device = einfo.def_input;
        let output_device = einfo.def_output;

        let input_params =
            pa::StreamParameters::<f32>::new(input_device, 1, is_interleaved, einfo.input_latency);
        let output_params =
            pa::StreamParameters::new(output_device, 1, is_interleaved, einfo.output_latency);


        println!("Using input device {:?}, output device {:?}", input_device, output_device);

        let settings =
            pa::DuplexStreamSettings::new(input_params, output_params, sample_rate, chunk_size);

        IOSettings {
            sample_rate: sample_rate,
            pa_settings: settings,
            pa_handle: &pa,
            chunk_size: chunk_size,
            is_interleaved: is_interleaved,
            input_device: input_device,
            output_device: output_device,
        }
    }

    pub fn print_devices(pa: &portaudio::PortAudio) -> () {
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
