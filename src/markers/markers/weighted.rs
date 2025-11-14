pub trait WeightedTag {
    fn weighted_tag(&self) -> String;
}

#[macro_export]
macro_rules! impl_weighted_tag {
    ($marker:ident, $value:literal) => {
        impl crate::markers::markers::weighted::WeightedTag for $marker {
            fn weighted_tag() -> String {
                $value
            }
        }
    };
}
