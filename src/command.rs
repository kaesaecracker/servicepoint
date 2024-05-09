use crate::{BitVec, Header, Packet, PixelGrid, ToPacket};

/// A window
#[derive(Debug)]
pub struct Window(pub Origin, pub Size);

/// An origin marks the top left position of the
/// data sent to the display.
/// A window
#[derive(Debug)]
pub struct Origin(pub u16, pub u16);

/// Size defines the width and height of a window
/// A window
#[derive(Debug)]
pub struct Size(pub u16, pub u16);

type Offset = u16;

type Brightness = u8;


#[derive(Debug)]
pub enum Command {
    Clear,
    HardReset,
    FadeOut,
    CharBrightness(Window, Vec<Brightness>),
    Brightness(Brightness),
    BitmapLinear(Offset, BitVec),
    BitmapLinearAnd(Offset, BitVec),
    BitmapLinearOr(Offset, BitVec),
    BitmapLinearXor(Offset, BitVec),
    Cp437Data(Window, Vec<u8>),
    BitmapLinearWin(Window, PixelGrid),
}

fn offset_and_payload(command: u16, offset: Offset, payload: Vec<u8>) -> Packet {
    Packet(Header(command, offset, payload.len() as u16, 0, 0), payload)
}

fn window_and_payload(command: u16, window: Window, payload: Vec<u8>) -> Packet {
    let Window(Origin(x, y), Size(w, h)) = window;
    Packet(Header(command, x, y, w, h), payload.into())
}

impl ToPacket for Command {
    fn to_packet(self) -> Packet {
        match self {
            Command::Clear => Packet(Header(0x0002, 0x0000, 0x0000, 0x0000, 0x0000), vec!()),
            Command::CharBrightness(window, payload) => window_and_payload(0x0005, window, payload),
            Command::Brightness(brightness) => Packet(Header(0x0007, 0x00000, 0x0000, 0x0000, 0x0000), vec!(brightness)),
            Command::HardReset => Packet(Header(0x000b, 0x0000, 0x0000, 0x0000, 0x0000), vec!()),
            Command::FadeOut => Packet(Header(0x000d, 0x0000, 0x0000, 0x0000, 0x0000), vec!()),
            Command::BitmapLinear(offset, bits) => offset_and_payload(0x0012, offset, bits.into()),
            Command::BitmapLinearWin(window, pixels) => window_and_payload(0x0013, window, pixels.into()),
            Command::BitmapLinearAnd(offset, bits) => offset_and_payload(0x0014, offset, bits.into()),
            Command::BitmapLinearOr(offset, bits) => offset_and_payload(0x0015, offset, bits.into()),
            Command::BitmapLinearXor(offset, bits) => offset_and_payload(0x0016, offset, bits.into()),
            Command::Cp437Data(window, payload) => window_and_payload(0x0003, window, payload),
        }
    }
}
