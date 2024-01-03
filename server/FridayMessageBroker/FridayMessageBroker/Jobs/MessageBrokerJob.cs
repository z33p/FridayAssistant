using Libs.NewsletterStateMachine.Sagas;
using MassTransit;

namespace FridayMessageBroker.Jobs;

public class MessageBrokerJob
    : IConsumer<FetchOAuthTokenEvent>,
    IConsumer<SendNewsletterEvent>,
    IConsumer<ConcludedEvent>
{
    public Task Consume(ConsumeContext<FetchOAuthTokenEvent> context)
    {
        throw new NotImplementedException();
    }

    public Task Consume(ConsumeContext<SendNewsletterEvent> context)
    {
        throw new NotImplementedException();
    }

    public Task Consume(ConsumeContext<ConcludedEvent> context)
    {
        throw new NotImplementedException();
    }
}
