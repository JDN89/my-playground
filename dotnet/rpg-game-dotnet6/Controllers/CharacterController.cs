using Microsoft.AspNetCore.Mvc;
using rpg_game_dotnet6.Dtos.Character;
using rpg_game_dotnet6.Models;
using rpg_game_dotnet6.Services.CharacterService;

namespace rpg_game_dotnet6.Controllers
{
    [ApiController]
    // part without controller route name
    [Route("[controller]")]
    public class CharacterController : ControllerBase
    {

        private readonly ICharacterService _characterService;

        public CharacterController(ICharacterService characterService)
        {
            _characterService = characterService;
        }

        [HttpGet("GetAll")]
        public async Task<ActionResult<ServiceResponse<List<GetCharacterDto>>>> Get()
        {
            return Ok(await _characterService.GetAllCharacters());
        }

        [HttpGet("{id}")]
        public async Task<ActionResult<ServiceResponse<GetCharacterDto>>> GetSingle(int id)
        {
            // first throws exceptions if no character found
            // firstOrDefault returns null if no character found
            return Ok(await _characterService.GetCharacterById(id));
        }

        [HttpPost]
        public async Task<ActionResult<ServiceResponse<List<GetCharacterDto>>>> AddCharacter(AddCharacterDto newCharacter)
        {
            return Ok(await _characterService.AddCharacter(newCharacter));
        }
    }
}