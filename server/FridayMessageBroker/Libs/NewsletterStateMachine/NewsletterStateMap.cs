using MassTransit;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;

namespace Libs.NewsletterStateMachine.Sagas;

public class NewsletterStateMap : SagaClassMap<NewsletterState>
{
    protected override void Configure(EntityTypeBuilder<NewsletterState> entity, ModelBuilder model)
    {
        entity.Property(state => state.RowVersion)
            .HasColumnName("xmin")
            .HasColumnType("xid")
            .IsRowVersion();

        entity.Property(state => state.PreviousState).HasMaxLength(64);
        entity.Property(state => state.CurrentState).HasMaxLength(64);
    }
}