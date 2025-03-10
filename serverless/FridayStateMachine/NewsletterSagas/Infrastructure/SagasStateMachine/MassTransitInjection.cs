using System.Reflection;
using Infrastructure.SagasStateMachine.Jobs;
using MassTransit;
using MassTransit.EntityFrameworkCoreIntegration;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;

namespace Infrastructure.SagasStateMachine;

public class MassTransitInjection
{
    private const int SAGAS_CONCURRENT_MESSAGE_LIMIT = 20;
    private const int SAGAS_PREFETCH_COUNT = 5;
    private const int SAGAS_RETRY_TIMES = 7;
    private const int SAGAS_RETRY_SECONDS = 2;

    public static void AddMassTransit(IConfiguration configuration, IServiceCollection serviceCollection, bool isSagas = true)
    {
        if (isSagas)
            serviceCollection.AddHostedService<BusControlJob>();

        serviceCollection.AddMassTransit(busConfigurator =>
        {
            busConfigurator
                .AddSagaStateMachine<NewsletterStateMachine, NewsletterState, NewsletterStateMachineDefinition>()
                .EntityFrameworkRepository(r =>
                {
                    r.ConcurrencyMode = ConcurrencyMode.Optimistic;
                    r.LockStatementProvider = new PostgresLockStatementProvider();

                    // r.UsePostgres();

                    r.AddDbContext<DbContext, NewsletterStateDbContext>((provider, builder) =>
                    {
                        string connectionString = configuration.GetValue<string>("ConnectionStrings:Postgres1")!;

                        builder
                            .EnableDetailedErrors()
                            .UseNpgsql(connectionString, m =>
                            {
                                m.MigrationsAssembly(Assembly.GetExecutingAssembly().GetName().Name);
                                m.MigrationsHistoryTable($"__{nameof(NewsletterStateDbContext)}");
                            });
                    });
                });

            busConfigurator.UsingRabbitMq((busContext, rabbitBusConfigurator) =>
            {
                string host = "localhost";
                string username = configuration.GetValue<string>("RabbitMQ:User")!;
                string password = configuration.GetValue<string>("RabbitMQ:Password")!;

                rabbitBusConfigurator.Host(host, "/", hostConfigurator =>
                {
                    hostConfigurator.Username(username);
                    hostConfigurator.Password(password);
                });

                // Uri messageBrokerEndpoint = new($"exchange:{configuration.GetValue<string>("MassTransitEndpoints:MessageBroker")}");
                string newsletterSagasEndpoint = configuration.GetValue<string>("MassTransitEndpoints:NewsletterSagas")!;

                EndpointConvention.Map<ReleaseInEvent>(new Uri($"exchange:{newsletterSagasEndpoint}"));

                if (isSagas)
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

                rabbitBusConfigurator.ConfigureEndpoints(busContext);

            });
        });
    }
}
