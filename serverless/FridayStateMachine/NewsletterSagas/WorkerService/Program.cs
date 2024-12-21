using Infrastructure.StateMachine;
using Libs.Shared;
using WorkerService.Jobs;

namespace WorkerService;

public class Program
{
    public static async Task Main()
    {
        IHost host = HostBuilderConfiguration
            .CreateHostBuilder()
            .ConfigureServices((context, services) =>
            {
                services.AddHostedService<BusControlJob>();
                NewsletterSagasConfiguration.AddMassTransit(context, services);
            })
            .Build();

        Task shutdown = host.WaitForShutdownAsync();

        await host.RunAsync();
        await shutdown;
    }
}