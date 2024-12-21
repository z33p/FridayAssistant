namespace Libs.Shared.RestService.Interfaces;

public interface ISecretManagerService
{
    Task<string> GetSecretValue(string secretName);
}