using Libs.NewsletterStateMachine.Sagas.Events;
using MassTransit;

namespace FridayMessageBroker.Jobs;

public class Worker : BackgroundService
{
    private readonly ILogger<Worker> _logger;
    private readonly IBus _bus;

    public Worker(ILogger<Worker> logger, IBus bus)
    {
        _logger = logger;
        _bus = bus;
    }

    protected override async Task ExecuteAsync(CancellationToken cancellationToken)
    {
        ReleaseInEvent releaseInEvent = new()
        {
            CorrelationId = Guid.NewGuid(),
        };

        await _bus.Send(releaseInEvent, cancellationToken);
    }
}
