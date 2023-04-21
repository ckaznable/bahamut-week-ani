use reqwest::Result;
use scraper::{ElementRef, Html, Selector};
use ui::run;

mod ui;

#[derive(Default, Clone)]
pub struct Day {
    pub anime: Vec<Anime>,
    pub week: String,
}

#[derive(Default, Clone)]
pub struct Anime {
    pub time: String,
    pub name: String,
    pub ep: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let html = get_html().await?;
    let doc = Html::parse_document(html.as_str());
    let root = doc.root_element();
    let mut days = get_days(&root);
    run(&mut days).ok();
    Ok(())
}

async fn get_html() -> Result<String> {
    let url = "https://ani.gamer.com.tw/";
    reqwest::get(url).await?.text().await
}

fn get_days(doc: &ElementRef) -> Vec<Day> {
    let selector = Selector::parse(".day-list").unwrap();
    doc.select(&selector)
        .map(|dom| {
            let selector = Selector::parse(".day-title").unwrap();
            let week = dom
                .select(&selector)
                .next()
                .unwrap()
                .text()
                .collect::<String>();

            Day {
                anime: get_anime_list(&dom),
                week,
            }
        })
        .collect::<Vec<Day>>()
}

fn get_anime_list(doc: &ElementRef) -> Vec<Anime> {
    let selector = Selector::parse(".text-anime-info").unwrap();
    let time_selector = Selector::parse(".text-anime-time").unwrap();
    let name_selector = Selector::parse(".text-anime-name").unwrap();
    let ep_selector = Selector::parse(".text-anime-number").unwrap();

    doc.select(&selector)
        .map(|dom| Anime {
            time: dom.select(&time_selector).next().unwrap().text().collect(),
            name: dom.select(&name_selector).next().unwrap().text().collect(),
            ep: dom.select(&ep_selector).next().unwrap().text().collect(),
        })
        .collect::<Vec<Anime>>()
}
