namespace leetcode_csharp.Problems;

/*      
place roman values into dictionary
reverse string
place string into array
loop over array
track curr and pre value
keep track or count
if curr value = I && pre value = V \\ X 
or if curr value  = X and pre value == L \\c
or if curr value = C and  pre value == D \\M
    return counter  - curr value
else return counter ++ curr value
*/
public class RomanInteger
{
    public static int RomanToInt(string s)
    {
        var counter = 0;

        char prev = 'A';
        char curr;
        
        List<char> reversedRomans = s.Reverse().ToList();
        var romanIntegers = new Dictionary<char, int>
        {
            ['I'] = 1, ['V'] = 5, ['X'] = 10, ['L'] = 50, ['C'] = 100, ['D'] = 500, ['M'] = 1000
        };
        for (int i = 0; i < reversedRomans.Count; i++)
        {
            curr = reversedRomans[i];
          
            if (curr == 'I' && (prev == 'V' || prev == 'X'))
            {
                counter -= romanIntegers[curr];
                prev = reversedRomans[i];
            }


           else if (curr == 'X' && (prev == 'L' || prev == 'C'))
            {
                counter -= romanIntegers[reversedRomans[i]];
               prev = reversedRomans[i];
            }

           else if (curr == 'C' && (prev == 'D' || prev == 'M'))
            {
                counter -= romanIntegers[reversedRomans[i]];
                 prev = reversedRomans[i];
            }
            else
            {
                counter += romanIntegers[reversedRomans[i]];
                prev = reversedRomans[i];
            }
        }

        return counter;
    }
}