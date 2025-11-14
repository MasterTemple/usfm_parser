pub trait PairedTag {
    fn paired_tag() -> &'static str;
}

#[macro_export]
macro_rules! impl_paired_tag {
    ($marker:ident, $value:literal) => {
        impl crate::markers::markers::tag::Tag for $marker {
            const TAG: &'static str = $value;
        }
        impl crate::markers::markers::paired::PairedTag for $marker {
            fn paired_tag() -> &'static str {
                $value
            }
        }
    };
}
