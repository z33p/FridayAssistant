using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Libs.Shared.RestService;
using Libs.Shared.RestService.Interfaces.Services;
using RestService.Intefaces.Responses;

namespace Libs.Shared;

public class HostBuilderConfiguration
{
    public static IHostBuilder CreateHostBuilder() =>
        Host
            .CreateDefaultBuilder()
            .ConfigureAppConfiguration(configurationBuilder =>
            {
                string? environment = Environment.GetEnvironmentVariable("DOTNET_ENVIRONMENT");
                if (environment is null || string.Compare(environment, "DEV", StringComparison.OrdinalIgnoreCase) == 0)
                {
                    // configurationBuilder.AddJsonFile("./Libs/Shared/appsettings.Development.json", false, true);

                    var appsettingsPath = Directory.GetCurrentDirectory() + "/Libs/Shared/appsettings.Development.json";
                    configurationBuilder.AddJsonFile(appsettingsPath, false, true);
                }
                else
                {
                    configurationBuilder.AddJsonFile("./appsettings.json", false, true);
                }
            })
            .ConfigureServices((hostContext, services) =>
            {
                services.Configure(delegate (HostOptions hostOptions)
                {
                    hostOptions.ShutdownTimeout = TimeSpan.FromSeconds(10);
                });

                services.AddSingleton<ISecretManagerService, SecretManagerService>();

                FetchAndSetConfiguration(hostContext.Configuration, services).Wait();

            });

    private static async Task FetchAndSetConfiguration(IConfiguration configuration, IServiceCollection services)
    {
        ServiceProvider serviceProvider = services.BuildServiceProvider();
        ISecretManagerService secretManagerService = serviceProvider.GetRequiredService<ISecretManagerService>();

        await LoadSecretToConfigurationAsync(configuration, secretManagerService, "ConnectionStrings:Postgres");
        await LoadSecretToConfigurationAsync(configuration, secretManagerService, "RabbitMQ:Password");
        await LoadSecretToConfigurationAsync(configuration, secretManagerService, "RabbitMQ:User");
        // await LoadSecretToConfigurationAsync(configuration, secretManagerService, "RabbitMQ:Host");
    }

    private static async Task LoadSecretToConfigurationAsync(IConfiguration configuration, ISecretManagerService secretManagerService, string secretName)
    {
        GetSecretResponse connectionStringsResponse = await secretManagerService.GetSecretValue(secretName);

        if (connectionStringsResponse is null)
            throw new Exception($"Não foi possível obter o valor do segredo {secretName}");
        else
            configuration[secretName] = connectionStringsResponse.Data;
    }
}