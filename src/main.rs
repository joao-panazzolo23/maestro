use crate::structs::work_tools::WorkTools;

slint::include_modules!();
pub mod structs;

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    let app = App::new()?;
    //todo: this is supposed to be dinamically set.
    let work_tools = WorkTools::new("http://localhost:11434/api", "qwen3:8b");
    start_tutor(&work_tools, &app).await;
    app.run()
}

pub async fn start_tutor(wt: &WorkTools, app: &App) {
    //seria uma bomba dessas aqui
    // let weak_app = app.as_weak();

    // tokio::spawn(async move {
    //     let result = service
    //         .send_message(message.as_str())
    //         .await;

    //     let _ = slint::invoke_from_event_loop(
    //         move || {
    //             if let Some(app) = weak_app.upgrade() {
    //                 match result {
    //                     Ok(response) => {
    //                         app.set_assistant_message(
    //                             response.into()
    //                         );
    //                     }

    //                     Err(error) => {
    //                         app.set_assistant_message(
    //                             format!("Erro: {error}")
    //                                 .into()
    //                         );
    //                     }
    //                 }
    //             }
    //         }
    //     );
    // });

    app.on_send_message(move |message| {});
}
