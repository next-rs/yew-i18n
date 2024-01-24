use serde_json::Value;
use std::collections::HashMap;
use yew::prelude::*;

/// Configuration for the YewI18n module, specifying supported languages and translations.
#[derive(Debug, Clone, PartialEq)]
pub struct YewI18nConfig {
    /// List of supported languages in the application.
    pub supported_languages: Vec<&'static str>,
    /// Translations for different languages, represented as a mapping from language codes to JSON values.
    pub translations: HashMap<String, serde_json::Value>,
}

/// Configuration for the YewI18nProvider component.
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct YewI18nProviderConfig {
    /// List of supported languages. Defaults to English and French if not specified.
    #[prop_or_else(|| vec!["en", "fr"])]
    pub supported_languages: Vec<&'static str>,
    /// Translations for different languages, represented as a mapping from language codes to JSON values.
    #[prop_or_default]
    pub translations: HashMap<String, serde_json::Value>,
    /// The child components to be wrapped with the YewI18n context.
    pub children: Html,
}

/// The YewI18n struct representing the state and methods for internationalization.
#[derive(Clone, PartialEq)]
pub struct YewI18n {
    /// Configuration for YewI18n, specifying supported languages and translations.
    pub config: YewI18nConfig,
    /// The current language code for translations.
    current_language: String,
    /// Translations for different languages, represented as a mapping from language codes to JSON values.
    translations: HashMap<String, serde_json::Value>,
}

impl YewI18n {
    /// Creates a new instance of YewI18n.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for YewI18n.
    /// * `translations` - Translations for different languages.
    ///
    /// # Returns
    ///
    /// A Result containing the initialized YewI18n instance or an error message.
    ///
    /// # Examples
    ///
    /// ```
    /// use yew_i18n::{YewI18n, YewI18nConfig};
    /// use std::collections::HashMap;
    ///
    /// let supported_languages = vec!["en", "fr"];
    /// let translations = HashMap::new();
    ///
    /// let result = YewI18n::new(YewI18nConfig { supported_languages, translations: translations.clone()}, translations);
    /// assert!(result.is_ok());
    /// ```
    pub fn new(
        config: YewI18nConfig,
        translations: HashMap<String, serde_json::Value>,
    ) -> Result<Self, String> {
        let current_language = config
            .supported_languages
            .get(0)
            .cloned()
            .ok_or_else(|| "You must add at least one supported language".to_string())?;

        Ok(YewI18n {
            config,
            current_language: current_language.to_string(),
            translations,
        })
    }

    /// Sets the current language for translations.
    ///
    /// # Arguments
    ///
    /// * `language` - The language code to set.
    ///
    /// # Returns
    ///
    /// A Result indicating success or an error message if the language is not supported.
    ///
    /// # Examples
    ///
    /// ```
    /// use yew_i18n::{YewI18n, YewI18nConfig};
    /// use std::collections::HashMap;
    ///
    /// let supported_languages = vec!["en", "fr"];
    /// let translations = HashMap::new();
    ///
    /// let mut i18n = YewI18n::new(YewI18nConfig { supported_languages, translations: translations.clone()}, translations).unwrap();
    /// assert!(i18n.set_translation_language("fr").is_ok());
    /// ```
    pub fn set_translation_language(&mut self, language: &str) -> Result<(), String> {
        if self.config.supported_languages.contains(&&language) {
            self.current_language = language.to_string();
            Ok(())
        } else {
            Err(format!("Language '{}' is not supported", language))
        }
    }

    /// Retrieves a translated string for a given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The translation key.
    ///
    /// # Returns
    ///
    /// The translated string or an error message if the key is not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use yew_i18n::{YewI18n, YewI18nConfig};
    /// use std::collections::HashMap;
    /// use serde_json::json;
    ///
    /// let supported_languages = vec!["en", "fr"];
    /// let mut translations = HashMap::new();
    /// translations.insert("en".to_string(), json!({ "greeting": "Hello" }));
    /// translations.insert("fr".to_string(), json!({ "greeting": "Bonjour" }));
    ///
    /// let i18n = YewI18n::new(YewI18nConfig { supported_languages, translations: translations.clone()}, translations).unwrap();
    /// assert_eq!(i18n.t("greeting"), "Hello");
    /// ```
    pub fn t(&self, key: &str) -> String {
        self.translations
            .get(&self.current_language)
            .and_then(|language_json| language_json.get(key))
            .map_or_else(
                || {
                    Err(format!(
                        "Unable to find the key '{}' in the language '{}'",
                        key, self.current_language
                    ))
                },
                |value| match value {
                    Value::String(s) => Ok(s.clone()),
                    _ => Ok(value.to_string()),
                },
            )
            .unwrap_or_else(|err| err)
    }
}

/// Yew component for providing the YewI18n context to its children.
#[function_component(I18nProvider)]
pub fn i18n_provider(props: &YewI18nProviderConfig) -> Html {
    let i18n = YewI18n::new(
        YewI18nConfig {
            supported_languages: props.supported_languages.clone(),
            translations: props.translations.clone(),
        },
        props.translations.clone(),
    )
    .expect("Failed to initialize YewI18n");

    let ctx = use_state(|| i18n);

    html! {
        <ContextProvider<YewI18n> context={(*ctx).clone()}>{ props.children.clone() }</ContextProvider<YewI18n>>
    }
}
