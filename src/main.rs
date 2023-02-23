#![no_std]
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

extern crate alloc;

use agb::display::busy_wait_for_vblank;
use agb::display::object::ObjectController;
use agb::display::tiled::RegularBackgroundSize;
use agb::display::tiled::TileFormat;
use agb::display::tiled::TileSet;
use agb::display::tiled::TileSetting;
use agb::display::tiled::TiledMap;
use agb::display::Priority;
use agb::hash_map::HashMap;
use agb::input::ButtonController;
use agb::println;

agb::include_gfx!("gfx/background.toml");

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
  let (tiled0, mut vram) = gba.display.video.tiled0();
  let object_ctrl = gba.display.object.get();
  let tileset = TileSet::new(background::background.tiles, TileFormat::EightBpp);
  vram.set_background_palettes(background::PALETTES);
  let mut bg = tiled0.background(Priority::P0, RegularBackgroundSize::Background32x32);
  for y in 0..20u16 {
    for x in 0..30u16 {
      let idx = x + y * 30u16;
      let pos = (x, y).into();
      bg.set_tile(&mut vram, pos, &tileset, TileSetting::new(idx, false, false, 0));
    }
  }
  bg.commit(&mut vram);
  bg.show();

  loop {
    busy_wait_for_vblank();
  }
}
