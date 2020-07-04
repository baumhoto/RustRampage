use serde_repr::*;

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum Thing {
    Nothing,
    Player,
}
