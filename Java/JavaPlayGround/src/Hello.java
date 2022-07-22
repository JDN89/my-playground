public class Hello {
    public static void  main (String[] args) {
        /*
        System.out.println("Hello World");
        int [] myArr = {1,2,3,4,5,};
        Method.CallMe(myArr);

        for (int i = 0; i< 10; i++) {
            MultiThreadMe thread = new MultiThreadMe();
            thread.start();
        }

         */
        for (int i = 0; i<5; i++) {

            RunMeBaby baby = new RunMeBaby();
            Thread thread = new Thread(baby);
            thread.start();
        }
    }
}
