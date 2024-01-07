namespace Libs.NewsletterStateMachine.Sagas.Events;

public class FetchContentEvent : BaseEvent, IActionEvent
{
    public string FunctionName { get; } = "friday-newsletter";

    public string GetInvocationPayload()
    {
        throw new NotImplementedException();
    }
}


public class ResultFetchContentEvent : BaseEvent { }
