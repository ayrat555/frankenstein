macro_rules_attribute::attribute_alias! {
    // Shared configuration for all builder derives. Make sure to add them to
    // all API types.
    //
    // We enable `into` for specific types to reduce boilerplate for the callers.
    #[apply(builder!)] =
        #[derive(::bon::Builder)]
        #[builder(
            on(String, into),
            on(ChatId, into),
            on(FileUpload, into),
            on(Box<_>, into),
        )];
}
