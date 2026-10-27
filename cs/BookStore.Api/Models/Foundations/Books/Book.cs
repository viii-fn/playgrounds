using System;

namespace BookStore.Api.Models.Foundations.Books;

public class Book
{
    public Guid Id { get; set; }
    public string Title { get; set; }
    public string Author { get; set; }
    public DateTimeOffset PublishedDate { get; set; }
    public decimal Price { get; set; }
}