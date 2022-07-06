using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace cSharp.codewars;

public static class CategorizeNewMembers
{
    //alternatief


    public static IEnumerable<string> OpenOrSenior(int[][] data)
    {
        //return data.Select(member => member[0] >= 55 && member[1] > 7 ? "Senior" : "Open").ToList();
        var result = new List<string>();
        //your code here
        foreach (var arr in data)
        {
            Console.WriteLine($"{arr[0]}");
            if (arr[0] > 55 && arr[1] > 7)
            {
                result.Add("Senior");
            }
            else result.Add("Open");


        }
        foreach (var i in result)
        {
            Console.WriteLine(i);
        }
        return result;
    }

}
