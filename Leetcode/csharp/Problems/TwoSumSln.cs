/*
 Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.
 
 */

// brute force slow solution
// loop over array 
// Inner loop over same array
// if target => return new array new int[]= {i,j}

//efficient solution =>
//  loop once and store values inside a hashtable
// calculate difference target and nums[i]
// if difference is inside the hashtable
// return new int [hashtbale [diff], i]
// target - [i]
// else store nums[i]in the hashtable
// hashtable[nums[i]] = i


namespace leetcode_csharp.Problems;

public class TwoSumSln
{
    //public int [] nums = 2,7,11,15];
    //public int target = 9;

    static public int[] TwoSum(int[] nums, int target)
    {
        Dictionary<int, int> hashTable = new Dictionary<int, int>();
        for (int i = 0; i < nums.Length; i++)
        {
            var diff = target - nums[i];
            if (hashTable.ContainsKey(diff))
            {
                return new int[] {hashTable[diff], i};
            }
            else
            {
                // with just the add method we can have duplicate keys and an error
                // hashTable.Add(nums[i],i);     
                /*Alternatively, we can use the Item property. If the key does not exist in the collection, a new item is added. If the same key already exists in the collection, the item value is updated to the new value. 
            
                 The following code snippet adds an item and updates the existing item in the collection.
                 */
                hashTable[nums[i]] = i;
            }
        }

        throw new Exception("not found");
    }
}

