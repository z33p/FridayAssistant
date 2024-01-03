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
                configurationBuilder.AddJsonFile("./Libs/Shared/appsettings.Development.json", false, true);
            })
            .ConfigureServices((hostContext, services) =>
            {
                services.Configure(delegate (HostOptions hostOptions)
                {
                    hostOptions.ShutdownTimeout = TimeSpan.FromSeconds(10);
                });
            });
}