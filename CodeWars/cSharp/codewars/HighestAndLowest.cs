using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace cSharp.codewars
{
    public class HighestAndLowest
    {
        public static string HighAndLow(string numbers)
        {
            var parsed = numbers.Split(' ').Select(int.Parse);
            foreach (var i in parsed)
            {
                Console.WriteLine(i);
            }
            int high = parsed.Max();
            int low = parsed.Min();

            Console.WriteLine($"{high} {" "} {low}");
            return $"{high} {low}";
        }
    }
}