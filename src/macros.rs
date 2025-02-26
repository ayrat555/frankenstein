pub use macro_rules_attribute::apply;

macro_rules_attribute::attribute_alias! {
    // Enable [`bon::builder`] `into` for specific types to reduce boilerplate for the callers.
    #[apply(apistruct!)] =
        #[::serde_with::skip_serializing_none]
        #[derive(Clone, Debug, PartialEq, ::bon::Builder, ::serde::Serialize, ::serde::Deserialize)]
        #[builder(
            on(Box<_>, into),
            on(ChatId, into),
            on(FileUpload, into),
            on(InputFile, into),
            on(String, into),
        )];
}
