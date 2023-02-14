mod intent_request_matcher;
mod launch_request;
mod request_type_default;
use lambda_runtime::{Error, LambdaEvent};
use serde_derive::{Deserialize, Serialize};
use tracing::debug;

#[derive(Deserialize, Serialize)]
pub struct AlexaRequest {
    #[serde(rename = "request")]
    request: RequestData,
}

#[derive(Deserialize, Serialize)]
pub struct RequestData {
    #[serde(rename = "requestId")]
    request_id: String,
    #[serde(rename = "type")]
    request_type: String,
    #[serde(rename = "timestamp")]
    timestamp: String,
    #[serde(rename = "locale")]
    locale: String,
    #[serde(rename = "intent")]
    intent: Option<Intent>,
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
    #[serde(rename = "FirstName")]
    first_name: Option<String>,
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

pub async fn handler(event: LambdaEvent<AlexaRequest>) -> Result<AlexaResponse, Error> {
    debug!("Starting alexa lambda handler");

    // Aqui você pode processar a requisição Alexa e tomar decisões sobre
    // qual frase retornar com base no tipo de requisição ou no conteúdo
    // da requisição.
    let response = match event.payload.request.request_type.as_ref() {
        "IntentRequest" => intent_request_matcher::handle_intent(event),
        "LaunchRequest" => launch_request::handle_launch(),
        _ => request_type_default::not_handled_request_type(),
    };

    Ok(response)
}
