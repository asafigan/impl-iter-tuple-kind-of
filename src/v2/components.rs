use axum::response::{Html, IntoResponse};
pub struct Page<T>(pub Vec<T>);

impl<T: Component> IntoResponse for Page<T> {
    fn into_response(self) -> axum::response::Response {
        let mut content = String::new();

        for component in &self.0 {
            content.push_str(&component.render());
        }

        let html =
            format!("<!Doctype html><html lang=\"en\"><head></head><body>{content}</body></html>");
        Html(html).into_response()
    }
}

pub trait Component {
    fn render(&self) -> String;
    fn to_dyn(self) -> Box<dyn Component>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

impl<'a> Component for &'a str {
    fn render(&self) -> String {
        html_escape::encode_text(self).to_string()
    }
}

impl<'a> Component for Box<dyn Component> {
    fn render(&self) -> String {
        Box::as_ref(&self).render()
    }
}

pub struct Ul<T>(pub Vec<T>);

impl<T: Component> Component for Ul<T> {
    fn render(&self) -> String {
        let mut content = String::new();

        for component in &self.0 {
            content.push_str(&format!("<li>{}</li>", component.render()));
        }

        format!("<ul>{content}</ul>")
    }
}

pub struct HList<T>(pub Vec<T>);

impl<T: Component> Component for HList<T> {
    fn render(&self) -> String {
        let mut content = String::new();

        for component in &self.0 {
            content.push_str(&component.render());
        }

        format!("<div style=\"display: flex; align-items: center;\">{content}</div>")
    }
}

pub struct H3<T>(pub T);

impl<T: Component> Component for H3<T> {
    fn render(&self) -> String {
        format!("<h3>{}</h3>", self.0.render())
    }
}

pub struct H2<T>(pub T);

impl<T: Component> Component for H2<T> {
    fn render(&self) -> String {
        format!("<h2>{}</h2>", self.0.render())
    }
}

pub struct H1<T>(pub T);

impl<T: Component> Component for H1<T> {
    fn render(&self) -> String {
        format!("<h1>{}</h1>", self.0.render())
    }
}

pub struct P<T>(pub T);

impl<T: Component> Component for P<T> {
    fn render(&self) -> String {
        format!("<p>{}</p>", self.0.render())
    }
}

pub struct Img<T> {
    pub src: T,
    pub width: u32,
    pub height: u32,
}

impl<T: AsRef<str>> Component for Img<T> {
    fn render(&self) -> String {
        let Self { src, width, height } = self;
        let src = html_escape::encode_quoted_attribute(src.as_ref());
        format!("<img src=\"{src}\" width=\"{width}p\" height=\"{height}px\"/>",)
    }
}
