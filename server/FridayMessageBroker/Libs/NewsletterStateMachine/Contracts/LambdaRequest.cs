namespace NewsletterStateMachine.Contracts;

public class LambdaRequest<TPayload>
{
    public string Action { get; set; } = string.Empty;
    public TPayload? Data { get; set; }
    public string CorrelationId { get; set; } = string.Empty;
}
