namespace Libs.NewsletterStateMachine.Sagas.Events;

public interface IActionEvent
{
    string FunctionName { get; }
    string GetInvocationPayload();
}
