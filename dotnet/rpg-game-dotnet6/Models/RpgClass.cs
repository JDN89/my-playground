using System.Text.Json.Serialization;

namespace rpg_game_dotnet6.Models
{
    //convert enum to json
    [JsonConverter(typeof(JsonStringEnumConverter))]
    public enum RpgClass
    {
        Knight,
        Mage,
        Cleric
    }
}