
public class Kata
{
    public static bool ValidatePin(string pin)
    {
        var alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz.,-";
        var pinTrimmed = pin.Trim().Replace(" ", "").Replace("\n", "");
        var result = pin switch
        {

            string s when s.Trim().Length < 4 => false,
            string s when s.Trim().Length == 5 || s.Length > 6 => false,

            string s when (s.Trim().Length == 4 || s.Trim().Length == 6) && s.Any(x => alpha.Contains(x)) => false,
            _ => true
        };

        return result;

        // Regex validate PIN code

    }

}

