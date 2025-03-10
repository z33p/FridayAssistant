using MassTransit;
using Microsoft.Extensions.Logging;

namespace Infrastructure.SagasStateMachine;

public class NewsletterStateMachine : MassTransitStateMachine<NewsletterState>
{
    public State FirstState { get; set; }
    public readonly ILogger<NewsletterStateMachine> _logger;

    public NewsletterStateMachine(ILogger<NewsletterStateMachine> logger)
    {
        _logger = logger;

        InstanceState(x => x.CurrentState);

        Event(() => BaseEvent.ReleaseInEvent, e => e.CorrelateById(i => i.Message.CorrelationId));

        Initially(
            When(BaseEvent.ReleaseInEvent)
                .Then(context =>
                {
                    context.Saga.PreviousState = context.Saga.CurrentState;
                    _logger.LogInformation("NewsletterStateMachine: Initially");
                })
                .TransitionTo(FirstState)
        );
    }
}
