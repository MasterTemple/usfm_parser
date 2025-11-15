use crate::markers::markers::{any::AnyMarker, tag::Tag};

#[derive(Clone, Copy, Debug)]
pub struct MarkerParameters {
    pub digits: Option<u8>,
    pub has_asterisk: bool,
}

impl MarkerParameters {
    pub fn new(digits: Option<u8>, has_asterisk: bool) -> Self {
        Self {
            digits,
            has_asterisk,
        }
    }
}

#[enum_dispatch::enum_dispatch]
pub trait FromMarkerParameters: Tag // + Into<AnyMarker>
{
    fn from_parameters(params: MarkerParameters) -> Self;
}
