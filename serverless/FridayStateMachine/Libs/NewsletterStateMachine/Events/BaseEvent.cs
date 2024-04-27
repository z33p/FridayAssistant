using MassTransit;

namespace Libs.NewsletterStateMachine.Sagas.Events;

public class BaseEvent : NewsletterStateAbstract
{
    public static Event<ReleaseInEvent> ReleaseInEvent { get; private set; }

    public static Event<FetchContentEvent> FetchContentEvent { get; private set; }
    public static Event<ResultFetchContentEvent> ResultFetchContentEvent { get; private set; }

    public static Event<FetchOAuthTokenEvent> FetchOAuthTokenEvent { get; private set; }
    public static Event<ResultFetchOAuthTokenEvent> ResultFetchOAuthTokenEvent { get; private set; }

    public static Event<SendNewsletterEvent> SendNewsletterEvent { get; private set; }
    public static Event<ResultSendNewsletterEvent> ResultSendNewsletterEvent { get; private set; }

    public static Event<ConcludedEvent> ConcludedEvent { get; private set; }
}

