using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace cSharp.codewars
{
    public class PigLatin
    {
        public static string PigIt(string str)
        {
            string sentence = "";
            var temp = str.Split(' ').ToList();

            foreach (var i in temp)
            {
                if (Char.IsLetter(i[0]))
                {

                    var first = i.First();

                    var cut = i.Remove(0, 1);

                    var final = cut + first + "ay";
                    sentence += $"{final} ";
                }
                else
                {
                    sentence += i;
                }

                Console.WriteLine(sentence);

            }
            return sentence.TrimEnd();
        }
    }
}