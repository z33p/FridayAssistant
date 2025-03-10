using System.Reflection;
using Infrastructure.StateMachine.Sagas;
using Infrastructure.StateMachine.Sagas.Events;
using MassTransit;
using MassTransit.EntityFrameworkCoreIntegration;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.DependencyInjection;

namespace Infrastructure.StateMachine;

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
                .AddSagaStateMachine<NewsletterStateMachine, NewsletterState, NewsletterSagaDefinition>()
                .EntityFrameworkRepository(configure =>
                {
                    string connectionString = context.Configuration.GetValue<string>("ConnectionStrings:Postgres")!;
                    ConfigureEntityFrameworkRepository(configure, connectionString);
                });

            IConfigurationSection massTransitEndpoints = context.Configuration.GetSection("MassTransitEndpoints");

            busConfigurator.UsingRabbitMq((busContext, rabbitBusConfigurator) =>
            {
                string host = "localhost";
                string username = context.Configuration.GetValue<string>("RabbitMQ:User");
                string password = context.Configuration.GetValue<string>("RabbitMQ:Password");

                rabbitBusConfigurator.Host(host, "/", configure =>
                {
                    configure.Username(username);
                    configure.Password(password);
                });

                Uri messageBrokerEndpoint = new($"exchange:{massTransitEndpoints.GetValue<string>("MessageBroker")}");
                string newsletterSagasEndpoint = massTransitEndpoints.GetValue<string>("NewsletterSagasEndpoint")!;

                EndpointConvention.Map<ReleaseInEvent>(new Uri($"exchange:{newsletterSagasEndpoint}"));
                EndpointConvention.Map<FetchContentEvent>(messageBrokerEndpoint);
                EndpointConvention.Map<FetchOAuthTokenEvent>(messageBrokerEndpoint);
                EndpointConvention.Map<SendNewsletterEvent>(messageBrokerEndpoint);
                EndpointConvention.Map<ConcludedEvent>(messageBrokerEndpoint);

                ConfigureSagasReceiveEndpoint(busContext, rabbitBusConfigurator, newsletterSagasEndpoint);

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

    private static void ConfigureSagasReceiveEndpoint(IBusRegistrationContext busContext, IRabbitMqBusFactoryConfigurator rabbitBusConfigurator, string newsletterSagasEndpoint)
    {
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