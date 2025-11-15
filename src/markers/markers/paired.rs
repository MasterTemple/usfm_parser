use std::ops::Deref;

pub trait PairedTag: Deref<Target = bool> {
    fn paired_tag() -> &'static str;
    fn is_opening(&self) -> bool {
        *self.deref()
    }
    fn is_closing(&self) -> bool {
        !self.is_opening()
    }
}

#[macro_export]
macro_rules! impl_paired_tag {
    ($marker:ident, $value:literal) => {
        impl crate::markers::markers::tag::Tag for $marker {
            const TAG: &'static str = $value;
        }

        impl $crate::markers::markers::parameters::FromMarkerParameters for $marker {
            fn from_parameters(
                params: $crate::markers::markers::parameters::MarkerParameters,
            ) -> Self {
                Self(!params.has_asterisk)
            }
        }

        impl crate::markers::markers::paired::PairedTag for $marker {
            fn paired_tag() -> &'static str {
                $value
            }
        }
    };
}
