public class MultiThreadMe extends Thread{
    public void run() {
        System.out.println("Thread " + Thread.currentThread().getId()+ " is running");
    }
}
