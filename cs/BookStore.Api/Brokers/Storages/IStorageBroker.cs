using System.Linq;
using System.Threading.Tasks;
using BookStore.Api.Models.Foundations.Books;

namespace BookStore.Api.Brokers.Storages;

public partial interface IStorageBroker
{
    ValueTask<Book> InsertBookAsync(Book book);
    IQueryable<Book> SelectAllBooks();
}