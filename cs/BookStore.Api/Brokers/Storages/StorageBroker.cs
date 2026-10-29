using System.Linq;
using System.Threading.Tasks;
using Microsoft.EntityFrameworkCore;

namespace BookStore.Api.Brokers.Storages;

internal partial class StorageBroker : DbContext, IStorageBroker
{
    public StorageBroker(DbContextOptions<StorageBroker> options)
        : base(options) { }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        base.OnModelCreating(modelBuilder);
        // additional configurations can go here
    }

    private async ValueTask<T> InsertAsync<T>(T @oject)
    {
        Entry(@object).State = EntityState.Added;
        await SaveChangesAsync();

        return @object;
    }

    private IQueryable<T> SelectAll<T>() where T : class
    {
        return Set<T>();
    }
}