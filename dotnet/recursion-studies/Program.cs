// See https://aka.ms/new-console-template for more information

namespace console_app
{
    static class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Hello World!");
            Console.WriteLine($"{args}");


            static int FindFactorial(int x)
            {
                if (x == 1)
                {
                    return 1;
                }
                else return x * FindFactorial(x - 1);
            }

            var result = FindFactorial(4);

            Console.WriteLine($"{result}");
            // other form of recursion by passing in a second param
            // we use 2 parameters to loop over the elements in the array
            // in this case we pass a default param - the index
            // example, double all the elements in an array

            var arr = new[] {1, 2, 3, 4, 5};

            static int[] DoubleArray(int[] arr, int i = 0)
            {
                if (i >= arr.Length)
                {
                    return arr;
                }

                arr[i] *= 2;
                Console.WriteLine(arr[i]);
                return DoubleArray(arr, i + 1);
            }


            var doubledArray = DoubleArray(arr);

// here we use a for loop to print all the values in the double array but
            for (int i = 0; i < doubledArray.Length; i++)
            {
                Console.WriteLine($"{doubledArray[i]}");
            }

            // another exampole => counting the number off x's in a string
            // loop over the string
            // string axxeaxrx
            // 1 identitfy base case > string.lenthg = 0 return o
            // next take next value a if x add 1 otherwise call functon again with remaing string legnth

            var xSession = "axxeaxrx";

            static int CountNumberOfXsInString(String s)
            {
                //base case 
                if (s.Length == 0)
                {
                    return 0;
                }

                if (s[0] == 'x')
                {
                    Console.WriteLine($"{s.Length}");
                    // get last element in arr or string => arr.lentgh-1 or string.length-1
                    // s => axxeaxrx
                    // base case is a
                    // parent case => s[1, s.lenght-1]
                    // you start from the complete string and you chop the first char off
                    // and call the same function with the substring = string minus first char
                    // a[xxeaxrx] -> x[xeaxrx] -> x[eaxrx] -> e[axrx] -> a[xrx] -> x[rx] -> r[x] -> x

                    return 1 + CountNumberOfXsInString(s.Substring(1, s.Length - 1));
                }

                return CountNumberOfXsInString(s.Substring(1, s.Length - 1));
            }

            var countedX = CountNumberOfXsInString(xSession);
            Console.WriteLine($"Number of x's counted is: {countedX}");
        }
    }
}