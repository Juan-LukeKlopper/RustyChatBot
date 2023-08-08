use leptos::{*, html::Input};

#[component]
pub fn TypeArea(cx: Scope, send: Action<String, Result<String, ServerFnError>>) -> impl IntoView {
    let input_ref = create_node_ref::<Input>(cx);
    view!{ cx,
        <div>
           <form on:submit=move |ev| {
                ev.prevent_default();
                let input = input_ref.get().expect("input to exist");
                send.dispatch(input.value());
                input.set_value("");
           }
           >
                <input type="text" placeholder="Enter your prompt" node_ref=input_ref/>
                <input type="submit"/>
           </form>
        </div>
    }
}
