#![doc(
    html_logo_url = "https://github.com/next-rs/yew-i18n/assets/62179149/a46bdd6b-2d70-4fa6-ae95-02f6303ec7a3",
    html_favicon_url = "https://github.com/next-rs/yew-i18n/assets/62179149/eaaebb8f-95bb-45ab-87e8-f278b583a0c7"
)]

//! # Yew I18n - Documentation
//!
//! Welcome to the official Yew I18n documentation. This library
//! provides internationalization support for your Yew applications.
//!
//! ## Usage
//!
//! To use the Yew I18n library, add the following dependency to your `Cargo.toml` file:
//!
//! ```sh
//! cargo add yew-i18n
//! ```
//!
//! To integrate the library into your Yew application, you can use the `I18nProvider` component.
//! Here's a simple example of how to use it:
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use std::collections::HashMap;
//! use yew_i18n::{I18nProvider, YewI18nProviderConfig, use_translation};
//!
//! // Your Yew component structure here...
//!
//! #[function_component]
//! pub fn MyI18nComponent() -> Html {
//!     // Your component logic here...
//!
//!     let translation = use_translation();
//!
//!     html! {
//!         <I18nProvider
//!             supported_languages={vec!["en", "fr"]}
//!             translations={HashMap::new()}
//!         >
//!             <div />
//!             // Your components that need translation here...
//!         </I18nProvider>
//!     }
//! }
//! ```
//!
//! For more detailed information, check the [examples] provided in the library.
//!
//! [examples]: https://github.com/next-rs/yew-i18n/tree/main/examples
//!
//! ## Configuration
//!
//! Yew I18n allows you to configure the supported languages and translations through the
//! `YewI18nProviderConfig` structure. You can also use the `YewI18n` struct to handle
//! translation-related operations programmatically. Refer to the respective documentation
//! for detailed configuration options.
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use yew_i18n::{YewI18nProviderConfig, YewI18nConfig, YewI18n, I18nProvider};
//! use std::collections::HashMap;
//!
//! let i18n_provider_config = YewI18nProviderConfig {
//!     supported_languages: vec!["en", "fr"],
//!     translations: HashMap::new(),
//!     children: html! { /* Your child components here... */ },
//! };
//!
//! let i18n_provider_component = html! {
//!     <I18nProvider ..i18n_provider_config />
//! };
//!
//! let supported_languages = vec!["en", "fr"];
//! let translations = HashMap::new();
//!
//! let i18n = YewI18n::new(YewI18nConfig { supported_languages, translations: translations.clone()}, translations);
//! assert!(i18n.is_ok());
//! ```
//!
//! ## Translation
//!
//! Yew I18n provides a hook function `use_translation` to easily access the translation context
//! within your components. You can use this hook to retrieve translations for keys in your
//! components.
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use yew_i18n::use_translation;
//!
//! #[function_component]
//! pub fn MyTranslatableComponent() -> Html {
//!     // Your component logic here...
//!
//!     let i18n = use_translation();
//!     let greeting = i18n.t("greeting");
//!
//!     html! {
//!         <div>{ greeting }</div>
//!     }
//! }
//! ```
//!
//! ## Contribution
//!
//! If you encounter any issues or have suggestions for improvements, feel free to contribute
//! to the [GitHub repository](https://github.com/next-rs/yew-i18n). We appreciate your feedback
//! and involvement in making Yew I18n better!
//!
//! ## Acknowledgments
//!
//! Special thanks to the Yew community and contributors for such an amazing framework.
//!

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

#[hook]
pub fn use_translation() -> YewI18n {
    use_context::<YewI18n>().expect("No I18n context provided")
}
