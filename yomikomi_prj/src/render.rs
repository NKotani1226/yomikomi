use tera::{Tera, Context};
pub fn render_page() -> Result<String, tera::Error> {
        
    let tmpl = Tera::new("templates/**/*").unwrap();
    let mut ctx = Context::new();
    ctx.insert("title", "kingsman");

    tmpl.render("index.html", &ctx) 
}