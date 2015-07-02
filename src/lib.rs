extern crate typemap;
extern crate diecast;
extern crate hoedown;

use diecast::{Item, Handle};

use hoedown::Render;
use hoedown::renderer::html;

pub struct Markdown;

pub fn markdown() -> Markdown {
    Markdown
}

impl Handle<Item> for Markdown {
    fn handle(&self, item: &mut Item) -> diecast::Result<()> {
        let document =
            hoedown::Markdown::new(&item.body)
            .extensions({
                use hoedown::*;

                AUTOLINK |
                FENCED_CODE |
                FOOTNOTES |
                MATH |
                MATH_EXPLICIT |
                SPACE_HEADERS |
                STRIKETHROUGH |
                SUPERSCRIPT |
                TABLES
            });

        let mut renderer = hoedown::Html::new(html::Flags::empty(), 0);

        let output = renderer.render(&document);

        let mut smartypants = hoedown::Buffer::new(64);
        html::smartypants(&output, &mut smartypants);

        item.body = String::from(smartypants.to_str().unwrap());

        Ok(())
    }
}

