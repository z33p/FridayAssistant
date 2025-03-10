using MassTransit;

namespace Infrastructure.SagasStateMachine
{

    public class BaseEvent
    {
        public Guid CorrelationId { get; set; } = Guid.NewGuid();
        public uint RowVersion { get; set; }
        public string CurrentState { get; set; }
        public string PreviousState { get; set; } = string.Empty;

        public static Event<ReleaseInEvent> ReleaseInEvent { get; set; }
        public static Event<FetchContentEvent> FetchContentEvent { get; set; }
        public static Event<ResultFetchContentEvent> ResultFetchContentEvent { get; set; }
        public static Event<FetchOAuthTokenEvent> FetchOAuthTokenEvent { get; set; }
        public static Event<ResultFetchOAuthTokenEvent> ResultFetchOAuthTokenEvent { get; set; }
        public static Event<SendNewsletterEvent> SendNewsletterEvent { get; set; }
        public static Event<ResultSendNewsletterEvent> ResultSendNewsletterEvent { get; set; }
        public static Event<ConcludedEvent> ConcludedEvent { get; set; }
    }

    public class ReleaseInEvent : BaseEvent { }
    public class FetchContentEvent : BaseEvent { }
    public class ResultFetchContentEvent : BaseEvent { }
    public class FetchOAuthTokenEvent : BaseEvent { }
    public class ResultFetchOAuthTokenEvent : BaseEvent { }
    public class SendNewsletterEvent : BaseEvent { }
    public class ResultSendNewsletterEvent : BaseEvent { }
    public class ConcludedEvent : BaseEvent { }
}