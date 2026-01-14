use macro_rules_attribute::derive_alias;

derive_alias! {
    #[derive(Copy!)] = #[derive(Clone, Copy)];
    #[derive(Eq!)] = #[derive(PartialEq, Eq)];
    #[derive(Ord!)] = #[derive(PartialOrd, Ord)];
    #[derive(Partial!)] = #[derive(PartialOrd, PartialEq)];
    #[derive(Cmp!)] = #[derive(crate::Eq!, crate::Ord!)];
    #[derive(Basic!)] = #[derive(crate::Cmp!, Clone, Debug)];

    #[derive(Serde!)] = #[derive(serde::Serialize, serde::Deserialize)];

    #[derive(Marker!)] = #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)];
    #[derive(Deref!)] = #[derive(derive_more::Deref, derive_more::DerefMut)];
    #[derive(From!)] = #[derive(derive_more::Into, derive_more::From)];

    #[derive(Add!)] = #[derive(derive_more::Add, derive_more::AddAssign, derive_more::Sum)];
    #[derive(Mul!)] = #[derive(derive_more::Mul, derive_more::MulAssign, derive_more::Product)];
    #[derive(Sub!)] = #[derive(derive_more::Sub, derive_more::SubAssign)];
    #[derive(Div!)] = #[derive(derive_more::Div, derive_more::DivAssign)];
    #[derive(Math!)] = #[derive(
        crate::Add!,
        crate::Sub!,
        crate::Mul!,
        crate::Div!,
    )];
}
