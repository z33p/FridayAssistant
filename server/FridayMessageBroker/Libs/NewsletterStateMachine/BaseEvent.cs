using MassTransit;

namespace Libs.NewsletterStateMachine.Sagas;

public class BaseEvent : NewsletterStateAbstract
{
    public static Event<ReleaseInEvent> ReleaseInEvent { get; private set; }
}

public class ReleaseInEvent : BaseEvent { }
