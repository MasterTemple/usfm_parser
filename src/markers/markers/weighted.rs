pub trait WeightedTag {
    fn unweighted_tag() -> &'static str;
    // fn weighted_tag(&self) -> String;
}

#[macro_export]
macro_rules! impl_weighted_tag {
    ($marker:ident, $value:literal) => {
        impl crate::markers::markers::weighted::WeightedTag for $marker {
            fn unweighted_tag() -> &'static str {
                $value
            }
        }
    };
}
