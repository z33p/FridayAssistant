using MassTransit.EntityFrameworkCoreIntegration;
using Microsoft.EntityFrameworkCore;

namespace Infrastructure.SagasStateMachine;

public class NewsletterStateDbContext : SagaDbContext
{
    public NewsletterStateDbContext(DbContextOptions<NewsletterStateDbContext> options) : base(options)
    { }

    public DbSet<NewsletterState> NewsletterState { get; set; }

    protected override IEnumerable<ISagaClassMap> Configurations
    {
        get { yield return new NewsletterStateMap(); }
    }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        base.OnModelCreating(modelBuilder);
        modelBuilder.Entity<NewsletterState>().ToTable("tb_newsletter_state");
    }
}
