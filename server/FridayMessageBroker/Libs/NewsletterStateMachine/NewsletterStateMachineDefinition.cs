using Libs.NewsletterStateMachine.Sagas;
using Libs.NewsletterStateMachine.Sagas.Events;
using MassTransit;

namespace NewsletterStateMachine;

public class NewsletterSagaDefinition : SagaDefinition<NewsletterState>
{
    protected override void ConfigureSaga(IReceiveEndpointConfigurator endpointConfigurator, ISagaConfigurator<NewsletterState> sagaConfigurator)
    {
        IPartitioner partition = endpointConfigurator.CreatePartitioner(5);

        sagaConfigurator.Message<ReleaseInEvent>(configure => configure.UsePartitioner(partition, keyProvider => keyProvider.Message.CorrelationId));

        sagaConfigurator.Message<FetchContentEvent>(configure => configure.UsePartitioner(partition, keyProvider => keyProvider.Message.CorrelationId));
        sagaConfigurator.Message<ResultFetchContentEvent>(configure => configure.UsePartitioner(partition, keyProvider => keyProvider.Message.CorrelationId));

        sagaConfigurator.Message<FetchOAuthTokenEvent>(configure => configure.UsePartitioner(partition, keyProvider => keyProvider.Message.CorrelationId));
        sagaConfigurator.Message<ResultFetchOAuthTokenEvent>(configure => configure.UsePartitioner(partition, keyProvider => keyProvider.Message.CorrelationId));

        sagaConfigurator.Message<SendNewsletterEvent>(configure => configure.UsePartitioner(partition, keyProvider => keyProvider.Message.CorrelationId));
        sagaConfigurator.Message<ResultSendNewsletterEvent>(configure => configure.UsePartitioner(partition, keyProvider => keyProvider.Message.CorrelationId));

        sagaConfigurator.Message<ConcludedEvent>(configure => configure.UsePartitioner(partition, keyProvider => keyProvider.Message.CorrelationId));
        
        endpointConfigurator.UseMessageRetry(configure => configure.Intervals(1000, 2000, 5000));
        endpointConfigurator.UseInMemoryOutbox();
    }
}
