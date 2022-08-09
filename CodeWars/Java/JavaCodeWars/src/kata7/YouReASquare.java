package kata7;

public class YouReASquare {
    public static boolean isSquare(int n) {
        // calculate the square
        double sqr = Math.sqrt(n);
 // check if diff sqr  adn Math floor (sqr) == 0
        return ((sqr - Math.floor(sqr))== 0);
    }
}
