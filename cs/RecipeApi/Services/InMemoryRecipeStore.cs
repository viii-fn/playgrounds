using RecipeApi.Models;

namespace RecipeApi.Services;

public class InMemoryRecipeStore : IRecipeStore
{
    private readonly List<Recipe> _items = new();
    private int _nextId = 1;
    public Task<List<Recipe>> GetAllAsync() =>
        Task.FromResult(_items.ToList());

    public Task<Recipe?> GetByIdAsync(int id) =>
        Task.FromResult(_items.FirstOrDefault(r => r.Id == id));

    public Task<Recipe> AddAsync(Recipe recipe)
    {
        recipe.Id = _nextId++;
        recipe.CreatedAt = DateTime.UtcNow;
        _items.Add(recipe);
        return Task.FromResult(recipe);
    }

    public Task<bool> DeleteAsync(int id)
    {
        var removed = _items.RemoveAll(r => r.Id == id);
        return Task.FromResult(removed > 0);
    }
}