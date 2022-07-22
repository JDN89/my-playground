public class RunMeBaby implements Runnable{
    @Override
    public void run() {
        for (int i = 0; i < 6; i++) {
            System.out.println(i + " i from thread " + Thread.currentThread().getId()); }
            try {
Thread.sleep(1000);

        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

}
