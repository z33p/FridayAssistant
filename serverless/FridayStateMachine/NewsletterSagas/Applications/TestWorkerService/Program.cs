using Libs.Shared;
using Infrastructure.SagasStateMachine;
using Applications.TestWorkerService.Jobs;
using MassTransit;

namespace Applications.TestWorkerService;

public class Program
{
    public static async Task Main()
    {
        IHost host = HostBuilderConfiguration
            .CreateHostBuilder()
            .ConfigureServices((context, serviceCollection) =>
            {
                MassTransitInjection.AddMassTransit(
                    context.Configuration,
                    serviceCollection,
                    () =>
                    {
                        string newsletterSagasEndpoint = context.Configuration.GetValue<string>("MassTransitEndpoints:NewsletterSagas")!;
                        EndpointConvention.Map<ReleaseInEvent>(new Uri($"exchange:{newsletterSagasEndpoint}"));
                    },
                    (busRegistrationContext, rabbitMqBusFactoryConfigurator) => { }
                );

                serviceCollection.AddHostedService<TestJob>();
            })
            .Build();

        Task shutdown = host.WaitForShutdownAsync();

        await host.RunAsync();
        await shutdown;
    }
}