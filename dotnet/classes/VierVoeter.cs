using System.Linq.Expressions;

namespace classes;

public class VierVoeter
{
    private string Name { get; set; }

    public VierVoeter(string name)
    {
        Name = name;
    }

    public virtual void Wandel()
    {
        Console.WriteLine("stap stap");
    }
}