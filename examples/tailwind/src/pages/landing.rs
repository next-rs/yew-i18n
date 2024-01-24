use crate::components::landing::Trending;
use std::collections::HashMap;
use yew::prelude::*;
use yew_i18n::I18nProvider;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    let mut translations = HashMap::new();

    translations.insert(
        "en".to_string(),
        serde_json::json!({
            "24 Apr, 2023": "24 Apr, 2023",
            "02 May, 2023": "02 May, 2023",
            "11 May, 2023": "11 May, 2023",
            "Trending Posts": "Trending Posts",
            "Rust: The Next Big Thing in Data Science": "Rust: The Next Big Thing in Data Science",
            "Data Science": "Data Science",
            "The Ultimate Ndarray Handbook: Mastering the Art of Scientific Computing with Rust": "The Ultimate Ndarray Handbook: Mastering the Art of Scientific Computing with Rust",
            "Rust Polars: Unlocking High-Performance Data Analysis — Part 1": "Rust Polars: Unlocking High-Performance Data Analysis — Part 1",
        }),
    );

    translations.insert(
        "fr".to_string(),
        serde_json::json!({
            "24 Apr, 2023": "24 Avr, 2023",
            "02 May, 2023": "02 Mai, 2023",
            "11 May, 2023": "11 Mai, 2023",
            "Trending Posts": "Articles Tendances",
            "Rust: The Next Big Thing in Data Science": "Rust : La Prochaine Grande Avancée en Science des Données",
            "Data Science": "Science des Données",
            "The Ultimate Ndarray Handbook: Mastering the Art of Scientific Computing with Rust": "Le Manuel Ultime Ndarray : Maîtriser l'Art du Calcul Scientifique avec Rust",
            "Rust Polars: Unlocking High-Performance Data Analysis — Part 1": "Rust Polars : Libérer l'Analyse de Données Haute Performance — Partie 1",
        }),
    );

    translations.insert(
        "de".to_string(),
        serde_json::json!({
            "24 Apr, 2023": "24 Apr, 2023",
            "02 May, 2023": "02 Mai, 2023",
            "11 May, 2023": "11 Mai, 2023",
            "Trending Posts": "Trending Beiträge",
            "Rust: The Next Big Thing in Data Science": "Rust: Die nächste große Sache in der Datenwissenschaft",
            "Data Science": "Datenwissenschaft",
            "The Ultimate Ndarray Handbook: Mastering the Art of Scientific Computing with Rust": "Das ultimative Ndarray-Handbuch: Die Kunst des wissenschaftlichen Rechnens mit Rust meistern",
            "Rust Polars: Unlocking High-Performance Data Analysis — Part 1": "Rust Polars: Freischalten der High-Performance-Datenanalyse — Teil 1",
        }),
    );

    translations.insert(
        "es".to_string(),
        serde_json::json!({
            "24 Apr, 2023": "24 Abr, 2023",
            "02 May, 2023": "02 May, 2023",
            "11 May, 2023": "11 May, 2023",
            "Trending Posts": "Publicaciones Destacadas",
            "Rust: The Next Big Thing in Data Science": "Rust: La Próxima Gran Novedad en Ciencia de Datos",
            "Data Science": "Ciencia de Datos",
            "The Ultimate Ndarray Handbook: Mastering the Art of Scientific Computing with Rust": "El Manual Definitivo de Ndarray: Dominando el Arte de la Computación Científica con Rust",
            "Rust Polars: Unlocking High-Performance Data Analysis — Part 1": "Rust Polars: Desbloqueando el Análisis de Datos de Alto Rendimiento — Parte 1",
        }),
    );

    html! {
        <I18nProvider
            supported_languages={vec!["en", "fr", "de", "es"]}
            translations={translations}
        ><Trending /></I18nProvider>
    }
}
