use rust_bert::m2m_100::{
    M2M100ConfigResources, M2M100MergesResources, M2M100ModelResources, M2M100SourceLanguages,
    M2M100TargetLanguages, M2M100VocabResources,
};
use rust_bert::pipelines::common::ModelType;
use rust_bert::pipelines::translation::{Language, TranslationConfig, TranslationModel};
use rust_bert::resources::RemoteResource;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use tch::Device;
#[derive(Serialize, Deserialize, Debug)]
struct History {
    description: String,
    date: chrono::naive::NaiveDate,
    document: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Measure {
    name: String,
    date: chrono::naive::NaiveDate,
    heading: String,
    authors: Vec<String>,
    history: Vec<History>,
}
fn main() {
    let model_resource = RemoteResource::from_pretrained(M2M100ModelResources::M2M100_418M);
    let config_resource = RemoteResource::from_pretrained(M2M100ConfigResources::M2M100_418M);
    let vocab_resource = RemoteResource::from_pretrained(M2M100VocabResources::M2M100_418M);
    let merges_resource = RemoteResource::from_pretrained(M2M100MergesResources::M2M100_418M);

    let source_languages = M2M100SourceLanguages::M2M100_418M;
    let target_languages = M2M100TargetLanguages::M2M100_418M;

    let translation_config = TranslationConfig::new(
        ModelType::M2M100,
        model_resource,
        config_resource,
        vocab_resource,
        merges_resource,
        source_languages,
        target_languages,
        Device::Cpu,
    );
    let model_r = TranslationModel::new(translation_config);
    let model;
    if let Ok(t) = model_r {
        model = t
    } else {
        return;
    }
    let contents = fs::read_to_string("../pr-legislation/measures.json")
        .expect("Something went wrong reading the file");
    let measures: Vec<Measure> = match serde_json::from_str(&contents) {
        Ok(it) => it,
        Err(_e) => Vec::<Measure>::new(),
    };

    let mut en_measures = Vec::<Measure>::new();
    for m in measures {
        let mut vs = Vec::<String>::new();
        vs.push(m.heading);

        let output_r = model.translate(&vs, Language::Spanish, Language::English);

        let output: Vec<String>;
        match output_r {
            Ok(t) => output = t,
            Err(e) => {
                print!("{}", e);
                return;
            }
        }
        let en_header = output.iter().next().expect("error");

        en_measures.push(Measure {
            name: m.name,
            date: m.date,
            heading: en_header.to_string(),
            authors: m.authors,
            history: m.history,
        })
    }
    let measure_json = serde_json::to_string(&en_measures).unwrap();
    let mut file = std::fs::File::create("../pr-legislation/measures.en.json").unwrap();
    writeln!(&mut file, "{}", measure_json.as_str()).unwrap();
}
