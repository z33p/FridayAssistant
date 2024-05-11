using System.Text.Json.Serialization;

namespace Infrastructure.StateMachine.Contracts;

public class LambdaResponse<TPayload>
{
    [JsonPropertyName("status_code")]
    public int StatusCode { get; set; }
    public TPayload? Data { get; set; }
    public IList<string>? Errors { get; set; }
    public string CorrelationId { get; set; } = string.Empty;
}
