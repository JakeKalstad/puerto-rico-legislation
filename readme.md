# PR LEGISLATION

Making the legislation process in puerto rico more transparent 

## Scrape legislation past and present

<http://www.tucamarapr.org/dnncamara/web/ActividadLegislativa/tramitelegislativo.aspx?measureid=XXXX>

contents to be retrieved and stored as JSON files to be processed later
save as documents/{measure_name}.es.json

    measure {
        Measure Name :: string
        Date Filed :: date
        Authors :: string[]
        Heading :: string
        History :: History[]
    }

    History {
        Date :: date
        Description :: string 
        Document :: string (url)
    }

## download all associated documents

save contents to folders documents/{measure_name}/{history_date}.{history_description}.pdf

## Scrape vote date ??? how to find vote IDs? aspx sucks

<http://www.tucamarapr.org/dnncamara/web/ActividadLegislativa/votaciones.aspx?measureid=XXX&voteid=???>

## translation

translate documents/{measure_name}.es.json into documents/{measure_name}.en.json

## create table html

index page for es/en
filter by measure id/heading substring/authors

## generate html for each measure

convert json files into static html and md files for es/en

## gotchas

Translation is currently using a local build of rust-bert

To get it to work I updated openssl to 3.0 via the experimental ubuntu repo
downloaded from `https://download.pytorch.org/libtorch/cu113/libtorch-shared-with-deps-1.11.0%2Bcu113.zip` `libtorch-cxx11-abi-shared-with-deps-1.11.0+cu113.zip` and extracted it locally, pointed LIBTORCH at it (following instructions via rust-bert)
cloned rust-bert and built it via `cargo build` and pointed my translate cargo.toml to the extracted directory

It's currently running on CPU for the translation, my GPU is small otherwise it would be better to run it off GPU.

@inproceedings{becquin-2020-end,
    title = "End-to-end {NLP} Pipelines in Rust",
    author = "Becquin, Guillaume",
    booktitle = "Proceedings of Second Workshop for NLP Open Source Software (NLP-OSS)",
    year = "2020",
    publisher = "Association for Computational Linguistics",
    url = "<https://www.aclweb.org/anthology/2020.nlposs-1.4">,
    pages = "20--25",
}
