using MassTransit;
using Microsoft.Extensions.Logging;

namespace Infrastructure.SagasStateMachine;

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

        Initially(WhenReleaseInThenFetchContent());

        During(
            FetchContentState,
            Ignore(BaseEvent.ReleaseInEvent),
            WhenResultFetchContentThenFetchOAuthToken()
        );

        During(
            FetchOAuthTokenState,
            Ignore(BaseEvent.ReleaseInEvent),
            WhenResultFetchOAuthTokenThenSendNewsletter()
        );

        During(
            SendNewsletterState,
            Ignore(BaseEvent.ReleaseInEvent),
            WhenConcludedThenConcluded()
        );
    }

    private EventActivityBinder<NewsletterState, ReleaseInEvent> WhenReleaseInThenFetchContent() =>
        When(BaseEvent.ReleaseInEvent)
            .Then(context =>
            {
                context.Saga.PreviousState = context.Saga.CurrentState;
                LogStateChange(context.Saga.CorrelationId, context.Saga.PreviousState, context.Saga.CurrentState);
            })
            .TransitionTo(FetchContentState)
            .SendAsync(context => context.Init<FetchContentEvent>(CreatePreviousMessageToNewEvent<FetchContentEvent>(context.Saga)));

    private EventActivityBinder<NewsletterState, ResultFetchContentEvent> WhenResultFetchContentThenFetchOAuthToken() =>
        When(BaseEvent.ResultFetchContentEvent)
            .Then(context =>
            {
                context.Saga.PreviousState = context.Saga.CurrentState;
                LogStateChange(context.Saga.CorrelationId, context.Saga.PreviousState, context.Saga.CurrentState);
            })
            .TransitionTo(FetchOAuthTokenState)
            .SendAsync(context => context.Init<FetchOAuthTokenEvent>(CreatePreviousMessageToNewEvent<FetchOAuthTokenEvent>(context.Saga)));

    private EventActivityBinder<NewsletterState, ResultFetchOAuthTokenEvent> WhenResultFetchOAuthTokenThenSendNewsletter() =>
        When(BaseEvent.ResultFetchOAuthTokenEvent)
            .Then(context =>
            {
                context.Saga.PreviousState = context.Saga.CurrentState;
                LogStateChange(context.Saga.CorrelationId, context.Saga.PreviousState, context.Saga.CurrentState);
            })
            .TransitionTo(SendNewsletterState)
            .SendAsync(context => context.Init<SendNewsletterEvent>(CreatePreviousMessageToNewEvent<SendNewsletterEvent>(context.Saga)));

    private EventActivityBinder<NewsletterState, ConcludedEvent> WhenConcludedThenConcluded() =>
        When(BaseEvent.ConcludedEvent)
            .Then(context =>
            {
                context.Saga.PreviousState = context.Saga.CurrentState;
                LogStateChange(context.Saga.CorrelationId, context.Saga.PreviousState, context.Saga.CurrentState);
            })
            .TransitionTo(ConcludedState);

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

        Event(() => BaseEvent.FetchContentEvent, e => e.CorrelateById(i => i.Message.CorrelationId));
        Event(() => BaseEvent.ResultFetchContentEvent, e => e.CorrelateById(i => i.Message.CorrelationId));

        Event(() => BaseEvent.FetchOAuthTokenEvent, e => e.CorrelateById(i => i.Message.CorrelationId));
        Event(() => BaseEvent.ResultFetchOAuthTokenEvent, e => e.CorrelateById(i => i.Message.CorrelationId));

        Event(() => BaseEvent.SendNewsletterEvent, e => e.CorrelateById(i => i.Message.CorrelationId));
        Event(() => BaseEvent.ResultSendNewsletterEvent, e => e.CorrelateById(i => i.Message.CorrelationId));

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
