macro_rules_attribute::attribute_alias! {
    // Enable [`bon::builder`] `into` for specific types to reduce boilerplate for the callers.
    // Keep [`PartialEq`] and [`Clone`] out of it as [`Copy`] / [`Eq`] might also exist.
    #[apply(serdebuilder!)] =
        #[::serde_with::skip_serializing_none]
        #[derive(::bon::Builder, ::serde::Serialize, ::serde::Deserialize)]
        #[builder(
            on(String, into),
            on(ChatId, into),
            on(FileUpload, into),
            on(Box<_>, into),
        )];
}
