public static class sortTheOdd
{
    public static int[] SortArray(int[] array)
    {
        Console.WriteLine("in the class");
        var even = new Dictionary<int, int>();
        var uneven = new List<int>();
        var indexUneven = new List<int>();
        var result = new int[array.Length];

        foreach (int e in array)
        {

            if (e % 2 != 0)
            {
                uneven.Add(e);
                indexUneven.Add(Array.IndexOf(array, e));
            }
            else even.Add(Array.IndexOf(array, e), e);

        }
        foreach (var item in even)
        {
            Console.WriteLine($"key: {item.Key} + {item.Value}");

            //Console.WriteLine($"value: {item.Value}");

        }

        //unevenSorted = uneven.OrderByDescending(e => e.Key).ToDictionary();
        foreach (var item in even)
        {
            result[item.Key] = item.Value;
            // Console.WriteLine($"key: {item.Key}");

            //Console.WriteLine($"value: {item.Value}");

        }
        uneven.Sort();
        indexUneven.Sort();

        for (int i = 0; i < uneven.Count; i++)
        {
            result[indexUneven[i]] = uneven[i];
            //Console.WriteLine(result[i]);
        }
        foreach (int it in result)
        {
            Console.WriteLine(it);
        }


        return result;

    }
}
