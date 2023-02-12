use lambda_runtime::{Error, LambdaEvent};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AlexaRequest {
    #[serde(rename = "requestId")]
    request_id: String,
    #[serde(rename = "type")]
    request_type: String,
    #[serde(rename = "timestamp")]
    timestamp: String,
    #[serde(rename = "locale")]
    locale: String,
    #[serde(rename = "intent")]
    intent: Intent,
}

#[derive(Deserialize, Serialize)]
pub struct Intent {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "slots")]
    slots: Option<Slots>,
}

#[derive(Deserialize, Serialize)]
pub struct Slots {
    // Define os slots aqui, se for o caso
}

#[derive(Serialize, Deserialize)]
pub struct AlexaResponse {
    #[serde(rename = "version")]
    version: String,
    #[serde(rename = "response")]
    response: Response,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "outputSpeech")]
    output_speech: OutputSpeech,
}

#[derive(Serialize, Deserialize)]
pub struct OutputSpeech {
    #[serde(rename = "type")]
    speech_type: String,
    #[serde(rename = "text")]
    text: String,
}

pub async fn handler(e: LambdaEvent<AlexaRequest>) -> Result<AlexaResponse, Error> {
    // Aqui você pode processar a requisição Alexa e tomar decisões sobre
    // qual frase retornar com base no tipo de requisição ou no conteúdo
    // da requisição.
    let response = match e.payload.request_type.as_ref() {
        "IntentRequest" => {
            let text = match e.payload.intent.name.as_ref() {
                "HelloIntent" => "Olá!",
                "GoodbyeIntent" => "Adeus!",
                _ => "Não entendi o que você quer dizer.",
            };

            AlexaResponse {
                version: "1.0".to_string(),
                response: Response {
                    output_speech: OutputSpeech {
                        speech_type: "PlainText".to_string(),
                        text: text.to_string(),
                    },
                },
            }
        },
        _ => AlexaResponse {
            version: "1.0".to_string(),
            response: Response {
                output_speech: OutputSpeech {
                    speech_type: "PlainText".to_string(),
                    text: "Não entendi o que você quer dizer.".to_string(),
                },
            },
        },
    };

    Ok(response)
}
