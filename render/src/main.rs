use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
use std::io::Write;
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
#[derive(TemplateOnce)]
#[template(path = "../templates/measures.stpl")]
struct MeasureTemplate<'a> {
    measure: &'a Measure,
}

#[derive(TemplateOnce)]
#[template(path = "../templates/measures_md.stpl")]
struct MeasureMDTemplate<'a> {
    measure: &'a Measure,
}
fn main() {
    let en_contents = fs::read_to_string("../output/measures.en.json")
        .expect("Something went wrong reading the en file");
    let es_contents = fs::read_to_string("../output/measures.es.json")
        .expect("Something went wrong reading the es file");

    let en_measure_result: Result<Vec<Measure>> = serde_json::from_str(&en_contents);
    let en_measures = en_measure_result.unwrap();

    let es_measure_result: Result<Vec<Measure>> = serde_json::from_str(&es_contents);
    let es_measures = es_measure_result.unwrap();

    for i in 0..en_measures.len() {
        let en_ctx = MeasureTemplate {
            measure: &en_measures[i],
        };
        let es_ctx = MeasureTemplate {
            measure: &es_measures[i],
        };
        let en_ctx_md = MeasureMDTemplate {
            measure: &en_measures[i],
        };
        let es_ctx_md = MeasureMDTemplate {
            measure: &es_measures[i],
        };
        let file_name = &en_measures[i].name;
        let mut file =
            std::fs::File::create(format!("../output/measures/{}.en.html", file_name)).unwrap();
        writeln!(&mut file, "{}", en_ctx.render_once().unwrap()).unwrap();

        file = std::fs::File::create(format!("../output/measures/{}.es.html", file_name)).unwrap();
        writeln!(&mut file, "{}", es_ctx.render_once().unwrap()).unwrap();

        file = std::fs::File::create(format!("../output/measures/{}.en.md", file_name)).unwrap();
        writeln!(&mut file, "{}", en_ctx_md.render_once().unwrap()).unwrap();

        file = std::fs::File::create(format!("../output/measures/{}.es.md", file_name)).unwrap();
        writeln!(&mut file, "{}", es_ctx_md.render_once().unwrap()).unwrap();
    }
}
