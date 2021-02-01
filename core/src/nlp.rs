use rust_bert::pipelines::ner::{NERModel};
use rust_bert::pipelines::token_classification::TokenClassificationConfig;

const BASE_CONFIDENCE_SCORE: f64 = 0.24;

// Uses Named Entity Recognition to find possible Organizations in a given string
pub fn bert_organization_tokenization (input: &Vec<String>) -> Vec<Vec<String>> {
    let ner_model = NERModel::new(TokenClassificationConfig::default()).unwrap();
    let mut companies_list: Vec<Vec<String>> = vec![];
    for s in input.into_iter() {
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
