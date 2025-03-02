use macro_rules_attribute::attribute_alias;

attribute_alias! {
    #[apply(base)] =
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))];

    #[apply(ext)] =
        #[apply(crate::macros::base)]
        #[derive(Copy)];
}
