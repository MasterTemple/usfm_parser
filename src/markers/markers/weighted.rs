use std::ops::Deref;

/// https://ubsicap.github.io/usfm/characters/nesting.html
pub trait WeightedTag: Deref<Target = Option<u8>> {
    fn unweighted_tag() -> &'static str;
    fn weighted_tag(&self) -> String {
        let tag = Self::unweighted_tag();
        let num = self.deref();
        let num = num.map(|n| n.to_string()).unwrap_or_default();
        format!("{}{}", tag, num)
    }
    fn weight(&self) -> u8 {
        self.deref().unwrap_or(1)
    }
}

#[macro_export]
macro_rules! impl_weighted_tag {
    ($marker:ident, $value:literal) => {
        impl crate::markers::markers::tag::Tag for $marker {
            const TAG: &'static str = $value;
        }

        impl $crate::markers::markers::parameters::FromMarkerParameters for $marker {
            fn from_parameters(
                params: $crate::markers::markers::parameters::MarkerParameters,
            ) -> Self {
                Self(params.digits)
            }
        }

        impl crate::markers::markers::weighted::WeightedTag for $marker {
            fn unweighted_tag() -> &'static str {
                $value
            }
        }
    };
}
