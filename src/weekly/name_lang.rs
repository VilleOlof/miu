/// All the languages for Challenge names
pub enum NameLang {
    /// English
    En,
    /// Spanish
    Es,
    /// French
    Fr,
    /// German
    De,
    /// Italy
    It,
    /// Japanese
    Jp,
    /// Arabic
    Ar,
    /// Chinese
    ZhCh,
    /// Taiwainese
    ZhTw,
    /// Netherlands
    Nl,
    /// Korean
    Ko,
    /// Portgual
    Pt,
    /// Russian
    Ru,
    /// Turkey
    Tr,
}
impl ToString for NameLang {
    fn to_string(&self) -> String {
        String::from(match self {
            NameLang::En => "en",
            NameLang::Es => "es",
            NameLang::Fr => "fr",
            NameLang::De => "de",
            NameLang::It => "it",
            NameLang::Jp => "jp",
            NameLang::Ar => "ar",
            NameLang::ZhCh => "zh-CN",
            NameLang::ZhTw => "zh-TW",
            NameLang::Nl => "nl",
            NameLang::Ko => "ko",
            NameLang::Pt => "pt",
            NameLang::Ru => "ru",
            NameLang::Tr => "tr",
        })
    }
}
