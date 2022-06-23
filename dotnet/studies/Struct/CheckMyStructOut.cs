using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace studies.Struct
{
    public struct CheckMyStructOut
    {
        public int Num { get; set; }
        //struct can have a cotr!
        public CheckMyStructOut(int num)

        {
            this.Num = num;

        }
        public void MyMethod(int newNum)
        {
            Num += newNum;
            Console.WriteLine(Num);

        }


    }
}