using Amazon;
using Amazon.Lambda;
using Amazon.Lambda.Model;
using Libs.NewsletterStateMachine.Sagas.Events;
using MassTransit;

namespace FridayMessageBroker.Jobs;

public class MessageBrokerJob : IConsumer<IActionEvent>
{
    private readonly ILogger<MessageBrokerJob> _logger;
    private readonly AmazonLambdaClient _amazonLambdaClient;

    public MessageBrokerJob(ILogger<MessageBrokerJob> logger, IConfiguration configuration)
    {
        _logger = logger;
        RegionEndpoint region = ResolveAwsRegion(configuration);
        _amazonLambdaClient = new AmazonLambdaClient(region);
    }

    private static RegionEndpoint ResolveAwsRegion(IConfiguration configuration)
    {
        string awsRegion = configuration.GetValue<string>("AWSRegion")!;
        RegionEndpoint region = RegionEndpoint.GetBySystemName(awsRegion);
        return region;
    }

    public Task Consume(ConsumeContext<IActionEvent> context)
    {
        InvokeRequest request = new()
        {
            FunctionName = context.Message.FunctionName,
            Payload = context.Message.GetInvocationPayload()
        };

        FireAndForgetLambdaInvocation(request);
        return Task.CompletedTask;
    }

    public async void FireAndForgetLambdaInvocation(InvokeRequest request)
    {
        try
        {
            await _amazonLambdaClient.InvokeAsync(request);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao invocar a função lambda {functionName}", request.FunctionName);
        }
    }    
}
