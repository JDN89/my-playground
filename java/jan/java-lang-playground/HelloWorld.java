import java.util.ArrayList;
import java.util.HashMap;

public class HelloWorld {
  


    static void findHighestInt(int[] a) {
  var hashy = new HashMap<Integer,Integer>();
    int odd = 0;

      for (int x : a) {
     if(!hashy.containsKey(x)) {

      hashy.put(x,1); 
      }
      
    else {
      hashy.replace(x,hashy.get(x)+1);
    }}
    
    for (int key : hashy.keySet()) {
      System.out.println(hashy.get(key));
      
    }

    }
public static void main(String[] args) {


//
findHighestInt(new int [] {1,2,3,2,4,5,6});
    

    // ARRAYLIST
//  var arr = new ArrayList<Integer>();
//     arr.add(2);
// arr.add(3);
//     arr.add(6);
//
//     for (int x : arr) {
//       System.out.println(x);
//       
//     }
}
}

