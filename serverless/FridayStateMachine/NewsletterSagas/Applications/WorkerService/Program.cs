using Libs.Shared;
using Infrastructure.SagasStateMachine;
using Infrastructure.SagasStateMachine.Jobs;
using MassTransit;

namespace Applications.WorkerService;

public class Program
{
    public static async Task Main()
    {
        IHost host = HostBuilderConfiguration
            .CreateHostBuilder()
            .ConfigureServices((context, serviceCollection) =>
            {
                serviceCollection.AddHostedService<BusControlJob>();

                MassTransitInjection.AddMassTransit(
                    context.Configuration,
                    serviceCollection,
                    () =>
                    {
                        string newsletterSagasEndpoint = context.Configuration.GetValue<string>("MassTransitEndpoints:NewsletterSagas")!;
                        EndpointConvention.Map<ReleaseInEvent>(new Uri($"exchange:{newsletterSagasEndpoint}"));
                    },
                    (busRegistrationContext, rabbitMqBusFactoryConfigurator) => ReceiveEndpoint(context.Configuration, busRegistrationContext, rabbitMqBusFactoryConfigurator));
            })
            .Build();

        Task shutdown = host.WaitForShutdownAsync();

        await host.RunAsync();
        await shutdown;
    }

    private static void ReceiveEndpoint(IConfiguration configuration, IBusRegistrationContext busRegistrationContext, IRabbitMqBusFactoryConfigurator rabbitMqBusFactoryConfigurator)
    {
        string newsletterSagasEndpoint = configuration.GetValue<string>("MassTransitEndpoints:NewsletterSagas")!;

        rabbitMqBusFactoryConfigurator.ReceiveEndpoint(
            newsletterSagasEndpoint,
            configure =>
            {
                configure.PrefetchCount = 5;
                configure.UseMessageRetry(r => r.Interval(5, TimeSpan.FromMilliseconds(1500)));
                configure.ConfigureSaga<NewsletterState>(busRegistrationContext);
            });
    }
}