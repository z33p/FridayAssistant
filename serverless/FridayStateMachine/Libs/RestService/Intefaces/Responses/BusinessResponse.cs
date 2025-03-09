namespace RestService.Intefaces.Responses;

public record BusinessResponse<T>
{
    public bool Success { get; set; }
    public T? Data { get; set; }
    public string[] Errors { get; set; } = [];
}
