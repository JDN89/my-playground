// See https://aka.ms/new-console-template for more information

var numList = new List<int>() { 3, 5, 8, 33, 45, 1, 34 };

var result = numList.Select(x => x + 1);
var greater = numList.Select(x => x).Where(x => x > 10);
var ordered = numList.OrderBy(x => x);
//var deleted = numList.RemoveAll(x => x > 10);
//Console.WriteLine($"{deleted}");
/*foreach (int el in result)
{
    Console.WriteLine($"{el}");
}
*/

foreach (int el in ordered)
{
    Console.WriteLine($"{el}");
}

var persons = new List<Person>()
{ new Person{Id = 5, Name = "Jan", Age = 33},
{new Person{ Id=2, Name = "Helen",Age = 35}}
};

/*var ctorPersons = new List<CtorPerson>() {
    new CtorPerson(4,"jan",4)
}; */

//var findOnePerson = persons.Single(x => x.Id == 5);
var orderedById = persons.OrderBy(x => x.Id);
foreach (Person p in orderedById)
{
    Console.WriteLine(p.Name);
}



/*if (findOnePerson.Name is null)
{
    throw new NullReferenceException();
}
else
{

    Console.WriteLine($"{findOnePerson.Id},{findOnePerson.Name},{findOnePerson.Age}");
}

*/

public class Person
{
    public int Id { get; set; }
    public string Name { get; set; }
    public int Age { get; set; }


}

public class CtorPerson
{
    public CtorPerson(int id, string name, int age)
    {
        Id = id;
        Name = name;
        Age = age;
    }

    public int Id { get; set; }
    public string Name { get; set; }
    public int Age { get; set; }

}