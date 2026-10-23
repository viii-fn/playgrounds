using Microsoft.AspNetCore.Mvc;
using RecipeApi.Dtos;
using RecipeApi.Models;
using RecipeApi.Services;

namespace RecipeApi.Controllers;

[ApiController]
[Route("api/[controller]")]

public class RecipesController : ControllerBase
{
    private readonly IRecipeStore _store;

    public RecipesController(IRecipeStore store)
    {
        _store = store;
    }

    [HttpGet]
    public async Task<ActionResult<List<RecipeDto>>> GetAll()             
    {
        var recipes = await _store.GetAllAsync();

        // Transform every Recipe into a RecipeDto. Explained in section 7.
        var dtos = recipes.Select(r => ToDto(r)).ToList();

        return Ok(dtos);   // Ok(x) = status 200 with x serialized as JSON
    }            

    [HttpGet("{id:int}")]
    public async Task<ActionResult<RecipeDtos>> GetById(int id)
    {
        var recipe = await _store.GetByIdAsync(id);

        if (recipe is null)
        {
            return NotFound();
        }

        return Ok(ToDto(recipe));
    }

    [HttpPost]
    public async Task<ActionResult<RecipeDtos>> Create(CreateRecipeRequest request)
    {
        var recipe = new Recipe
        {
            Title = request.Title,
            Description = request.Description
        };

        var created = await _store.AddAsync(recipe);

        return CreatedAtAction(nameof(GetById), new { id = created.Id }, ToDto(created));
    }

    [HttpDelete("{id:int}")]
    public async Task<IActionResult> Delete(int id)
    {
        var deleted = await _store.DeleteAsync(id);

        return deleted ? NoContent() : NotFound();
    }
    
    private static RecipeDtos ToDto(Recipe r) =>
        new RecipeDtos(r.Id, r.Title, r.Description, r.CreatedAt);
}