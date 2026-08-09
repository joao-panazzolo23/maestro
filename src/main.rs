pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;
pub mod schema;

use crate::{
    domain::chats::dtos::{
        chat_message::ChatMessage, chat_request::ChatRequest, chat_response::ChatResponse,
    },
    infrastructure::work_tools::WorkTools,
};
use std::sync::Arc;

slint::include_modules!();
///TODO: REFACTOR MAIN
#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    let app = App::new()?;
    //todo: this is supposed to be dinamically set.
    let wt = Arc::new(WorkTools::new("http://localhost:11434/api", "qwen3:8b"));
    start_tutor(wt, &app);

    app.run()
}

///ARC = ATOMIC REFERENCE COUNTER (SMART POINTER)
pub fn start_tutor(wt: Arc<WorkTools>, app: &App) {
    let weak_app = app.as_weak();

    //slint`s callback is not async. i cant call requests in here.
    app.on_send_message(move |message| {
        let wt = Arc::clone(&wt);
        let weak_app = weak_app.clone();
        let user_message: String = message.to_string();

        let request = ChatRequest {
            model: "qwen3:8b".to_owned(),
            messages: vec![
                ChatMessage {
                    role: "system".to_owned(),
                    content: "You are Maestro, a multilingual language tutor.".to_owned(),
                },
                ChatMessage {
                    role: "user".to_owned(),
                    content: user_message,
                },
            ],
            stream: false,
        };

        tokio::spawn(async move {
            let result = wt.send_message::<ChatResponse>(&request).await;
            // por enquanto, só testa a resposta
            match result {
                Ok(response) => {
                    slint::invoke_from_event_loop(move || {
                        if let Some(app) = weak_app.upgrade() {
                            app.set_ai_response(response.message.content.into());
                        }
                    });
                }
                Err(error) => {
                    eprintln!("Erro ao chamar o Ollama: {error}");
                }
            }
        });
    })
}
