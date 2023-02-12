use lambda_runtime::LambdaEvent;
use tracing::{debug, info};

use crate::alexa_handler::{OutputSpeech, Response};

use super::{AlexaRequest, AlexaResponse};

pub fn handle_intent(e: LambdaEvent<AlexaRequest>) -> AlexaResponse {
    let intent = e.payload.request.intent.unwrap();

    debug!("Trying intent: {}", intent.name);

    let intent_response = match intent.name.as_ref() {
        "HelloIntent" => "Olá Iago!",
        "GoodbyeIntent" => "Adeus!",
        _ => "Não entendi o que você quer dizer.",
    };

    info!("Intent response: {}", intent_response);

    AlexaResponse {
        version: "1.0".to_string(),
        response: Response {
            output_speech: OutputSpeech {
                speech_type: "PlainText".to_string(),
                text: intent_response.to_string(),
            },
        },
    }
}
