namespace leetcode_csharp.Problems;

public class PalindromeNumber
{
    static public bool IsPalindrome(int x)
    {

        if (x < 0)
        {
            return false;
        }

        string original = x.ToString();
        string reverse = string.Join("", original.ToArray().Reverse());


        return reverse.Equals(original);

    }
}
