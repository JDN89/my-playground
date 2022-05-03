using Microsoft.AspNetCore.Mvc;

namespace Minimal.Api;

[ApiController]
public class ExampleController : ControllerBase
{
    [HttpGet("test")]
    public IActionResult Test()
    {
        return Ok();
    }
}
