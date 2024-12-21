using Microsoft.Extensions.Configuration;
using Libs.Shared.RestService.Interfaces;

namespace Libs.Shared.RestService;

public class SecretManagerService : ISecretManagerService
{
    private readonly string BASE_URL;

    public SecretManagerService(IConfiguration configuration)
    {
        BASE_URL = configuration.GetValue<string>("Endpoints:SecretManager")!;
    }

    public async Task<string> GetSecretValue(string secretName)
    {
        using HttpClient client = new();

        string url = $"{BASE_URL}/get_secret_value/{secretName}";
        HttpResponseMessage response = await client.GetAsync(url);
        response.EnsureSuccessStatusCode();
        string jsonResponse = await response.Content.ReadAsStringAsync();

        string secretData = System.Text.Json.JsonSerializer.Deserialize<string>(jsonResponse)!;
        return secretData;
    }
}