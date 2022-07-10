using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using api.Data;
using api.Services;
using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;

namespace api.Controllers;

[ApiController]
[Route("api/[controller]")]

public class PersonController : ControllerBase
{
    private readonly IConstrollerService _controllerService;

    public PersonController(IConstrollerService controllerService)
    {
        _controllerService = controllerService;
    }

    List<Person> persons = new List<Person>()
    { new Person{Id = 1, Name = "Jan", Age = 33 }

    };

    [AllowAnonymous]
    [HttpGet]
    public IActionResult FetchPerson()
    {
        _controllerService.Write(persons[0]);
        return Ok(persons);
    }

}
