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
        Console.WriteLine("the fuck is het hondenwandelgeluid, kies de volgende keer een beter voorbeeld mong");
    }
}