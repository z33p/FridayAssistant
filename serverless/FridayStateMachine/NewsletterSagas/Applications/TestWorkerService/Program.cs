using Libs.Shared;
using Infrastructure.SagasStateMachine;
using Applications.TestWorkerService.Jobs;

namespace Applications.TestWorkerService;

public class Program
{
    public static async Task Main()
    {
        IHost host = HostBuilderConfiguration
            .CreateHostBuilder()
            .ConfigureServices((context, services) =>
            {
                MassTransitInjection.AddMassTransit(context.Configuration, services, isSagas: false);
                services.AddHostedService<TestJob>();
            })
            .Build();

        Task shutdown = host.WaitForShutdownAsync();

        await host.RunAsync();
        await shutdown;
    }
}