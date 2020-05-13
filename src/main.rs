//! A simple CLI UFO editor.

use norad::Ufo;
use clap::App;

fn main() {

    App::new("lilufo")
       .version("1.0")
       .about("Tools for working with Unified Font Objects.")
       .author("Eli Heuer")
       .get_matches(); 

    println!("    .     *     .           .     ");
    println!("   .-----.                        ");
    println!(" _/___@_@_\\_              .      ");
    println!("(___________)      *              ");
    println!("                                  ");
    println!("Hello, world!");

    let path = "~/Type/clones/rompacta/sources/rompacta.ufo";
    let ufo = Ufo::load(&path).expect("failed to load file");

    let font_name = ufo
        .font_info
        .as_ref()
        .and_then(|f| f.family_name.clone())
        .unwrap_or_else(|| "an unnamed font".into());

    println!("Loaded {} glyphs from {}.", ufo.glyph_count(), font_name);

}
