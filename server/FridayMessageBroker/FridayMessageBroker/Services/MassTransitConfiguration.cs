using Libs.NewsletterStateMachine.Sagas.Events;
using MassTransit;

namespace FridayMessageBroker.Services;

public class MassTransitConfiguration
{
    private const int DEFAULT_CONCURRENT_MESSAGE_LIMIT = 20;
    private const int DEFAULT_PREFETCH_COUNT = 5;
    private const int DEFAULT_RETRY_TIMES = 7;
    private const int DEFAULT_RETRY_SECONDS = 2;

    public static void AddMassTransit(
        HostBuilderContext context,
        IServiceCollection services,
        Action<IConfigurationSection, IRabbitMqBusFactoryConfigurator, IBusRegistrationContext> registerReceiveEndpoints,
        Action<IBusRegistrationConfigurator> addJobConsumers
    )
    {
        services.AddMassTransit(busConfigurator =>
        {
            IConfigurationSection massTransitEndpoints = context.Configuration.GetSection("MassTransitEndpoints");

            busConfigurator.UsingRabbitMq((busContext, rabbitBusConfigurator) =>
            {
                IConfigurationSection rabbitConfigSection = context.Configuration.GetSection("RabbitMQ");

                rabbitBusConfigurator.Host(rabbitConfigSection.GetValue<string>("Host"), "/", configure =>
                {
                    configure.Username(rabbitConfigSection.GetValue<string>("User"));
                    configure.Username(rabbitConfigSection.GetValue<string>("Password"));
                });

                Uri newsletterSagasEndpoint = new($"exchange:{massTransitEndpoints.GetValue<string>("NewsletterSagasEndpoint")!}");

                EndpointConvention.Map<ReleaseInEvent>(newsletterSagasEndpoint);
                EndpointConvention.Map<FetchContentEvent>(newsletterSagasEndpoint);
                EndpointConvention.Map<FetchOAuthTokenEvent>(newsletterSagasEndpoint);
                EndpointConvention.Map<SendNewsletterEvent>(newsletterSagasEndpoint);
                EndpointConvention.Map<ConcludedEvent>(newsletterSagasEndpoint);

                registerReceiveEndpoints(massTransitEndpoints, rabbitBusConfigurator, busContext);

                rabbitBusConfigurator.ConfigureEndpoints(busContext);
            });

            addJobConsumers(busConfigurator);
        });
    }

    public static void RegisterConsumerEndpoint<TConsumer>(
        string massTransitEndpoint,
        IRabbitMqBusFactoryConfigurator rabbitMqBusFactoryConfigurator,
        IBusRegistrationContext busContext,
        int concurrentMessageLimit = DEFAULT_CONCURRENT_MESSAGE_LIMIT,
        int prefetchCount = DEFAULT_PREFETCH_COUNT
    ) where TConsumer : class, IConsumer
    {
        rabbitMqBusFactoryConfigurator.ReceiveEndpoint(
            massTransitEndpoint,
            configure =>
            {
                configure.ConcurrentMessageLimit = concurrentMessageLimit;
                configure.PrefetchCount = prefetchCount;
                configure.ConfigureConsumeTopology = false;
                configure.UseMessageRetry(r => r.Interval(DEFAULT_RETRY_TIMES, TimeSpan.FromSeconds(DEFAULT_RETRY_SECONDS)));
                configure.ConfigureConsumer<TConsumer>(busContext);
            }
        );
    }
}