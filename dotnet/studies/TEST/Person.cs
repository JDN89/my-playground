namespace studies.TEST;

class Person : Weirdo
{
    public string Name { get; set; }
    public int Age { get; set; }
    //default of a nullable prop is null?
    public int? Nulla { get; set; } = default;





}

abstract class Weirdo
{
    public void comment()
    {
        Console.WriteLine("run boy run");
    }
    public static void staticComment()
    {
        Console.WriteLine("I comment without being instantiated!");
    }
}

abstract class Pussy
{
    public void MorethenOne()
    {
        Console.WriteLine("Can I instantiate more then one class?"); // niet mogelijk
    }

}

