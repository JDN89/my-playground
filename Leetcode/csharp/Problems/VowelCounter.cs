using System.Reflection.Metadata.Ecma335;

namespace leetcode_csharp.Problems;

public class A 
{
    /// Tests that one of the arguments, or their sum, equals​​​​​​‌​​‌‌​‌​‌​‌‌‌​​​​​‌‌​​‌‌‌ 1.
    public class Algorithm
    {
        /// Finds the largest number in the <c>numbers</c>​​​​​​‌​​‌‌​‌​‌​‌‌‌​​​​​‌‌​​‌‌‌ array.
        public static int FindLargest(int[] numbers)
        {
            int largest = 0;
         for (int i=0; i < numbers.Length; i++)
         {
             if (numbers[i] > largest)
             {
                 largest = numbers[i];
             }
         }

         return largest;
        }
    }
}
    
