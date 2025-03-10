using MassTransit;

namespace Infrastructure.SagasStateMachine;

public class NewsletterStateMachineDefinition : SagaDefinition<NewsletterState>
{
    protected override void ConfigureSaga(IReceiveEndpointConfigurator endpointConfigurator, ISagaConfigurator<NewsletterState> sagaConfigurator)
    {
        IPartitioner partitioner = endpointConfigurator.CreatePartitioner(5);

        sagaConfigurator.Message<ReleaseInEvent>(x => x.UsePartitioner(partitioner, m => m.Message.CorrelationId));

        endpointConfigurator.UseMessageRetry(r => r.Intervals(1000, 2000, 5000));
        endpointConfigurator.UseInMemoryOutbox();
    }
}
