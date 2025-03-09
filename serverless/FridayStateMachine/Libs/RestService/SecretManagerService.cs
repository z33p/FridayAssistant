using Microsoft.Extensions.Configuration;
using Libs.Shared.RestService.Interfaces.Services;
using System.Net.Http.Json;
using RestService.Intefaces.Responses;

namespace Libs.Shared.RestService;

public class SecretManagerService : ISecretManagerService
{
    private readonly string BASE_URL;

    public SecretManagerService(IConfiguration configuration)
    {
        BASE_URL = configuration.GetValue<string>("Endpoints:SecretManager")!;
    }

    public async Task<GetSecretResponse> GetSecretValue(string secretName)
    {
        using HttpClient client = new();

        string url = $"{BASE_URL}/get_secret_value/{secretName}";
        HttpResponseMessage response = await client.GetAsync(url);
        response.EnsureSuccessStatusCode();

        GetSecretResponse? secretData = await response.Content.ReadFromJsonAsync<GetSecretResponse>();

        if (secretData is null)
            return new();
        else
            return secretData;
    }
}