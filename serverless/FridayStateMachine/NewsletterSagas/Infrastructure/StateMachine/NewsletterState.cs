using MassTransit;

namespace Infrastructure.StateMachine.Sagas;

public class NewsletterState : NewsletterStateAbstract, SagaStateMachineInstance
{

}


public abstract class NewsletterStateAbstract
{
    public Guid CorrelationId { get; set; }
    public string? Payload { get; set; }
    public uint RowVersion { get; set; }

    public string PreviousState { get; set; } = string.Empty;
    public string CurrentState { get; set; }
}