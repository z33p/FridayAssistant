using MassTransit.EntityFrameworkCoreIntegration;
using Microsoft.EntityFrameworkCore;

namespace Libs.NewsletterStateMachine.Sagas;

public class NewsletterStateDbContext : SagaDbContext
{
    public NewsletterStateDbContext(DbContextOptions<NewsletterStateDbContext> options) : base(options)
    { }

    public DbSet<NewsletterState> NewsletterState { get; set; }

    protected override IEnumerable<ISagaClassMap> Configurations
    {
        get { yield return new NewsletterStateMap(); }
    }
}
