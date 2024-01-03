using MassTransit;

namespace Libs.NewsletterStateMachine.Sagas;

public class NewsletterState : NewsletterStateAbstract, SagaStateMachineInstance
{

}


public abstract class NewsletterStateAbstract
{
    public Guid CorrelationId { get; set; }
    public uint RowVersion { get; set; }

    public string PreviousState { get; set; } = string.Empty;
    public string CurrentState { get; set; }
}