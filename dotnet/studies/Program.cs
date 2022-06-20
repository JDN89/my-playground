// See https://aka.ms/new-console-template for more information
Console.WriteLine("Hello, World!");


Person jan = new Person
{
    Name = "Juan",
    Age = 25
};

string name = "sandeep";
string myName = name;
Console.WriteLine("== operator result is {0}", name == myName);
Console.WriteLine("Equals method result is {0}", name.Equals(myName));
Console.ReadKey();


Console.WriteLine(jan.Age);

Person dork = jan;
// jan is using the public method from the abstract class
// public method => need to instantiate class and call method on instantiated class
// belongs to an instance of a class <> static belongs to an class
jan.comment();

//static method => you can only call this on the Class without instantiating it

Person.staticComment();

dork.Age = 30;
Console.WriteLine(jan.Age); // output is 30

/* you can instantiate an abstract clas
Weirdo bert = new Weirdo
{

    Name = "Juan",
    Age = 25
}

*/
