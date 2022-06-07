using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using rpg_game_dotnet6.Dtos.Character;
using rpg_game_dotnet6.Models;

namespace rpg_game_dotnet6.Services.CharacterService
{
    public interface ICharacterService
    {

        Task<ServiceResponse<List<GetCharacterDto>>> GetAllCharacters();
        Task<ServiceResponse<GetCharacterDto>> GetCharacterById(int id);
        Task<ServiceResponse<List<GetCharacterDto>>> AddCharacter(AddCharacterDto newCharacter);
    }

}