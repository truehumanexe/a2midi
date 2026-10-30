
use nice_plug::prelude::*;
use a2midi::NicePlug;

fn main() {
    eprintln!(">>> APP STARTED SUCCESSFULLY <<<");
    nice_export_standalone::<NicePlug>();
}
