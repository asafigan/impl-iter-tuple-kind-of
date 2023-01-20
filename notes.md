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

* talk about `type Item: Component;`

* talk about `type IntoIter<'a>: Iterator<Item = &'a Self::Item> where Self: 'a;`

* implementing it on `Vec<T>`

* implementing it on `[T;N]`

* talk about const generics

* implementing it on tuple `(A, B)`

* talk about `type Item = dyn Component + 'static;`

* talk about changing to `type Item: Component + ?Sized;`