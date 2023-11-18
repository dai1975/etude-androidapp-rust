use dioxus::prelude::*;

#[component]
fn FutureElement<'a, T: core::fmt::Display + 'static>(
   cx: Scope,
   f: &'a UseFuture<Result<T, String>>,
) -> Element {
   match f.value() {
      None => cx.render(rsx! { "accessing..." }),
      Some(Ok(v)) => cx.render(rsx! { "{*v}"}),
      Some(Err(e)) => cx.render(rsx! { "error: {e}" }),
   }
}

pub fn app(cx: Scope) -> Element {
   let last_count = use_future(&cx, (), |()| crate::file::read_count());
   let count: &UseState<Option<Result<u32, String>>> = use_state(&cx, || None);
   let write_count = {
      let count2 = count.clone();
      use_future(&cx, (), |()| async move {
         let r = match count2.get() {
            None => crate::file::read_count().await,
            Some(Err(ref e)) => Err(e.clone()),
            Some(Ok(ref v)) => crate::file::write_count(v + 1).await,
         };
         count2.set(Some(r));
      })
   };

   let count_view = match count.get() {
      None => cx.render(rsx! { "loading..." }),
      Some(Err(ref e)) => cx.render(rsx! {"{e}" }),
      Some(Ok(ref v)) => cx.render(rsx! {div {"{v}" button {
         onclick: move|_| {
            log::debug!("Clicked!");
            write_count.restart();
         },
         "Incr"
      } }}),
   };

   let https = use_future(&cx, (), |()| crate::https::get());

   log::debug!("Hello from the app");
   render! {
     div {
        h1 { "Hello, Mobile"}
         div {
            margin_left: "auto", margin_right: "auto", width: "200px", padding: "10px", border: "1px solid black",
            div {
               "previous written data: " FutureElement { f: last_count }
            }
            count_view
            div {
               "network: " FutureElement { f: https }
            }
         }
      }
   }
}
