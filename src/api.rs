use leptos::*;
use leptos_router::History;
use crate::model::conversation::Conversation;

#[server(Converse "/api")]
pub async fn converse(cx: Scope, prompt: Conversation) -> Result<String, ServerFnError> {
    use llm::models::Llama;
    use leptos_actix::extract;
    use actix_web::web::Data;
    use actix_web::dev::ConnectionInfo;

    let model = extract(cx, |data: Data<Llama>, _connection: ConnectionInfo| async {
        data.into_inner();
    })
    .await.unwrap();

    use llm::KnownModel;
    let char_name = "### Assistant";
    let user_name = "### Human";
    let persona = "A chat between a human and a LLM";
    let mut history = format!(
        "{char_name}:Hello - How may I help you today?\n\
        {user_name}:What is the capital of France?\n\
        {char_name}:Paris is the capital of France.\n"
    );

    for message in prompt.messages.into_iter() {
        let msg = message.text;
        let current_line = if message.user {
            format!("{char_name}:{msg}\n")
        } else {
            format!("{user_name}:{msg}\n")
        };

        history.push_str(&current_line);
    }

    Ok(String::from(""))

}

