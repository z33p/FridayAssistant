namespace Infrastructure.StateMachine.Sagas.Events;

public interface IActionEvent
{
    string FunctionName { get; }
    string GetInvocationPayload();
}
