//using studies.TEST;
//using studies.Struct;

var test = new NoOutExample();
var test1 = new OutExample();
int value = 5;
int secondVal = 6;
var result = test.ExampleOut(value);
test1.ExampleOut(out secondVal);
Console.WriteLine(value);
Console.WriteLine(result);
Console.WriteLine(secondVal);

public class NoOutExample
{
    public int ExampleOut(int value)
    {
        //    intValue = 10;
        //   Console.WriteLine(intValue);
        return value = 10;
    }
}


public class OutExample
{
    public void ExampleOut(out int intValue)
    {
        intValue = 16;
    }
}

/*
var mees = new Bird(2, true);
mees.flies();
var ditto = new Penguin(2, true);
ditto.flies();
ditto.Wings = 3;
Console.WriteLine($" ditto has {ditto.Wings} wings now ");
Penguin.Dive();
var bekky = ditto;
bekky.Wings = 2;
var strucky = new CheckMyStructOut(2);
var struktje = strucky;
struktje.MyMethod(3);
Console.WriteLine($"the value Num {strucky.Num} of strucky  hasn;t changed, because Struct is a value type stored in the stack -> ref points to the heap");
Console.WriteLine($"the value Num of struktje{struktje.Num}");

Console.WriteLine($" ditto has {ditto.Wings} wings now, after bekky = ditto");
Wadde.Access();

var jan = new Person()
{
    Name = "Jan",
    Age = 33
};
//ref are by default nullable
//Value types you have to mark as nullable
if (jan.Nulla is null)
{

    Console.WriteLine($"The value of Jan.nulla is null");
}

public class Bird
{
    public int Wings { get; set; }
    public bool Flies { get; set; }
    public Bird(int wings, bool flies)

    {
        this.Wings = wings;
        this.Flies = flies;
    }
    public virtual void flies()
    {
        Console.WriteLine($"I believe I can fly, because I have {Wings} wings");
    }
}

public class Penguin : Bird
{
    // how to inherit ctor
    public Penguin(int wings, bool flies) : base(wings, flies)
    { }

    public override void flies()
    {
        Console.WriteLine($"I believe I can't fly, eventough I have {Wings} wings");
    }
    public static void Dive()
    {
        Console.WriteLine($"I can dive and swim, I don't need to fly!");
    }
}
*/
