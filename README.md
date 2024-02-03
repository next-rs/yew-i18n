# 🌐 Yew I18n

[![Crates.io](https://img.shields.io/crates/v/yew-i18n)](https://crates.io/crates/yew-i18n)
[![Crates.io Downloads](https://img.shields.io/crates/d/yew-i18n)](https://crates.io/crates/yew-i18n)
![Crates.io License](https://img.shields.io/crates/l/yew-i18n)
![Rust](https://img.shields.io/badge/rust-stable-orange)
[![Netlify Status](https://api.netlify.com/api/v1/badges/e8246c00-9789-4483-bcd4-b500eefa6f6a/deploy-status)](https://yew-i18n.netlify.app)

---

[![Demo](https://github.com/wiseaidev/yew-i18n/assets/62179149/473423ad-d3e2-4080-810b-637c119e6d37)](https://yew-i18n.netlify.app)

---

## 📜 Introduction

Yew I18n is a Yew component that provides internationalization (i18n) support for your web applications. It allows you to manage translations and switch between different languages seamlessly, enhancing the user experience for a global audience.

## 🤔 Why is this Component Useful?

This library offers several benefits to make i18n implementation in your Yew projects straightforward:

1. 🌍 Multi-Language Support: Easily manage translations for various languages in your application.

1. 🚀 Seamless Integration: Integrate i18n seamlessly into your Yew components, providing a consistent language experience.

1. 💬 Dynamic Language Switching: Dynamically switch between supported languages to cater to diverse user preferences.

## ⚙️ Installation

Integrating Yew I18n into your Yew project is a simple process. Follow these steps:

1. Make sure you have Yew set up in your project. If not, refer to the [Yew documentation](https://yew.rs/docs/getting-started/introduction) for installation instructions.

1. Install the library using your preferred package manager:

   ```bash
   $ cargo add yew-i18n
   ```

1. Start using the library to manage translations and enhance the multilingual capabilities of your application.

## 🛠️ Usage

Incorporating Yew I18n into your application is easy. Follow these steps:

1. Set up the i18n configuration and provider:

   ```rust
   use crate::components::my_component::MyComponent;
   use yew_i18n::I18nProvider;
   use std::collections::HashMap;
   use yew::prelude::*;

   #[function_component(App)]
   pub fn app() -> Html {
       let supported_languages = vec!["en", "fr"];
       let mut translations = HashMap::new();

       translations.insert(
       	   // en to en
           "en".to_string(),
           serde_json::json!({
               "24 Apr, 2023": "24 Apr, 2023",
               "02 May, 2023": "02 May, 2023",
               "11 May, 2023": "11 May, 2023",
               "Trending Posts": "Trending Posts",
               "Rust: The Next Big Thing in Data Science": "Rust: The Next Big Thing in Data Science",
               "Data Science": "Data Science",
           }),
       );

       translations.insert(
       	   // en to fr
           "fr".to_string(),
           serde_json::json!({
               "24 Apr, 2023": "24 Avr, 2023",
               "02 May, 2023": "02 Mai, 2023",
               "11 May, 2023": "11 Mai, 2023",
               "Trending Posts": "Articles Tendances",
               "Rust: The Next Big Thing in Data Science": "Rust : La Prochaine Grande Avancée en Science des Données",
               "Data Science": "Science des Données",
           }),
       );

       html! {
        	<I18nProvider supported_languages={supported_languages} translations={translations} >
        	    <MyComponent />
        	</I18nProvider>
       }
   }
   ```

1. Use the `use_translation` hook to access the i18n context in your components:

   ```rust
   // ./src/components/my_component.rs
   use yew::prelude::*;
   use yew_i18n::use_translation;

   #[function_component(MyComponent)]
   pub fn my_component() -> Html {
       let i18n = use_translation();

       i18n.set_translation_language(&"fr");

       // Your component, states, etc.

       html! {
       	   <div>
               { i18n.t("Trending Posts") }
       	   </div>
       }
   }
   ```

1. Customize the language and translations based on user preferences.

## 🔧 Props

| Name | Type | Description | Example | Default Value |
| --- | --- | --- | --- | --- |
| `supported_languages` | `Vec<&'static str>` | List of supported languages in your application. | `vec!["en", "fr", "de"]` | `vec!["en"]` |
| `translations` | `HashMap<String, serde_json::Value>` | Translations for different languages. | Refer to the usage examples for translations | An empty HashMap |

## 📙 Examples

If you're curious about how to use it with tailwind css, you can check out [the examples folder](examples/tailwind) for more information.

## 🤝 Contribution

We welcome contributions from the community to enhance this Yew I18n component. Feel free to open issues, submit pull requests, or provide feedback. Let's collaborate to make multilingual support in Yew even more powerful!

## 📜 License

Yew I18n is licensed under the `MIT` License, allowing you to use, modify, and distribute it freely. Refer to the [`LICENSE`](LICENSE) file for more details.