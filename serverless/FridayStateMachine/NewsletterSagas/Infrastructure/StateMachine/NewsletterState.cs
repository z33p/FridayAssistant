using MassTransit;

namespace Infrastructure.StateMachine;

public class NewsletterState : SagaStateMachineInstance
{
    public Guid CorrelationId { get; set; }
    public uint RowVersion { get; set; }
    public string CurrentState { get; set; }
    public string PreviousState { get; set; } = string.Empty;
}