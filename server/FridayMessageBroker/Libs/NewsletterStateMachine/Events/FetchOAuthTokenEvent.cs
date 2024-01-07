namespace Libs.NewsletterStateMachine.Sagas.Events;

public class FetchOAuthTokenEvent : BaseEvent, IActionEvent
{
    public string FunctionName { get; } = "friday-google-oauth";

    public string GetInvocationPayload() => "{\"action\":\"GENERATE_ACCESS_TOKEN\",\"data\":null}";
}

public class ResultFetchOAuthTokenEvent : BaseEvent { }
