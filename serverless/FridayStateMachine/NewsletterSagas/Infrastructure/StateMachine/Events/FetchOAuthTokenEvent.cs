using System.Text.Json;
using Infrastructure.StateMachine.Contracts;

namespace Infrastructure.StateMachine.Sagas.Events;

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
