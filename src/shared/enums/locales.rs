#[derive(Clone, IntoStaticStr, EnumIter)]
pub enum LocaleVariables {
    Username,
    AppName,
    Location,
}

#[derive(Clone, IntoStaticStr, EnumIter)]
pub enum LocaleKeys {
    GreetingUsername,
    GreetingNoUsername,
    GreetingLocation,
    ShouldBePlainText,
    ShouldBeLocation,
    RecordedCredentials,
}
