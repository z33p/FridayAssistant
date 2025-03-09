using RestService.Intefaces.Responses;

namespace Libs.Shared.RestService.Interfaces.Services;

public interface ISecretManagerService
{
    Task<GetSecretResponse> GetSecretValue(string secretName);
}