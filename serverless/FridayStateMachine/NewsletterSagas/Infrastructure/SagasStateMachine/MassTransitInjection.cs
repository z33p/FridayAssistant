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
    public static void AddMassTransit(IConfiguration configuration, IServiceCollection serviceCollection, Action configureSendEndpoint, Action<IBusRegistrationContext, IRabbitMqBusFactoryConfigurator> configureReceiveEndpoint)
    {
        serviceCollection.AddMassTransit(busConfigurator =>
        {
            busConfigurator
                .AddSagaStateMachine<NewsletterStateMachine, NewsletterState, NewsletterStateMachineDefinition>()
                .EntityFrameworkRepository(r =>
                {
                    r.ConcurrencyMode = ConcurrencyMode.Optimistic;
                    r.LockStatementProvider = new PostgresLockStatementProvider();

                    r.AddDbContext<DbContext, NewsletterStateDbContext>((provider, builder) =>
                    {
                        string connectionString = configuration.GetValue<string>("ConnectionStrings:PostgresCsharp")!;

                        builder
                            .EnableDetailedErrors()
                            .UseNpgsql(connectionString, m =>
                            {
                                m.MigrationsAssembly(Assembly.GetExecutingAssembly().GetName().Name);
                                m.MigrationsHistoryTable($"__{nameof(NewsletterStateDbContext)}");
                            });
                    });
                });

            busConfigurator.UsingRabbitMq((busRegistrationContext, rabbitMqBusFactoryConfigurator) =>
            {
                string host = "localhost";
                string username = configuration.GetValue<string>("RabbitMQ:User")!;
                string password = configuration.GetValue<string>("RabbitMQ:Password")!;

                rabbitMqBusFactoryConfigurator.Host(host, "/", hostConfigurator =>
                {
                    hostConfigurator.Username(username);
                    hostConfigurator.Password(password);
                });

                // Uri messageBrokerEndpoint = new($"exchange:{configuration.GetValue<string>("MassTransitEndpoints:MessageBroker")}");

                configureSendEndpoint();
                configureReceiveEndpoint(busRegistrationContext, rabbitMqBusFactoryConfigurator);

                rabbitMqBusFactoryConfigurator.ConfigureEndpoints(busRegistrationContext);

            });
        });
    }
}
