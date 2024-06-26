use std::time::Duration;

use clap::Parser;
use rand::Rng;

use servicepoint2::Command::{BitmapLinearWin, Brightness, CharBrightness};
use servicepoint2::*;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value = "localhost:2342")]
    destination: String,
    #[arg(short, long, default_value_t = true)]
    enable_all: bool,
    #[arg(short, long, default_value_t = 100, allow_negative_numbers = false)]
    wait_ms: u64,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    let connection = Connection::open(cli.destination).unwrap();
    let wait_duration = Duration::from_millis(cli.wait_ms);

    // put all pixels in on state
    if cli.enable_all {
        let mut filled_grid = PixelGrid::max_sized();
        filled_grid.fill(true);

        let command =
            BitmapLinearWin(Origin(0, 0), filled_grid, CompressionCode::Lzma);
        connection.send(command.into()).expect("send failed");
    }

    // set all pixels to the same random brightness
    let mut rng = rand::thread_rng();
    connection.send(Brightness(rng.gen()).into()).unwrap();

    // continuously update random windows to new random brightness
    loop {
        let min_size = 1;
        let x = rng.gen_range(0..TILE_WIDTH - min_size);
        let y = rng.gen_range(0..TILE_HEIGHT - min_size);

        let w = rng.gen_range(min_size..=TILE_WIDTH - x);
        let h = rng.gen_range(min_size..=TILE_HEIGHT - y);

        let origin = Origin(x, y);
        let mut luma = ByteGrid::new(w, h);

        for y in 0..h {
            for x in 0..w {
                luma.set(x, y, rng.gen());
            }
        }

        connection
            .send(CharBrightness(origin, luma).into())
            .unwrap();
        std::thread::sleep(wait_duration);
    }
}
