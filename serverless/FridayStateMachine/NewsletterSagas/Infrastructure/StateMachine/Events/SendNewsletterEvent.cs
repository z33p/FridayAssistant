namespace Infrastructure.StateMachine.Sagas.Events;

public class SendNewsletterEvent : BaseEvent, IActionEvent
{
    public string FunctionName { get; } = "friday-email-sender";

    public string GetInvocationPayload()
    {
        throw new NotImplementedException();
    }
}

public class ResultSendNewsletterEvent : BaseEvent { }
