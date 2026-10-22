using RecipeApi.Models;

namespace RecipeApi.Services;

public interface IRecipeStore
{
    Task<List<Recipe>> GetAllAsync();
    Task<Recipe?> GetByIdAsync(int id);
    Task<Recipe> AddAsync(Recipe recipe);
    Task<bool> DeleteAsync(int id);
}