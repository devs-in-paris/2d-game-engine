mod libs;

use crate::libs::walk;
use crate::libs::walk::notan_main;

fn main() {
    walk::notan_main().expect("TODO: panic message");
}