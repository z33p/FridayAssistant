using Infrastructure.SagasStateMachine;
using MassTransit;

namespace Applications.TestWorkerService.Jobs;

public class TestJob : BackgroundService
{
    private readonly ILogger<TestJob> _logger;
    private readonly IBus _bus;

    public TestJob(ILogger<TestJob> logger, IBus bus)
    {
        _logger = logger;
        _bus = bus;
    }

    protected override async Task ExecuteAsync(CancellationToken stoppingToken)
    {
        // while (!stoppingToken.IsCancellationRequested)
        // {
        //     _logger.LogInformation("Worker running at: {time}", DateTimeOffset.Now);
        //     await Task.Delay(1000, stoppingToken);
        // }

        Guid guid = Guid.NewGuid();

        try
        {
            await _bus.Send(new ReleaseInEvent()
            {
                CorrelationId = guid,
            });

            _logger.LogInformation("ReleaseInEvent sent with CorrelationId: {CorrelationId}", guid);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to send ReleaseInEvent with CorrelationId: {CorrelationId}", guid);
        }
    }
}