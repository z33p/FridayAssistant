using MassTransit;

namespace Infrastructure.StateMachine;

public class BaseEvent
{
    public Guid CorrelationId { get; set; } = Guid.NewGuid();
    public uint RowVersion { get; set; }
    public string CurrentState { get; set; }
    public string PreviousState { get; set; } = string.Empty;

    public static Event<ReleaseInEvent> ReleaseInEvent { get; set; }
}

public class ReleaseInEvent : BaseEvent
{
}