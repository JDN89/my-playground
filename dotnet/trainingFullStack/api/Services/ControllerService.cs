using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using api.Data;

namespace api.Services
{
    public class ControllerService : IConstrollerService
    {
        public void Write(Person person)
        {
            Console.WriteLine(person.Id);
        }
    }
}