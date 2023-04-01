mod indice;
#[cfg(test)]
mod unit_test;
use flate2::read::MultiGzDecoder;
use indice::InvertedIndex;
use quick_xml::events::Event;
use quick_xml::Reader;
use search::collection::{Collection, Document};
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file =
        File::open("/home/georges/data_app_search/app-search-rust/enwiki-latest-abstract.xml.gz")
            .unwrap();
    let mut index = InvertedIndex::new();
    let mut collection = Collection::new();
    let mut flag_abs = false;
    let mut doc_id = 0;
    let bufreader = BufReader::new(file);
    let mgz = MultiGzDecoder::new(bufreader);
    let mut reader = Reader::from_reader(BufReader::new(mgz));
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                if let b"abstract" = e.name().as_ref() {
                    flag_abs = true;
                }
            }
            Ok(Event::End(ref e)) => {
                if let b"abstract" = e.name().as_ref() {
                    flag_abs = false;
                }
            }
            Ok(Event::Text(e)) => {
                let text = e.unescape().unwrap().into_owned();
                if text.len() >= 10 && flag_abs {
                    let doc = Document::init(doc_id + 1, text);
                    index.index_document(doc, &mut collection);
                    flag_abs = false;
                    doc_id += 1;
                    if collection.len() % 1000 == 0 {
                        println!("{}", collection.len());
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        if collection.len() == 10_000 {
            break;
        }
    }
    buf.clear();
    println!("{:#?}", collection.get_document(&20));
    println!("{}", collection.len());
}
