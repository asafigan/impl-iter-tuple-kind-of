use super::components::*;
use axum::{response::IntoResponse, routing::get, Router};

pub fn router() -> Router {
    Router::new().route("/", get(index))
}

async fn index() -> impl IntoResponse {
    Page(vec![
        Box::new(header()) as Box<dyn Component>,
        Box::new(H1(
            "Rust talk: How to implement Iterator on tuples... kind of",
        )),
        Box::new(H2("Details")),
        Box::new(P(
            "Have you ever wanted to iterate over a collection of types
            that share the same trait? You could put them in an array or 
            a vector but than you would have to convert them into a trait
            object first. What a hassle! What if I told you you can use a 
            tuple instead?",
        )),
        Box::new(P(
            "Andrew Safigan (Software Engineer at NEI Japan) will give a talk
            about implementing iterators on tuples in Rust. Join us to learn 
            about the benefits and challenges of trying to do so.",
        )),
        Box::new(P(
            "The event is being held at Kyoto City Higashiyama Iki-Iki Citizens’
            Activity Centre in Room 102. It is about 6 minutes walk for Sanjo Station.",
        )),
        Box::new(H3("Agenda")),
        Box::new(Ul(vec![
            "15 minutes open chat",
            "30 minutes presentation & questions",
            "15 minutes open chat",
        ])),
        Box::new(P(
            "After the event you are welcome go to a local cafe and talk casually with others.",
        )),
    ])
}

fn header() -> impl Component {
    HList {
        children: vec![
            Box::new(Img {
                src: "/public/logo.svg",
                width: 50,
                height: 50,
            }) as Box<dyn Component>,
            Box::new(H2("Kansai Rust")),
        ],
        ..Default::default()
    }
}
