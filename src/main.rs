use scraper::{ElementRef, Html, Selector};
use std::fmt;

const BASE_URL: &str = "https://autoselect.ru/auto/?&set_filter=Показать&n123=&arrFilter_118_848981442=Y&arrFilter_90_MIN=&arrFilter_90_MAX=&arrFilter_67_2645610321=Y&arrFilter_84_MIN=&arrFilter_84_MAX=&arrFilter_85_MIN=&arrFilter_85_MAX=&arrFilter_55=4088188550&arrFilter_56_MIN=&arrFilter_56_MAX=&arrFilter_67_2645610321=Y&arrFilter_67_511942527=Y&arrFilter_67_2274021061=Y&arrFilter_66_3808539628=Y&arrFilter_66_174200537=Y&arrFilter_59_MIN=&arrFilter_59_MAX=";

#[derive(Debug)]
struct Item {
    name: Option<String>,
    info: Option<String>,
    price: Option<String>,
    relative_url: Option<String>,
}

impl Item {
    fn price(&self) -> Option<u64> {
        self.price.as_deref().and_then(parse_price)
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.name {
            writeln!(f, "{}", name)?;
        }
        if let Some(info) = &self.info {
            writeln!(f, "{}", info)?;
        }
        if let Some(price) = &self.price {
            writeln!(f, "{}", price)?;
        }
        if let Some(url) = &self.relative_url {
            writeln!(f, "https://autoselect.ru{}", url)?;
        }
        Ok(())
    }
}

fn main() -> Result<(), main_error::MainError> {
    let selectors = Selectors::new();
    let document = get_html(BASE_URL)?;
    let mut items = scrape_items(&document, &selectors);
    items.sort_by_key(|item| item.price());
    for item in &items {
        println!("{}", item);
    }
    Ok(())
}

fn get_html(url: &str) -> Result<Html, main_error::MainError> {
    let resp = minreq::get(url).with_timeout(10).send()?;
    let html = resp.as_str()?;
    Ok(Html::parse_document(html))
}

fn scrape_items(document: &Html, selectors: &Selectors) -> Vec<Item> {
    let mut items: Vec<Item> = Vec::new();
    let mut n: u32 = 1;
    while let Some(element) = document.select(&item_selector(n)).next() {
        items.push(Item {
            name: element
                .select(&selectors.link)
                .next()
                .map(|el| prettify(el, " ")),
            info: element
                .select(&selectors.info)
                .next()
                .map(|el| prettify(el, "")),
            price: element
                .select(&selectors.price)
                .next()
                .map(|el| prettify(el, " ")),
            relative_url: element
                .select(&selectors.link)
                .next()
                .and_then(|el| el.value().attr("href").map(String::from)),
        });
        n += 1
    }
    items
}

fn parse_price(s: &str) -> Option<u64> {
    s.chars()
        .take_while(|ch| ch.is_digit(10) || ch.is_whitespace())
        .filter(|ch| ch.is_digit(10))
        .collect::<String>()
        .parse()
        .ok()
}

fn prettify(element: ElementRef, separator: &str) -> String {
    element
        .text()
        .map(|s| s.trim())
        .collect::<Vec<_>>()
        .join(separator)
}

struct Selectors {
    link: Selector,
    info: Selector,
    price: Selector,
}

impl Selectors {
    fn new() -> Self {
        Selectors {
            link: Selector::parse("div.item__content > h4 > a").unwrap(),
            info: Selector::parse("div.item__content > div.item__info").unwrap(),
            price: Selector::parse("div.item__content > div.item__price-row > div").unwrap(),
        }
    }
}

fn item_selector(n: u32) -> Selector {
    let selector = format!("body > div.wrap-main > section.content-main > div > div.catalog > div > div:nth-child({}) > div", n);
    Selector::parse(&selector).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::{assert_debug_snapshot, assert_display_snapshot};

    #[test]
    fn test_parse_price() {
        assert_eq!(parse_price("2 550 000 руб."), Some(2550000));
    }

    #[test]
    fn scrape_single_page() {
        let selectors = Selectors::new();
        let html = include_str!("../fixtures/single_page.html");
        let document = Html::parse_document(html);
        let items = scrape_items(&document, &selectors);
        assert_debug_snapshot!(items);
    }

    #[test]
    fn display_item() {
        let selectors = Selectors::new();
        let html = include_str!("../fixtures/single_page.html");
        let document = Html::parse_document(html);
        let item = scrape_items(&document, &selectors)
            .into_iter()
            .next()
            .unwrap();
        assert_display_snapshot!(item)
    }
}
