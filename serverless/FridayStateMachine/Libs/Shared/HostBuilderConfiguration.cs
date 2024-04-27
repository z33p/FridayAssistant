using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;

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
                    configurationBuilder.AddJsonFile("./Libs/Shared/appsettings.Development.json", false, true);
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
            });
}