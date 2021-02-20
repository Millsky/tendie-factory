use rust_bert::{
    pipelines::{
        ner::{NERModel},
        sentiment::{SentimentModel, SentimentPolarity},
    }
};
use rust_bert::pipelines::sentiment::Sentiment;
use std::sync::{Mutex};
use once_cell::sync::Lazy;

const BASE_CONFIDENCE_SCORE: f64 = 0.24;

// Lazy Init Models and allow them to be shared SAFELY Between Threads
pub static NER_MODEL: Lazy<Mutex<NERModel>> =
    Lazy::new(||
        Mutex::new(NERModel::new(Default::default()).unwrap()));

pub static SENTIMENT_MODEL: Lazy<Mutex<SentimentModel>> =
    Lazy::new(||
        Mutex::new(SentimentModel::new(Default::default()).unwrap()));


// Uses Named Entity Recognition to find possible Organizations in a given string
pub fn bert_organization_tokenization (input: String) -> Vec<String> {
    let mut companies_list: Vec<String> = vec![];
    let model = NER_MODEL.lock().unwrap();
    let output = model.predict([input.as_str()]);
    for entity in output.into_iter() {
        if entity.label == String::from("I-ORG") && entity.score > BASE_CONFIDENCE_SCORE {
            companies_list.push(String::from(entity.word));
        }
    }
    companies_list
}


pub fn sentiment_classifier (input: &str) -> Vec<SentimentPolarity> {
    let sentiment_classifier = SENTIMENT_MODEL.lock().unwrap();
    let mut sentiment_scores: Vec<SentimentPolarity> = vec![];
    let output: Vec<Sentiment> = sentiment_classifier.predict([input]);
    let scores: Vec<SentimentPolarity> = output.into_iter().map(|x| {
        x.polarity
    }).collect();
    match scores[0] {
        SentimentPolarity::Positive => {
            sentiment_scores.push(SentimentPolarity::Positive);
        },
        SentimentPolarity::Negative => {
            sentiment_scores.push(SentimentPolarity::Negative);
        }
    }
    return sentiment_scores;
}

