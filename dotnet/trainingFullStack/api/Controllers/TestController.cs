using api.Data;
using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;

namespace api.Controllers;

[ApiController]
[Route("api/[controller]")]
public class TestController : ControllerBase




{

    [AllowAnonymous]
    [HttpGet]
    public IActionResult Index()
    {
        return Ok("message fetched");
    }


    [AllowAnonymous]
    [HttpDelete]
    public IActionResult DeleteIndex()
    {
        return Ok("data deleted");
    }
    [AllowAnonymous]
    [HttpPost]
    public IActionResult PostIndex([FromBody] Message message)
    {


        Console.WriteLine(message.Title);
        Console.WriteLine($"{message.Title}");
        return NoContent();

    }

}