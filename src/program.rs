// State etc.
// Communicators.

use std::sync::{Arc, Mutex};



pub struct ReportGlobals {
    // TODO : Yes, we want other report globals. No we don't have a struct just for one member...
    pub current_volume : Arc<Mutex<f32>>
}


impl ReportGlobals {
    pub fn new() -> ReportGlobals { 
        ReportGlobals { current_volume : Arc::new(Mutex::new(0.0f32)) }
    }
}

