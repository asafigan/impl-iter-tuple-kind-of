use super::components::*;
use axum::{response::IntoResponse, routing::get, Router};

pub fn router() -> Router {
    Router::new().route("/", get(index))
}

async fn index() -> impl IntoResponse {
    Page(vec![
        header().to_dyn(),
        H1("Rust talk: How to implement Iterator on tuples... kind of").to_dyn(),
        H2("Details").to_dyn(),
        P("Have you ever wanted to iterate over a collection of types
            that share the same trait? You could put them in an array or 
            a vector but than you would have to convert them into a trait
            object first. What a hassle! What if I told you you can use a 
            tuple instead?")
        .to_dyn(),
        P(
            "Andrew Safigan (Software Engineer at NEI Japan) will give a talk
            about implementing iterators on tuples in Rust. Join us to learn 
            about the benefits and challenges of trying to do so.",
        )
        .to_dyn(),
        P(
            "The event is being held at Kyoto City Higashiyama Iki-Iki Citizens’
            Activity Centre in Room 102. It is about 6 minutes walk for Sanjo Station.",
        )
        .to_dyn(),
        H3("Agenda").to_dyn(),
        Ul(vec![
            "15 minutes open chat",
            "30 minutes presentation & questions",
            "15 minutes open chat",
        ])
        .to_dyn(),
        P("After the event you are welcome go to a local cafe and talk casually with others.")
            .to_dyn(),
    ])
}

fn header() -> impl Component {
    HList {
        children: vec![
            Img {
                src: "/public/logo.svg",
                width: 50,
                height: 50,
            }
            .to_dyn(),
            H2("Kansai Rust").to_dyn(),
        ],
        ..Default::default()
    }
}
