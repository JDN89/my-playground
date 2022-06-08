// See https://aka.ms/new-console-template for more information

using classes;

Console.WriteLine("Hello, World!");
// ctor to avoid null value object props

// Person person = new Person { Name = "Jan" };
// initialize empty object;
// even with init you can initialize an empty object
// we kunnen dit vermijden door de props aan een constructor door te geven
// probleem: als je veel props hebt -> veel ctor argumenten -> oplossing C#11 is Required
Person person1 = new Person("Juan");

var arr = new int[] { 0, 1, 2 };
foreach (int el in arr)
{

    Console.WriteLine(el);
}
// practice polymorphism
VierVoeter rat = new VierVoeter("ratman");
rat.Wandel();

Hondjes hondje = new Hondjes("Victor");
hondje.Wandel();

var list = new List<int>();
list.Add(1);

foreach (int el in list)
{

    Console.WriteLine("list el:{0}", el);
}

// person1 is an object -> instantiation of the Person class
// init keyword -> initialize prop upon class instantiation



Console.WriteLine(person1.Name);

