using AutoMapper;
using rpg_game_dotnet6.Dtos.Character;
using rpg_game_dotnet6.Models;

namespace rpg_game_dotnet6;

public class AutoMapperProfile : Profile
{
    public AutoMapperProfile()
    {
        CreateMap<Character, GetCharacterDto>();
        CreateMap<AddCharacterDto, Character>();
    }
}