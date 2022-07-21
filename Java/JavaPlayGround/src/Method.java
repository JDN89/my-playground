import java.util.Arrays;

public class Method {
    public static int CallMe(int [] arr) {
        var result = Arrays.stream(arr).max().getAsInt();
        System.out.println("Max val is " + result);
        for (var i: arr
             ) { System.out.println(i);

        }
        return result;
    }
}
