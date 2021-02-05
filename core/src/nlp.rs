use rust_bert::{
    pipelines::{
        ner::{NERModel},
        sentiment::{SentimentModel, SentimentPolarity},
        token_classification::{TokenClassificationConfig},
    }
};
use rust_bert::pipelines::sentiment::Sentiment;

const BASE_CONFIDENCE_SCORE: f64 = 0.24;

// Uses Named Entity Recognition to find possible Organizations in a given string
pub fn bert_organization_tokenization (input: &Vec<String>) -> Vec<Vec<String>> {
    let ner_model = NERModel::new(TokenClassificationConfig::default()).unwrap();
    let mut companies_list: Vec<Vec<String>> = vec![];
    for s in input {
        let output = ner_model.predict([s.as_str()]);
        let mut companies: Vec<String> = vec![];
        for entity in output.into_iter() {
            if entity.label == String::from("I-ORG") && entity.score > BASE_CONFIDENCE_SCORE {
                companies.push(String::from(entity.word));
            }
        }
        companies_list.push(companies)
    }
    companies_list
}

pub fn sentiment_classifier (input: &Vec<String>) -> Vec<SentimentPolarity> {
    let sentiment_classifier = SentimentModel::new(Default::default()).unwrap();
    let mut sentiment_scores: Vec<SentimentPolarity> = vec![];
    for s in input {
        let output: Vec<Sentiment> = sentiment_classifier.predict([s.as_str()]);
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
    }
    return sentiment_scores;
}
