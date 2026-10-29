using System.Linq;
using System.Threading.Tasks;
using BookStore.Api.Models.Foundations.Books;
using Microsoft.EntityFrameworkCore;

namespace BookStore.Api.Brokers.Storages;

internal partial class StorageBroker
{
    public DbSet<Book> Books { get; set; }

    public IQueryable<Book> SelectAllBooks() =>
        SelectAll<Book>();

    public async ValueTask<Book> InsertBookAsync(Book book) =>
        await InsertAsync(book);
}