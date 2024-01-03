using System.Reflection;
using Libs.NewsletterStateMachine.Sagas;
using MassTransit;
using MassTransit.EntityFrameworkCoreIntegration;
using Microsoft.EntityFrameworkCore;
using NewsletterStateMachine;

namespace NewsletterSagas;

public class NewsletterSagasConfiguration
{
    private const int SAGAS_CONCURRENT_MESSAGE_LIMIT = 20;
    private const int SAGAS_PREFETCH_COUNT = 5;
    private const int SAGAS_RETRY_TIMES = 7;
    private const int SAGAS_RETRY_SECONDS = 2;

    public static void AddMassTransit(HostBuilderContext context, IServiceCollection services)
    {
        services.AddMassTransit(busConfigurator =>
        {
            busConfigurator
                .AddSagaStateMachine<Libs.NewsletterStateMachine.Sagas.NewsletterStateMachine, NewsletterState, NewsletterSagaDefinition>()
                .EntityFrameworkRepository(configure => ConfigureEntityFrameworkRepository(
                    configure, context.Configuration.GetValue<string>("ConnectionStrings:Postgres")!
                ));

            IConfigurationSection massTransitEndpoints = context.Configuration.GetSection("MassTransitEndpoints");

            busConfigurator.UsingRabbitMq((busContext, rabbitBusConfigurator) =>
            {
                IConfigurationSection rabbitConfigSection = context.Configuration.GetSection("RabbitMQ");

                rabbitBusConfigurator.Host(rabbitConfigSection.GetValue<string>("Host"), "/", configure =>
                {
                    configure.Username(rabbitConfigSection.GetValue<string>("User"));
                    configure.Username(rabbitConfigSection.GetValue<string>("Password"));
                });

                Uri messageBrokerEndpoint = new($"exchange:{massTransitEndpoints.GetValue<string>("MessageBroker")}");

                EndpointConvention.Map<ReleaseInEvent>(messageBrokerEndpoint);
                EndpointConvention.Map<FetchOAuthTokenEvent>(messageBrokerEndpoint);
                EndpointConvention.Map<SendNewsletterEvent>(messageBrokerEndpoint);
                EndpointConvention.Map<ConcludedEvent>(messageBrokerEndpoint);

                ConfigureSagasReceiveEndpoint(busContext, rabbitBusConfigurator, massTransitEndpoints);

                rabbitBusConfigurator.ConfigureEndpoints(busContext);
            });
        });
    }

    private static void ConfigureEntityFrameworkRepository(
        IEntityFrameworkSagaRepositoryConfigurator<NewsletterState> configure,
        string connectionString)
    {
        configure.ConcurrencyMode = ConcurrencyMode.Optimistic;
        configure.LockStatementProvider = new PostgresLockStatementProvider();

        configure.UsePostgres();

        configure.AddDbContext<DbContext, NewsletterStateDbContext>((provider, builder) =>
        {
            builder
                .EnableDetailedErrors()
                .UseNpgsql(connectionString, m =>
                {
                    m.MigrationsAssembly(Assembly.GetExecutingAssembly().GetName().Name);
                    m.MigrationsHistoryTable($"__{nameof(NewsletterStateDbContext)}");
                });
        });
    }

    private static void ConfigureSagasReceiveEndpoint(IBusRegistrationContext busContext, IRabbitMqBusFactoryConfigurator rabbitBusConfigurator, IConfigurationSection massTransitEndpoints)
    {
        string newsletterSagasEndpoint = massTransitEndpoints.GetValue<string>("NewsletterSagasEndpoint")!;

        rabbitBusConfigurator.ReceiveEndpoint(
            newsletterSagasEndpoint,
            configure =>
            {
                configure.ConcurrentMessageLimit = SAGAS_CONCURRENT_MESSAGE_LIMIT;
                configure.PrefetchCount = SAGAS_PREFETCH_COUNT;
                configure.ConfigureConsumeTopology = false;
                configure.UseMessageRetry(r => r.Interval(SAGAS_RETRY_TIMES, TimeSpan.FromSeconds(SAGAS_RETRY_SECONDS)));
                configure.ConfigureSaga<NewsletterState>(busContext);
            }
        );
    }
}