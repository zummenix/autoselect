use scraper::{Html, Selector};

const BASE_URL: &str = "https://autoselect.ru/auto/?&set_filter=Показать&n123=&arrFilter_118_848981442=Y&arrFilter_90_MIN=&arrFilter_90_MAX=&arrFilter_67_2645610321=Y&arrFilter_84_MIN=&arrFilter_84_MAX=&arrFilter_85_MIN=&arrFilter_85_MAX=&arrFilter_55=4088188550&arrFilter_56_MIN=&arrFilter_56_MAX=&arrFilter_67_2645610321=Y&arrFilter_67_511942527=Y&arrFilter_67_2274021061=Y&arrFilter_66_3808539628=Y&arrFilter_66_174200537=Y&arrFilter_59_MIN=&arrFilter_59_MAX=";

fn main() -> Result<(), main_error::MainError> {
    let resp = minreq::get(BASE_URL).with_timeout(10).send()?;
    let html = resp.as_str()?;
    let document = Html::parse_document(html);
    let mut n: u32 = 1;
    while let Some(element) = document.select(&item(n)).next() {
        let name = element.select(&item_link()).next().unwrap().inner_html();
        println!("{}", name.trim());
        let info = element
            .select(&item_info())
            .next()
            .unwrap()
            .text()
            .map(|s| s.trim())
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", info.trim());
        let price = element
            .select(&item_price())
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", price.trim());
        let link = element
            .select(&item_link())
            .next()
            .unwrap()
            .value()
            .attr("href");
        println!("https://autoselect.ru{}", link.unwrap());
        println!("");
        n += 1
    }
    Ok(())
}

fn item(n: u32) -> Selector {
    let selector = format!("body > div.wrap-main > section.content-main > div > div.catalog > div > div:nth-child({}) > div", n);
    Selector::parse(&selector).unwrap()
}

fn item_link() -> Selector {
    Selector::parse("div.item__content > h4 > a").unwrap()
}

fn item_info() -> Selector {
    Selector::parse("div.item__content > div.item__info").unwrap()
}

fn item_price() -> Selector {
    Selector::parse("div.item__content > div.item__price-row > div").unwrap()
}
