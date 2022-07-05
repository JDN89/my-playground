using System.Collections.Generic;
using System.Linq;
using System;

public static class UniqueNumber
{
    public static int GetUnique(IEnumerable<int> numbers)
    {
        int firstDisticnict = numbers.ToArray().GroupBy(i => i).Where(g => g.Count() == 1).Select(g => g.Key).FirstOrDefault();

        return firstDisticnict;
    }
}
