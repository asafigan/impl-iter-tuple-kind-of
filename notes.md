* talk about type mismatch in a vec
example:

error[E0308]: mismatched types
  --> src/v1/handlers.rs:11:36
   |
11 |             children: vec![logo(), "Kansai Rust", menu()],
   |                                    ^^^^^^^^^^^^^ expected opaque type, found `&str`
...
40 | fn logo() -> impl Component {
   |              -------------- the expected opaque type
   |
   = note: expected opaque type `impl components::Component`
                found reference `&'static str`

* talk about manually casting to `Box<dyn Component>`
example:

`Box::new(H1("Title")) as Box<dyn Component>`

* only the first element of vec needs `as Box<dyn Component>`

* second iteration: `to_dyn` method

* talk about needing `where Self: Size + 'static` on `to_dyn`

* third iteration: get rid of `vec![]`

* implement `ComponentList` trait