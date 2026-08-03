use crate::structs::{chat_request::ChatRequest, work_tools::WorkTools};

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
    app.on_send_message(move |message| {});
}
