using System.Text.Json;
using NewsletterStateMachine.Contracts;

namespace Libs.NewsletterStateMachine.Sagas.Events;

public class FetchContentEvent : BaseEvent, IActionEvent
{
    public string FunctionName { get; } = "friday-newsletter";

    public string GetInvocationPayload()
    {
        LambdaRequest<object?> request = new()
        {
            Action = "GENERATE_LINKEDIN_NEWS_POST",
            Data = null,
            CorrelationId = CorrelationId.ToString()
        };

        return JsonSerializer.Serialize(request);
    }
}


public class ResultFetchContentEvent : BaseEvent { }
