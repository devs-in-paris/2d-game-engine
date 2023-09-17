use notan::notan_main;

pub mod walk;
pub mod draw;

#[notan_main]
fn main() -> Result<(), String> {
    // walk::walk()
    draw::sketch()
}