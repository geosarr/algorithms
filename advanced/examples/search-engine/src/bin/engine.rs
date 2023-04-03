use search::loader::Loader;

use clap::Parser;

#[derive(Parser)]
#[command(about = "\nSearches among english Wikipedia abstracts.")]
struct Cli {
    /// Absolute path to the file with the integers of the problem
    #[arg(short, long)]
    file_abs_path: String,
}

fn main() {
    let cli = Cli::parse();
    let loader = Loader::init(cli.file_abs_path);
    loader.load();
}
// mod indice;
// #[cfg(test)]
// mod unit_test;
// use flate2::read::MultiGzDecoder;
// use indice::InvertedIndex;
// use quick_xml::events::Event;
// use quick_xml::Reader;
// use search::collection::{Collection, Document};
// use std::fs::File;
// use std::io::BufReader;

// use clap::Parser;

// #[derive(Parser)]
// #[command(about = "\nSearches among english Wikipedia abstracts.")]
// struct Cli {
//     /// Absolute path to the file with the integers of the problem
//     #[arg(short, long)]
//     file_abs_path: String,
// }

// fn main() {
//     let cli = Cli::parse();
//     let file = File::open(cli.file_abs_path).unwrap();
//     let mut index = InvertedIndex::new();
//     let mut collection = Collection::new();
//     let mut flag_abs = false;
//     let mut doc_id = 0;
//     let bufreader = BufReader::new(file);
//     let mgz = MultiGzDecoder::new(bufreader);
//     let mut reader = Reader::from_reader(BufReader::new(mgz));
//     let mut buf = Vec::new();
//     loop {
//         match reader.read_event_into(&mut buf) {
//             Ok(Event::Start(ref e)) => {
//                 if let b"abstract" = e.name().as_ref() {
//                     flag_abs = true;
//                 }
//             }
//             Ok(Event::End(ref e)) => {
//                 if let b"abstract" = e.name().as_ref() {
//                     flag_abs = false;
//                 }
//             }
//             Ok(Event::Text(e)) => {
//                 let text = e.unescape().unwrap().into_owned();
//                 if text.len() >= 10 && flag_abs {
//                     let doc = Document::init(doc_id + 1, text);
//                     index.index_document(doc, &mut collection);
//                     flag_abs = false;
//                     doc_id += 1;
//                     if collection.len() % 1000 == 0 {
//                         println!("{}", collection.len());
//                     }
//                 }
//             }
//             Ok(Event::Eof) => break,
//             Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
//             _ => (),
//         }
//         if collection.len() == 10 {
//             break;
//         }
//     }
//     buf.clear();
//     println!("{:#?}", collection.get_document(&5));
//     println!("{}", collection.len());
//     println!("{:#?}", index.index());
//     println!("{:#?}", index.raw_freq());
//     println!("{:#?}", collection.get_document(&9));
//     println!("{:#?}", collection.get_document(&8));
//     println!("{:#?}", collection.get_document(&5));
//     println!("{:#?}", collection.get_document(&4));
// }
