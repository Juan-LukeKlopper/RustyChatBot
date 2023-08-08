use leptos::{*, html::Div};
use crate::model::conversation::Conversation;

const USER_MESSAGE_CLASS: &str = "user";
const MODEL_MESSAGE_CLASS: &str = "model";

#[component]
pub fn ChatArea(cx: Scope, conversation: ReadSignal<Conversation>) -> impl IntoView {
    let chat_div_ref = create_node_ref::<Div>(cx);
    create_effect(cx, move |_| {
      conversation.get();
      if let Some(div) = chat_div_ref.get() {
        div.set_scroll_top(div.scroll_height());
      }
    });
    view! { cx,
        <div class="msg" node_ref=chat_div_ref>
          {move || conversation.get().messages.iter().map(move |message| {
              let class_str = if message.user { USER_MESSAGE_CLASS } else { MODEL_MESSAGE_CLASS };
              view! {cx,
                <div class={class_str}>
                  {message.text.clone()}
                </div>
              }
            }).collect::<Vec<_>>()
          }
        </div>
    }
}
