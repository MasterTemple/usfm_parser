pub trait SimpleTag {
    fn simple_tag() -> &'static str;
}

#[macro_export]
macro_rules! impl_simple_tag {
    ($marker:ident, $value:literal) => {
        impl crate::markers::markers::tag::Tag for $marker {
            const TAG: &'static str = $value;
        }
        impl crate::markers::markers::simple::SimpleTag for $marker {
            fn simple_tag() -> &'static str {
                $value
            }
        }
    };
}
