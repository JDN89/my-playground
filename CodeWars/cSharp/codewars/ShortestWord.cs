using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace cSharp.codewars
{
    public class ShortestWord
    {
        public static int FindShort(string s)
        {
            var result = s.Split(' ').ToArray();
            var shortest = result[0].Length;
            foreach (var i in result)
            {
                if (i.Length < shortest)
                {
                    shortest = i.Length;
                }
            }
            return shortest;
        }
    }
}