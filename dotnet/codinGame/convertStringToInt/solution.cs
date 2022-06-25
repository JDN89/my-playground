class Solution
{
    static void Main(string[] args)
    {
        int c = int.Parse(Console.ReadLine());
        int n = int.Parse(Console.ReadLine());
        for (int i = 0; i < n; i++)
        {
            string row = Console.ReadLine();
            var list = new List<int>();
            foreach (char b in row)
            {
                list.Add(b - 'o');
            }
            var result = 0;
            foreach (int d in row)
            {
                if (d != 1)
                {
                    result += d;
                }
                else
                {
                    return;
                }

            }

            //return result;

        }


        // Write an answer using Console.WriteLine()
        // To debug: Console.Error.WriteLine("Debug messages...");

        Console.WriteLine("sum");
    }
}

