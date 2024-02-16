trait Render {
    fn render(&self, title: String, body: String);
}

struct HtmlRenderer {}

impl Render for HtmlRenderer {
    fn render(&self, title: String, body: String) {
        println!("<html><title>{}</title><body>{}</body></html>", title, body);
    }
}

struct MarkdownRenderer;

impl Render for MarkdownRenderer {
    fn render(&self, title: String, body: String) {
        println!("# {}\n{}", title, body);
    }
}

struct Page<T: Render> {
    renderer: T,
}

impl<T: Render> Page<T> {
    fn new(renderer: T) -> Page<T> {
        Page { renderer }
    }

    fn render(&self, title: String, body: String) {
        self.renderer.render(title, body);
    }
}

fn main() {
    let html = Page::new(HtmlRenderer {});
    html.render(String::from("제목"), String::from("내용"));

    let markdown = Page::new(MarkdownRenderer {});
    markdown.render(String::from("제목"), String::from("내용"));
}