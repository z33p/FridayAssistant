use super::{AlexaResponse, Response, OutputSpeech};

pub fn not_handled_request_type() -> AlexaResponse {
    AlexaResponse {
        version: "1.0".to_string(),
        response: Response {
            output_speech: OutputSpeech {
                speech_type: "PlainText".to_string(),
                text: "Não entendi o que você quer dizer.".to_string(),
            },
        },
    }
}
