namespace classes;

public class Hondjes : VierVoeter
{
    public Hondjes(string name) : base(name)
    {
        Name = name;
    }

    private string Name { get; set; }

    public override void Wandel()
    {
        Console.WriteLine(" waggel waggel, is dit een honden wandel geluid? ");
    }
}
