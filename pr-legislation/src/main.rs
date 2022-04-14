extern crate reqwest;
extern crate select;
use chrono::NaiveDate;
use futures::{stream, StreamExt};
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::io::Write;

const START_MEASURE: i32 = 53598;
const END_MEASURE: i32 = 71766;
const CONCURRENT_REQUESTS: usize = 5;

fn get_urls() -> Vec<String> {
    let string_list: Vec<String> = (START_MEASURE..START_MEASURE+1).map(|i| format!("http://www.tucamarapr.org/dnncamara/web/ActividadLegislativa/TramiteLegislativo.aspx?measureid={}", i)).collect();
    return string_list;
}

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

#[tokio::main]
async fn main() {
    scrape().await;
}

async fn scrape() {
    let client = Client::new();
    let urls = get_urls();
    println!("{:?}", urls);
    let bodies = stream::iter(urls)
        .map(|url| {
            let client = &client;
            async move {
                let resp = client.get(url).send().await?;
                resp.bytes().await
            }
        })
        .buffer_unordered(CONCURRENT_REQUESTS);
    let measures = bodies
        .fold(Vec::<Measure>::new(), |mut measures, b| async move {
            match b {
                Ok(b) => {
                    let commision_bold_span = Selector::parse(".FntResize").unwrap();
                    let header_select = Selector::parse(".Description-Justify > label").unwrap();
                    let table_select = Selector::parse(
                        ".measures-table > tbody > tr:not(.customheader):not(.separator)",
                    )
                    .unwrap();
                    let th_select = Selector::parse("th").unwrap();
                    let td_select = Selector::parse("td").unwrap();
                    let doc_select = Selector::parse("td > a").unwrap();
                    let fragment = Html::parse_document(&String::from_utf8_lossy(&b));

                    let mut measure_name = "".to_string();
                    let mut measure_date = "".to_string();
                    let mut authors = Vec::<String>::new();
                    let mut history = Vec::<History>::new();

                    for fnt_resize in fragment.select(&commision_bold_span) {
                        let story_txt = fnt_resize.text().collect::<Vec<_>>();
                        let mut measure_name_idx = 0;
                        let mut date_idx = 0;
                        let mut author_idx_start = 0;
                        let mut author_idx_end = 0;
                        for (i, story_part) in story_txt.iter().enumerate() {
                            if story_part.contains("Nombre") {
                                measure_name_idx = i + 1;
                            }
                            if story_part.contains("Fecha Radicada") {
                                date_idx = i + 1;
                            }
                            if story_part.contains("Autor:") {
                                author_idx_start = i + 1;
                            }
                            if author_idx_start > 0 && author_idx_end == 0 {
                                let author_text = story_part.replace("\n", "");
                                if author_text.trim_start().trim_end().len() == 0 {
                                    author_idx_end = i - 1;
                                }
                            }
                        }
                        if measure_name_idx > 0 && measure_name.chars().count() == 0 {
                            measure_name = story_txt[measure_name_idx].replace("\n", "");
                        }
                        if date_idx > 0 && measure_date.chars().count() == 0 {
                            measure_date = story_txt[date_idx].replace("\n", "");
                        }
                        if author_idx_start > 0 && author_idx_end > 0 && authors.len() == 0 {
                            let authors_docs = &story_txt[author_idx_start..author_idx_end];
                            for author in authors_docs {
                                let a = author.replace("\n", "");
                                authors.push(a.trim_start().trim_end().to_string())
                            }
                        }
                    }

                    let mut header = "".to_string();
                    for head_label in fragment.select(&header_select) {
                        header = head_label
                            .inner_html()
                            .replace("\n", "")
                            .trim_start()
                            .trim_end()
                            .to_string();
                    }
                    for table_li in fragment.select(&table_select) {
                        let mut d_string = "".to_string();
                        let xs = table_li.select(&th_select);
                        for txs in xs {
                            d_string = txs
                                .inner_html()
                                .replace("\n", "")
                                .trim_start()
                                .trim_end()
                                .to_string();
                            break;
                        }

                        let mut desc_string = "".to_string();
                        let tds = table_li.select(&td_select);
                        for tdds in tds {
                            desc_string = tdds
                                .inner_html()
                                .replace("\n", "")
                                .trim_start()
                                .trim_end()
                                .to_string();
                            break;
                        }

                        let mut doc_str = "".to_string();
                        let tdocs = table_li.select(&doc_select);

                        for tdoc in tdocs {
                            let href = tdoc.value().attr("href");
                            match href {
                                Some(b) => doc_str = "http://www.tucamarapr.org".to_owned() + b,
                                None => (),
                            }
                            break;
                        }

                        history.push(History {
                            description: desc_string,
                            date: NaiveDate::parse_from_str(d_string.as_str(), "%m/%d/%Y").unwrap(),
                            document: doc_str,
                        })
                    }
                    measures.push(Measure {
                        name: measure_name.trim_start().trim_end().to_string(),
                        date: NaiveDate::parse_from_str(
                            measure_date.trim_start().trim_end(),
                            "%m/%d/%Y",
                        )
                        .unwrap(),
                        heading: header,
                        authors,
                        history,
                    })
                }
                Err(e) => eprintln!("Got an error: {}", e),
            }
            measures
        })
        .await;
    let measure_json = serde_json::to_string(&measures).unwrap();
    let mut file = std::fs::File::create("measures.es.json").unwrap();
    writeln!(&mut file, "{}", measure_json.as_str()).unwrap();
}
