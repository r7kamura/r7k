use crate::result::Result;
use pulldown_cmark::{html, Event, Options, Parser, Tag};
use scraper::{Html, Selector};
use serde::Deserialize;

pub fn parse(content: &str) -> Result<Data> {
    let result = fronma::parser::parse::<Headers>(content)?;
    let html_body = parse_markdown(result.body);
    let title = if result.headers.title == "~" {
        "".to_string()
    } else {
        result.headers.title
    };
    let summary = extract_summary(&html_body);
    let image_url = extract_image_url(&html_body);
    Ok(Data {
        html_body,
        image_url,
        summary,
        title,
    })
}

#[derive(Debug)]
pub struct Data {
    pub title: String,
    pub html_body: String,
    pub image_url: Option<String>,
    pub summary: Option<String>,
}

#[derive(Deserialize)]
struct Headers {
    title: String,
}

fn parse_markdown(content: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(content, options);
    let parser = parser.map(|event| match event {
        Event::Start(Tag::Link(link_type, url, title)) => {
            let url = if url.as_ref().starts_with("https://www.amazon.co.jp/dp/") {
                format!(
                    "{url}?tag={tracking_id}",
                    url = url,
                    tracking_id = "r7kamuracom-22"
                )
                .into()
            } else {
                url
            };
            Event::Start(Tag::Link(link_type, url, title))
        }
        _ => event,
    });
    let mut string = String::new();
    html::push_html(&mut string, parser);
    string
}

fn truncate(str: &str, max_characters_count: usize) -> &str {
    match str.char_indices().nth(max_characters_count) {
        None => str,
        Some((index, _)) => &str[..index],
    }
}

fn extract_summary(html: &str) -> Option<String> {
    let selector = Selector::parse("* > p").unwrap();
    let fragment = Html::parse_fragment(html);
    for element in fragment.select(&selector) {
        let texts: Vec<_> = element.text().collect();
        let inner = texts.join("");
        if !inner.is_empty() {
            let str = inner.split_inclusive("。").next().unwrap();
            let truncated = truncate(str, 140);
            return Some(truncated.to_string());
        }
    }
    None
}

fn extract_image_url(html: &str) -> Option<String> {
    let fragment = Html::parse_fragment(html);
    let selector = Selector::parse("img[src]").unwrap();
    fragment
        .select(&selector)
        .next()
        .map(|element| element.value().attr("src").unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use super::{extract_image_url, extract_summary, parse};

    #[test]
    fn parse_returns_error_to_empty_content() {
        let content = "";
        let result = parse(content);
        assert!(result.is_err());
    }

    #[test]
    fn parse_returns_error_to_no_title_content() {
        let content = "---\n---\nbody\n";
        let result = parse(content);
        assert!(result.is_err());
    }

    #[test]
    fn parse_works() {
        let content = "---\ntitle: title\n---\nこんにちは。![](http://example.com/image.jpg)\n";
        let result = parse(content);
        let data = result.unwrap();
        assert_eq!(data.title, "title".to_string());
        assert_eq!(
            data.html_body,
            "<p>こんにちは。<img src=\"http://example.com/image.jpg\" alt=\"\" /></p>\n"
                .to_string()
        );
        assert_eq!(data.summary, Some("こんにちは。".to_string()));
        assert_eq!(
            data.image_url,
            Some("http://example.com/image.jpg".to_string())
        );
    }

    #[test]
    fn parse_works_to_empty_title_content() {
        let content = "---\ntitle:\n---\nbody\n";
        let result = parse(content);
        let data = result.unwrap();
        assert_eq!(data.title, "".to_string());
        assert_eq!(data.html_body, "<p>body</p>\n".to_string());
    }

    #[test]
    fn parse_appends_tracking_id_to_amazon_link() {
        let content = "---\ntitle:\n---\n[link](https://www.amazon.co.jp/dp/B07L5J1LY9)\n";
        let result = parse(content);
        let data = result.unwrap();
        assert_eq!(data.title, "".to_string());
        assert_eq!(data.html_body, "<p><a href=\"https://www.amazon.co.jp/dp/B07L5J1LY9?tag=r7kamuracom-22\">link</a></p>\n".to_string());
    }

    #[test]
    fn extract_summary_returns_none_to_empty_html() {
        let html = "";
        let option = extract_summary(html);
        assert!(option.is_none());
    }

    #[test]
    fn extract_summary_returns_summary_to_simple_paragraph() {
        let html = "<p>a b c.</p>";
        let option = extract_summary(html);
        let summary = option.unwrap();
        assert_eq!(summary, "a b c.");
    }

    #[test]
    fn extract_summary_returns_summary_to_linked_paragraph() {
        let html = "<p><a href=''>a</a> b c.</p>";
        let option = extract_summary(html);
        let summary = option.unwrap();
        assert_eq!(summary, "a b c.");
    }

    #[test]
    fn extract_summary_returns_summary_to_multiple_japanese_sentences() {
        let html = "<p>あ。い。</p>";
        let option = extract_summary(html);
        let summary = option.unwrap();
        assert_eq!(summary, "あ。");
    }

    #[test]
    fn extract_image_url_returns_ok_when_single_img_exists() {
        let html = r#"<p><img src="http://example.com/image.jpg"></p>"#;
        let option = extract_image_url(html);
        let image_url = option.unwrap();
        assert_eq!(image_url, "http://example.com/image.jpg");
    }

    #[test]
    fn extract_image_url_returns_image_url_when_single_img_exists() {
        let html = r#"<p><img src="http://example.com/image.jpg"></p>"#;
        let option = extract_image_url(html);
        let image_url = option.unwrap();
        assert_eq!(image_url, "http://example.com/image.jpg");
    }

    #[test]
    fn extract_image_url_returns_first_image_url_when_multi_img_exists() {
        let html = r#"<p><img src="http://example.com/image1.jpg"></p><p><img src="http://example.com/image2.jpg"></p>"#;
        let option = extract_image_url(html);
        let image_url = option.unwrap();
        assert_eq!(image_url, "http://example.com/image1.jpg");
    }

    #[test]
    fn extract_image_url_returns_none_when_no_img_exists() {
        let html = "<p>a</p>";
        let option = extract_image_url(html);
        assert!(option.is_none());
    }

    #[test]
    fn extract_image_url_returns_none_when_no_srced_img_exists() {
        let html = r#"<p><img></p>"#;
        let option = extract_image_url(html);
        assert!(option.is_none());
    }
}
