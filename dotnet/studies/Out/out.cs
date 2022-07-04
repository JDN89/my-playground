
class GFG
{

    // Main method
    static public void Main()
    {

        // Declaring variable
        // without assigning value
        int i;

        // Pass variable i to the method
        // using out keyword
        Addition(out i);

        // Display the value i
        Console.WriteLine("The addition of the value is: {0}", i);
    }

    // Method in which out parameter is passed
    // and this method returns the value of
    // the passed parameter
    public static void Addition(out int i)
    {
        i = 30;
        i += i;
    }
}

