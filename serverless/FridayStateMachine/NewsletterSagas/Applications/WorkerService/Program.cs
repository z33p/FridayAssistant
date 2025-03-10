using Libs.Shared;
using Infrastructure.StateMachine;

namespace Applications.WorkerService;

public class Program
{
    public static async Task Main()
    {
        IHost host = HostBuilderConfiguration
            .CreateHostBuilder()
            .ConfigureServices((context, services) =>
            {
                MassTransitInjection.AddMassTransit(context.Configuration, services);
            })
            .Build();

        Task shutdown = host.WaitForShutdownAsync();

        await host.RunAsync();
        await shutdown;
    }
}