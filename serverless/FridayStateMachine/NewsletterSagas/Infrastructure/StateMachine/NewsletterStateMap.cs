using MassTransit;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;

namespace Infrastructure.StateMachine;

public class NewsletterStateMap : SagaClassMap<NewsletterState>
{
    protected override void Configure(EntityTypeBuilder<NewsletterState> entity, ModelBuilder model)
    {
        entity.ToTable("tb_newsletter_state");

        entity.Property(x => x.CorrelationId)
            .HasColumnName("correlation_id")
            .IsRequired();

        entity.Property(state => state.RowVersion)
            .HasColumnName("xmin")
            .HasColumnType("xid")
            .IsRowVersion();

        entity.Property(x => x.CurrentState)
            .HasMaxLength(64)
            .HasColumnName("current_state");

        entity.Property(x => x.PreviousState)
            .HasMaxLength(64)
            .HasColumnName("previous_state");
    }
}