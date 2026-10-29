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
}