using MassTransit;
using Microsoft.Extensions.Logging;

namespace Libs.NewsletterStateMachine.Sagas;

public class NewsletterStateMachine : MassTransitStateMachine<NewsletterState>
{
    private readonly ILogger<NewsletterStateMachine> _logger;

    public State FetchContentState { get; private set; }
    public State FetchOAuthTokenState { get; private set; }
    public State SendNewsletterState { get; private set; }
    public State ConcludedState { get; private set; }

    public NewsletterStateMachine(ILogger<NewsletterStateMachine> logger)
    {
        _logger = logger;
    
        InstanceState(state => state.CurrentState);

        DeclareEvents();

        Initially(WhenReleaseIn());

        During(FetchContentState, Ignore(BaseEvent.ReleaseInEvent));
        During(ConcludedState, Ignore(BaseEvent.ReleaseInEvent));
    }

    private EventActivityBinder<NewsletterState, ReleaseInEvent> WhenReleaseIn() =>
        When(BaseEvent.ReleaseInEvent)
            .Then(context =>
            {
                context.Saga.PreviousState = context.Saga.CurrentState;
                LogStateChange(context.Saga.CorrelationId, context.Saga.PreviousState, context.Saga.CurrentState);
            })
            .TransitionTo(FetchContentState)
            .SendAsync(context => context.Init<FetchOAuthTokenEvent>(CreatePreviousMessageToNewEvent<FetchOAuthTokenEvent>(context.Saga)));

    private void LogStateChange(Guid correlationId, string previousState, string currentState)
    {
        _logger.LogDebug(
            "CorrelationId {CorrelationId} alterando status de {PreviousState} para {CurrentState}",
            correlationId,
            previousState,
            currentState
        );
    }

    private void DeclareEvents()
    {
        Event(() => BaseEvent.ReleaseInEvent, e => e.CorrelateById(i => i.Message.CorrelationId));
        Event(() => BaseEvent.FetchOAuthTokenEvent, e => e.CorrelateById(i => i.Message.CorrelationId));
        Event(() => BaseEvent.SendNewsletterEvent, e => e.CorrelateById(i => i.Message.CorrelationId));
        Event(() => BaseEvent.ConcludedEvent, e => e.CorrelateById(i => i.Message.CorrelationId));
    }

    private static T CreatePreviousMessageToNewEvent<T>(NewsletterState sagaState) where T : BaseEvent, new() => new()
    {
        CorrelationId = sagaState.CorrelationId,
        RowVersion = sagaState.RowVersion,
        PreviousState = sagaState.PreviousState,
        CurrentState = sagaState.CurrentState,
    };
}
