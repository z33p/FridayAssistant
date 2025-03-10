using MassTransit;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;

namespace Infrastructure.StateMachine.Jobs;

public class BusControlJob : IHostedService
{
    readonly IBusControl _busControl;
    readonly ILogger _logger;

    public BusControlJob(IBusControl busControl, ILoggerFactory loggerFactory)
    {
        _busControl = busControl;
        _logger = loggerFactory.CreateLogger<BusControlJob>();
    }

    public async Task StartAsync(CancellationToken cancellationToken)
    {
        _logger.LogInformation("BusControlJob: Starting bus control");
        await _busControl.StartAsync(cancellationToken).ConfigureAwait(false);
    }

    public Task StopAsync(CancellationToken cancellationToken)
    {
        _logger.LogInformation("BusControlJob: Stopping bus control");
        return _busControl.StopAsync(cancellationToken);
    }
}
