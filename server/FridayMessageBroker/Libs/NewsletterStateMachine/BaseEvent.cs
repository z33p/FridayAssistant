using MassTransit;

namespace Libs.NewsletterStateMachine.Sagas;

public class BaseEvent : NewsletterStateAbstract
{
    public static Event<ReleaseInEvent> ReleaseInEvent { get; private set; }
    public static Event<FetchOAuthTokenEvent> FetchOAuthTokenEvent { get; private set; }
    public static Event<SendNewsletterEvent> SendNewsletterEvent { get; private set; }
    public static Event<ConcludedEvent> ConcludedEvent { get; private set; }
}

public class ReleaseInEvent : BaseEvent { }
public class FetchOAuthTokenEvent : BaseEvent { }
public class SendNewsletterEvent : BaseEvent { }
public class ConcludedEvent : BaseEvent { }
