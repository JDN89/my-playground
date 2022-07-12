// See https://aka.ms/new-console-template for more information
Console.WriteLine("Hello, World!");

var list = new List<int>() {
1,2,3,6,8,4,2
};
var result = list.Select(x => x).Where(y => y > 3);
foreach (var e in result)
{
    Console.WriteLine(e);
}

var arr = new int[] { 1, 2, 3 };
foreach (int i in arr)
{
    Console.WriteLine(i);
}

Hound victor = new Hound();
victor.Howl();
victor.Waggle();
Canidae wolfie = new Canidae();
wolfie.Howl();
wolfie.Waggle();
interface canidae
{
    void Waggle();


}

public class Canidae : canidae
{
    public void Waggle()
    {
        Console.WriteLine("Waggle tail");

    }
    public virtual void Howl()
    {
        Console.WriteLine("howl at the moon");
    }
}

public class Hound : Canidae
{
    public override void Howl()
    {
        Console.WriteLine("Woof Woof");
    }
}
