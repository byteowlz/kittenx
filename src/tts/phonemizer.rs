use anyhow::Result;
use espeak_rs::text_to_phonemes;
use whatlang::{detect, Lang};

pub fn detect_language(text: &str) -> Option<String> {
    if let Some(info) = detect(text) {
        let lang_code = match info.lang() {
            Lang::Eng => "en-us",
            Lang::Spa => "es",
            Lang::Fra => "fr",
            Lang::Deu => "de",
            Lang::Ita => "it",
            Lang::Por => "pt",
            Lang::Rus => "ru",
            Lang::Jpn => "ja",
            Lang::Kor => "ko",
            Lang::Cmn => "zh",
            _ => "en-us",
        };
        Some(lang_code.to_string())
    } else {
        None
    }
}

pub fn text_to_phonemes_simple(text: &str, language: &str) -> Result<String> {
    // Simple tokenization for basic phonemization
    let tokens = basic_tokenize(text);
    let phonemes_str = tokens.join(" ");
    
    // Use espeak for phonemization if available, otherwise fallback to simple processing
    match text_to_phonemes(text, language, None, true, false) {
        Ok(phonemes) => Ok(phonemes.join("")),
        Err(_) => {
            // Fallback: simple character-based processing
            Ok(phonemes_str)
        }
    }
}

pub fn basic_tokenize(text: &str) -> Vec<String> {
    use regex::Regex;
    let re = Regex::new(r"\w+|[^\w\s]").unwrap();
    re.find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect()
}