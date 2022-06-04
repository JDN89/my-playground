using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace csharp.Problems
{
   // You are climbing a staircase. It takes n steps to reach the top.

// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
   // als je eerste 4 a 5 trede uitwerkt op papier
   // zie je dat elke voorgaande manier 
   // gelijk is n-1 + n-2
   // 2 = 2 manieren
    // 3 = 3 manieren
    // 4 = 5 manieren 

    public class RecursionStaircase
    {

        Dictionary<int,int> memo = new Dictionary<int, int>();
        public  int FindNumWays (int n) {
            
            if (n == 0 || n ==1 ) {
                return memo[n] = 1;

            } 
            if (memo.ContainsKey(n)) {
                return memo[n];
            }

            else
            {
                return memo[n] = FindNumWays(n-1) + FindNumWays(n-2);
            } 


        }
        
    }
}