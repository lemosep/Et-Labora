use std::fs::File;
use std::io::Write;

use scraper::{Html, Selector};
use ureq;

const PATH: &str = "./data/points.txt"; 
const BASE_URL: &str = "https://escriva.org/pt-br/";

static CONTROL: &[(&str, u32)] = &[("camino", 999), ("surco", 1000), ("forja", 1055)];

struct Point {
    book: String,
    chapter: String,
    text: String,
    number: u16,
    tags: Vec<String>,
}

fn get_page_content(url: &str) -> Result<String, ureq::Error> {
    let content = ureq::get(&url).call()?.into_string()?;
    Ok(content)
}

fn get_point_chapter(document: &Html) -> String {
    let mut chapter = String::new();

    let chapter_selector = Selector::parse(".pre-destacado > a").unwrap();
    
    for node in document.select(&chapter_selector) {
        chapter = node.text().collect();
    };

    chapter
}


fn get_point_tags(document: &Html) -> Vec<String> {
    let mut tags = Vec::<String>::new();

    let tag_selector = Selector::parse(".subjects > a").unwrap();

    for node in document.select(&tag_selector) {
        tags.push(node.text().collect());
    }

    tags
}



fn get_point_data(document: &Html) -> Point {
    let point_selector = Selector::parse("div.imperavi-body").unwrap();
    let point = document.select(&point_selector);

    let mut current_point = Point {
        book: String::new(),
        chapter: String::new(),
        number: 0,
        text: String::new(),
        tags: Vec::new(),
    };

    for node in point {
        let number_selector = Selector::parse("h1").unwrap();
        let text_selector = Selector::parse("p").unwrap();

        if let Some(number) = node.select(&number_selector).next() {
            current_point.number = number.text().collect::<String>().parse().unwrap();
        }

        if let Some(text) = node.select(&text_selector).next() {
            current_point.text = text.text().collect();
        }
    }

    current_point.tags = get_point_tags(&document);
    current_point.chapter = get_point_chapter(&document);

    current_point
}


fn main() {
    let mut point_vec = Vec::<Point>::new();
    let mut file = File::create(&PATH).expect("Error: unable to build file");

    for &(book, max_point) in CONTROL {

        for point in 1..=max_point {
            //Build url to specified book and point
            let url = format!("{BASE_URL}/{book}/{point}");

            //Obtain html page content
            let document = Html::parse_document(
                &get_page_content(&url).unwrap()
            );
            
            //Parsing specific div
            let mut point = get_point_data(&document);

            point.book = (&book).to_string();

            writeln!(&mut file, "{}", point.text).unwrap();
            writeln!(&mut file, "{}", point.chapter).unwrap();

            point_vec.push(point);
        }
    }
}