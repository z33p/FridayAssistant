using Libs.Shared;
using FridayMessageBroker.Services;
using FridayMessageBroker.Jobs;
using MassTransit;

namespace FridayMessageBroker;

public class Program
{
    public static async Task Main()
    {
        IHost host = HostBuilderConfiguration
            .CreateHostBuilder()
            .ConfigureServices((context, services) =>
            {
                MassTransitConfiguration.AddMassTransit(
                    context,
                    services,
                    RegisterReceiveEndpoints,
                    AddJobConsumers
                );

                services.AddHostedService<Worker>();
            })
            .Build();

        var shutdown = host.WaitForShutdownAsync();

        await host.RunAsync();
        await shutdown;
    }

    private static void AddJobConsumers(IBusRegistrationConfigurator busRegistrationConfigurator)
    {
        busRegistrationConfigurator.AddConsumer<MessageBrokerJob>();
    }

    private static void RegisterReceiveEndpoints(
        IConfigurationSection massTransitEndpoints,
        IRabbitMqBusFactoryConfigurator rabbitMqBusFactoryConfigurator,
        IBusRegistrationContext busRegistrationContext
    )
    {
        MassTransitConfiguration.RegisterConsumerEndpoint<MessageBrokerJob>(
            massTransitEndpoints.GetValue<string>("ReleaseIn")!,
            rabbitMqBusFactoryConfigurator,
            busRegistrationContext
        );
    }
}