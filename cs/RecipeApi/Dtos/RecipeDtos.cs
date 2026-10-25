using System.ComponentModel.DataAnnotations;

namespace RecipeApi.Dtos;

public record RecipeDto(int Id, string Title, string Description, DateTime CreatedAt);

public record CreateRecipeRequest
{
    [Required, StringLength(100, MinimumLength = 3)]
    public string Title { get; init; } = string.Empty;

    [StringLength(2000)]
    public string Description { get; init; } = string.Empty;
}
