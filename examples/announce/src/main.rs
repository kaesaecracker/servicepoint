use clap::Parser;

use servicepoint2::{ByteGrid, Command, Connection, Grid, Origin};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value = "localhost:2342")]
    destination: String,
    #[arg(short, long, num_args = 1.., value_delimiter = '\n')]
    text: Vec<String>,
    #[arg(short, long, default_value_t = true)]
    clear: bool,
}

/// example: `cargo run -- --text "Hallo,
/// CCCB"`

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    let connection = Connection::open(&cli.destination).unwrap();
    if cli.clear {
        connection.send(Command::Clear.into()).unwrap();
    }

    let max_width = cli.text.iter().map(|t| t.len()).max().unwrap();

    let mut chars = ByteGrid::new(max_width, cli.text.len());
    for y in 0..cli.text.len() {
        let row = &cli.text[y];

        for (x, char) in row.chars().enumerate() {
            let char = char.try_into().expect("invalid input char");
            chars.set(x, y, char);
        }
    }

    connection
        .send(Command::Cp437Data(Origin(0, 0), chars).into())
        .unwrap();
}
