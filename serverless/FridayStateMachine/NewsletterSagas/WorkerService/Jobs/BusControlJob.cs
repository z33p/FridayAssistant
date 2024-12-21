using MassTransit;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;

namespace WorkerService.Jobs;

public class BusControlJob : IHostedService
{
    private readonly ILogger<BusControlJob> _logger;
    private readonly IBusControl _busControl;

    public BusControlJob(ILogger<BusControlJob> logger, IBusControl busControl)
    {
        _logger = logger;
        _busControl = busControl;
    }

    public async Task StartAsync(CancellationToken cancellationToken)
    {
        _logger.LogDebug("Staring bus");
        await _busControl.StartAsync(cancellationToken);
    }

    public async Task StopAsync(CancellationToken cancellationToken)
    {
        _logger.LogDebug("Stoping bus");
        await _busControl.StopAsync(cancellationToken);
    }
}