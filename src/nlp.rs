use rust_bert::pipelines::ner::{NERModel};
use rust_bert::pipelines::token_classification::TokenClassificationConfig;

const BASE_CONFIDENCE_SCORE: f64 = 0.24;

pub fn bert_organization_tokenization (input: &str) -> Vec<String> {
    let ner_model = NERModel::new(TokenClassificationConfig::default()).unwrap();
    println!("{}", input);
    let output = ner_model.predict([input]);
    let mut companies: Vec<String> = vec![];
    for entity in output.into_iter() {
        if entity.label == String::from("I-ORG") && entity.score > BASE_CONFIDENCE_SCORE {
            companies.push(String::from(entity.word));
        }
    }
    companies
}
