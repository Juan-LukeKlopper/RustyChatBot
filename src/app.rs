use leptos::*;
use leptos_meta::*;

use crate::model::conversation::{Conversation, Message};

mod components;
use crate::app::components::chat_area::ChatArea;
use crate::app::components::type_area::TypeArea;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    let (conversation, set_conversation) = create_signal(cx, Conversation::new());

    let send = create_action(cx, move |new_message: &String| {
       let user_message = Message {
           text: new_message.clone(),
           user: true
        };

       set_conversation.update(move |c| {
           c.messages.push(user_message);
       });

       // TODO: add conversing
    });
        
    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rusty_chat_bot.css"/>

        // sets the document title
        <Title text="RustyChatBot"/>
        <ChatArea conversation/>
        <TypeArea send/>
    }
}

