pub trait PairedTag {
    fn paired_tag() -> &'static str;
}

#[macro_export]
macro_rules! impl_paired_tag {
    ($marker:ident, $value:literal) => {
        impl crate::markers::markers::paired::PairedTag for $marker {
            fn paired_tag() -> &'static str {
                $value
            }
        }
    };
}
