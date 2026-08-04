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
    model: String,
    created_at: String, //qq eh datetime aq mds
    total_duration: u64,
    done: bool,
    done_reason: String,
    eval_count: u64,
    eval_duration: u64,
    load_duration: u64,
    prompt_eval_count: u64,
    prompt_eval_duration: u64,
    pub message: AiMessage,
}

#[derive(Deserialize)]
pub struct AiMessage {
    role: String,
    pub content: String,
}
