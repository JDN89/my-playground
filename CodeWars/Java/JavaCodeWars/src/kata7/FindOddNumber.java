package kata7;

import java.util.HashMap;

public class FindOddNumber {
    public static int findIt(int[] a) {
        var hashy = new HashMap<Integer,Integer>();
        int odd = 0;
        for (int i: a
             ) {
if(!hashy.containsKey(i)) {
    hashy.put(i,1);
} else {
   hashy.replace(i,hashy.get(i)+1);
}
        }
        for (var entry: hashy.entrySet()
             ) {
            if (entry.getValue()%2 != 0)
            { odd = entry.getKey();}
            System.out.println(entry.getKey() + " : " + entry.getValue());

        }
        System.out.println(odd);
        return odd;
    }
}
