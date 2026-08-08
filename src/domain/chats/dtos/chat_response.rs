use serde::Deserialize;

//Expected result
// {
//   "model": "qwen3:8b",
//   "created_at": "2026-08-03T23:10:00.000Z",
//   "message": {
//     "role": "assistant",
//     "content": "Hello! How can I help you learn English?"
//   },
//   "done": true,
//   "done_reason": "stop",
//   "total_duration": 5773582377,
//   "load_duration": 140998520,
//   "prompt_eval_count": 30,
//   "prompt_eval_duration": 42674000,
//   "eval_count": 291,
//   "eval_duration": 5494091000
// }
#[derive(Deserialize)]
pub struct ChatResponse {
    pub model: String,
    pub created_at: String, //qq eh datetime aq mds
    pub total_duration: u64,
    pub done: bool,
    pub done_reason: String,
    pub eval_count: u64,
    pub eval_duration: u64,
    pub load_duration: u64,
    pub prompt_eval_count: u64,
    pub prompt_eval_duration: u64,
    pub message: AiMessage,
}

#[derive(Deserialize)]
pub struct AiMessage {
    pub role: String,
    pub content: String,
}
