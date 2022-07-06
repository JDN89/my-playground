using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace cSharp.codewars;

public class TwoSum
{
    public static int[] TwoSums(int[] numbers, int target)
    {
        var table = new Dictionary<int, int>();
        //var result = new int[2];
        int[] result;
        for (var i = 0; i < numbers.Length; i++)
        {
            var diff = target - numbers[i];
            if (table.ContainsKey(diff))
            {
                result = new int[] { table[diff], i };
                Console.WriteLine(result[1]);
                return result;

            }
            else
            {

                table.Add(numbers[i], i);
            }
        }
        return new int[] { 2 };
    }
}
