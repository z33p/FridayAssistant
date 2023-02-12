use super::{AlexaResponse, Response, OutputSpeech};

pub fn handle_launch() -> AlexaResponse {
    AlexaResponse {
        version: "1.0".to_string(),
        response: Response {
            output_speech: OutputSpeech {
                speech_type: "PlainText".to_string(),
                text: "Olá, AWS lambda acionada com sucesso".to_string(),
            },
        },
    }
}
