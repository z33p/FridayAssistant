using Libs.NewsletterStateMachine.Sagas;
using MassTransit;

namespace FridayMessageBroker.Jobs;

public class MessageBrokerJob : IConsumer<ReleaseInEvent>
{
    public Task Consume(ConsumeContext<ReleaseInEvent> context)
    {
        throw new NotImplementedException();
    }
}
