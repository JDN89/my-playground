package kata8;

public class ReversedStrings {
    public static String solution(String str) {
        // Your code here...
        System.out.println(str);
     StringBuilder reversed = new StringBuilder();
     return reversed.append(str).reverse().toString();
    }
}
