use axum::response::{Html, IntoResponse};
pub struct Page<T>(pub T);

impl<T: ComponentList> IntoResponse for Page<T> {
    fn into_response(self) -> axum::response::Response {
        let body_content = self.0.render_components();
        let html = format!(
            "<!Doctype html><html lang=\"en\"><head></head><body>{body_content}</body></html>"
        );
        Html(html).into_response()
    }
}

pub trait ComponentList {
    type Item: Component + ?Sized;
    type IntoIter<'a>: Iterator<Item = &'a Self::Item>
    where
        Self: 'a;

    fn render_components(&self) -> String {
        let mut content = String::new();

        for component in self.iter_components() {
            content.push_str(&component.render());
        }

        content
    }

    fn iter_components<'a>(&'a self) -> Self::IntoIter<'a>;
}

impl<T: Component> ComponentList for Vec<T> {
    type Item = T;

    type IntoIter<'a> = core::slice::Iter<'a, T> where T: 'a;
    fn iter_components<'a>(&'a self) -> Self::IntoIter<'a> {
        self.iter()
    }
}

impl<const N: usize, T: Component> ComponentList for [T; N] {
    type Item = T;

    type IntoIter<'a> = core::slice::Iter<'a, T> where T: 'a;
    fn iter_components<'a>(&'a self) -> Self::IntoIter<'a> {
        self.iter()
    }
}

impl<A: Component + 'static, B: Component + 'static> ComponentList for (A, B) {
    type Item = dyn Component + 'static;

    type IntoIter<'a> = IterAB<'a, A, B> where Self: 'a;

    fn iter_components<'a>(&'a self) -> Self::IntoIter<'a> {
        IterAB::new(self)
    }
}

pub struct IterAB<'a, A, B> {
    tuple: &'a (A, B),
    count: u8,
}

impl<'a, A, B> IterAB<'a, A, B> {
    pub fn new(tuple: &'a (A, B)) -> Self {
        Self { tuple, count: 0 }
    }
}

impl<'a, A: Component + 'static, B: Component + 'static> Iterator for IterAB<'a, A, B> {
    type Item = &'a (dyn Component + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        match self.count {
            1 => Some(&self.tuple.0),
            2 => Some(&self.tuple.1),
            _ => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = 0.max(2 - self.count as usize);
        (size, Some(size))
    }
}

impl<'a, A: Component + 'static, B: Component + 'static> ExactSizeIterator for IterAB<'a, A, B> {
    fn len(&self) -> usize {
        0.max(2 - self.count as usize)
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

pub struct Ul<T>(pub T);

impl<T: ComponentList> Component for Ul<T> {
    fn render(&self) -> String {
        let mut content = String::new();

        for component in self.0.iter_components() {
            content.push_str(&format!("<li>{}</li>", component.render()));
        }

        format!("<ul>{content}</ul>")
    }
}

pub struct HList<T>(pub T);

impl<T: ComponentList> Component for HList<T> {
    fn render(&self) -> String {
        format!(
            "<div style=\"display: flex; align-items: center;\">{}</div>",
            self.0.render_components()
        )
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
