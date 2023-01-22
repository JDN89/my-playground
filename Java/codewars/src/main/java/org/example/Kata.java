package org.example;

public class Kata {
    public static int quarterOf(int month) {
     var result =    switch (month) {
            case 1, 3, 2: yield 1;
            case 4,5,6: yield 2;
            case 7,8,9: yield 3;
            case 10,11,12 : yield 4;

         default:
             throw new IllegalStateException("Unexpected value: " + month);
     };
        return result;
    }
}
