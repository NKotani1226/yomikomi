use tera::{Tera, Context};
use crate::model::GoogleBooksResponse;
pub fn render_page(items:GoogleBooksResponse) -> Result<String, tera::Error> {
        
    let tmpl = Tera::new("templates/**/*").unwrap();
    let mut ctx = Context::new();

    ctx.insert("items",&items);

    tmpl.render("index.html", &ctx) 
}