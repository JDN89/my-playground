
class Person
{
    public string Name { get; init; }
    public Guid Id { get; init; }

    // zonder constructor kan je een null prop hebben in object
    // voor c#11 -> pas de verplichte props door als params aan een ctor
    public Person(string name)
    {
        Name = name;
    }
}
