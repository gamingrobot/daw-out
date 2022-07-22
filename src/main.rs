use nih_plug::prelude::*;

use daw_out::DawOut;

fn main() {
    nih_export_standalone::<DawOut>();
}
