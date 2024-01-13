using System.Text.Json;
using NewsletterStateMachine.Contracts;

namespace Libs.NewsletterStateMachine.Sagas.Events;

public class FetchOAuthTokenEvent : BaseEvent, IActionEvent
{
    public string FunctionName { get; } = "friday-google-oauth";

    public string GetInvocationPayload()
    {
        LambdaRequest<object?> request = new()
        {
            Action = "GENERATE_ACCESS_TOKEN",
            Data = null,
            CorrelationId = CorrelationId.ToString()
        };

        return JsonSerializer.Serialize(request);
    }
}

public class ResultFetchOAuthTokenEvent : BaseEvent { }
