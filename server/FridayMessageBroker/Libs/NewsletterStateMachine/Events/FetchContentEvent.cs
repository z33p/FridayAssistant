namespace Libs.NewsletterStateMachine.Sagas.Events;

public class FetchContentEvent : BaseEvent, IActionEvent
{
    public string FunctionName { get; } = "friday-newsletter";

    public string GetInvocationPayload() => "{\"action\":\"GENERATE_LINKEDIN_NEWS_POST\",\"data\":null}";
}


public class ResultFetchContentEvent : BaseEvent { }
