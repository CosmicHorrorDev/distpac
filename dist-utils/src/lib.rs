pub mod misc;
pub mod path;

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Client,
    Server,
}
