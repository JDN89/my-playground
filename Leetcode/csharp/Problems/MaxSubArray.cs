using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace csharp.Problems
{

    // initialize current sum val = nums[0]
    //initialize max sum val = nums[0]
    // loop over each element in the array
    // current sum = max of current sum + nums[i] en current sum?
    // max sum = max of curernt sum and max sum 
    // In C#, Max() is a Math class method which is used to returns the larger of the two specified numbers.
    public class MaxSubArray
    {
       public static int FindMax (int[] nums) {
           var current_sum = nums[0];
           var max_sum = nums[0];
            for(int i = 1; i < nums.Length; i++) {
                int num = nums[i];
            //compare curent sum value with value of current sum+1
            current_sum = Math.Max(nums[i], current_sum+nums[i]);
            //Console.WriteLine(num);
            Console.WriteLine(current_sum);
                       // Console.WriteLine(max_sum);

            max_sum = Math.Max(current_sum,max_sum);
            }

           return max_sum;

        }
    }
}