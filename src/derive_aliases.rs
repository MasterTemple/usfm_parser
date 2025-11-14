use macro_rules_attribute::derive_alias;

derive_alias! {
    #[derive(Marker!)] = #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)];
    #[derive(Weighted!)] = #[derive(derive_more::Deref, derive_more::DerefMut)];
}
