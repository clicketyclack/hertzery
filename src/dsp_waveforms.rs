/// DSP waveforms.

use std::collections::HashMap;
use std;
use std::f32;


/// Basic struct for holding DSP windows.
pub struct DSPWindow {
    forms_by_bsize : HashMap<u32, Vec<f32>>,
}



impl DSPWindow {
    
    fn new() -> DSPWindow {
        let v512 = [0.0f32 ; 512];
        let v2048 = [0.0f32 ; 512];
        let mut m : HashMap<u32, Vec<f32>> = HashMap::new(); //();
        DSPWindow { forms_by_bsize : m }
    }
    
}


/// Convert amplitude to dbfs value.
/// # Parameters
///
/// `amplitude` : An amplitude, usually in the range 0.0 to 1.0.
/// 
/// # Returns
///     An f32 which usually spans between 0.0 to -infinity.
pub fn ampl2dbfs(amplitude : f32) -> f32 { 
    10.0 * amplitude.log10()
}

pub struct DSPWindowCollection {
    hamming : DSPWindow,
}


impl DSPWindowCollection {
    
    fn new() -> DSPWindowCollection {
        let mut hamming = DSPWindow::new();
        DSPWindowCollection { hamming : hamming }    
    }
    
    fn get_hamming(&self, bsize : usize) -> Vec<f32> {
        // TODO: Cache this
        // Get a hamming window of bsize matching arg
        let alpha = 0.53836;
        let beta = 1.0 - alpha;
        
        let mut toreturn : Vec<f32> = vec![0.0f32;bsize];
        let pi = f32::consts::PI;
        
        let denom = (bsize as f32) - 1.0;
            
        for n in 0..bsize {
            let numer = 2.0 * pi * (n as f32);
            let frac = numer / denom;
            let val = alpha - beta * frac.cos();
            
            toreturn[n] = val;
            
        }
        
        toreturn
    }


}


pub struct WaveFormCache {
    waveforms : HashMap<[u32;2], Vec<f32>>,
}



impl WaveFormCache {
    pub fn new() -> WaveFormCache {
        let m : HashMap<[u32;2], Vec<f32>> = HashMap::new();
        //let mut waveforms = DSPWindow::new();
    
        WaveFormCache { waveforms : m }    
    }
    
    pub fn get_sine(&self, period : f32, length : usize) -> Vec<f32> {
        let mut toreturn : Vec<f32> = vec![0.0f32;length];
        for n in 0..length {
            let t = 2.0 * f32::consts::PI * (((n as f32)/(period)) % 1.0); 
            toreturn[n] = t.sin();
        }       
        
        toreturn
    }
}



#[test]
fn instansiate_waveformbuilders() {
    DSPWindowCollection::new();
    WaveFormCache::new();
}

#[test]
fn converter_ampl2dbfs() {
     let n3db = ampl2dbfs(0.5);
     assert!(-3.1 < n3db && n3db < -2.9);
     
     
     let n6db = ampl2dbfs(0.25);
     assert!(-6.1 < n6db && n6db < -5.9);
     
     let n0db = ampl2dbfs(1.0);
     assert!(-0.01 < n0db && n0db < 0.01);
     
     let n0db2 = ampl2dbfs(0.00);
     println!("Got {}", n0db2);
     assert!(n0db2 == f32::NEG_INFINITY);
     
}

#[test]
fn gen_hamming() {
    let d = DSPWindowCollection::new();
    let h16 = d.get_hamming(16);
    let h512 = d.get_hamming(512);
    assert!(h16.len() == 16);
    assert!(h512.len() == 512);
    
    
    assert!(h16[0] < 0.1);
    assert!(h16[7] > 0.6); 

    assert!(h512[511] < 0.1);
    assert!(h512[256] > 0.6); 
    
}

#[test]
fn gen_sine_16() {
    // Test simple block of 16-periodic sin function.
    let wfc = WaveFormCache::new();
    let sin16_16 = wfc.get_sine(16.0, 16);
    let sin16_17 = wfc.get_sine(16.0, 17);
    
    //for n in 0..sin16_16.len() {
    //    println!("{} {} {}", n, sin16_16[n], sin16_17[n]);
    //}

    for n in 0..sin16_16.len() {
        assert!(sin16_16[n] == sin16_17[n]);
    }
    
    assert!(sin16_17[0] == 0.0);
    assert!(sin16_17[0] == 0.0);
    assert!(sin16_17[0] == sin16_17[16]);
    
    assert!((sin16_17[1] + sin16_17[9]).abs() < 0.001);
    
    
    
}


