namespace leetcode_csharp.Problems;

/*
 - create a stack: LIFO
 - if char matches ['(', '{', '[']
   - add char to stack
 - if char matches [')','}', ']'] 
   -check if it matches with last opening parantheses added to the stack
   - if matches => pop from stack
    - continue until all elements are popped and return true
   - if it doesn't match => false 
 */
public class ValidParantheses
{
    public static bool IsValid(string s)
    {
    
        Stack<char> myStack = new Stack<char>();
        List<char> myList = s.ToList();
        if (myList.Count == 1) return false;
        
        var parenthesesDick = new Dictionary<char, char>()
        {
            {')', '('},
            {']', '['},
            {'}', '{'},
        };

        bool matches = true;

        foreach (var c in myList)
        {
            if ((c == '{') || (c == '[') || (c == '('))
            {
                myStack.Push(c);
            }

            if ((c == '}') || (c == ']') || (c == ')'))
            {
                char peekable = myStack.Peek();
                if (peekable == parenthesesDick[c])
                {
                    myStack.Pop();
                    matches = true;
                }
                else return matches = false;
            }

            if (myStack.Count > 0)
            {
                matches = false;
            }
        }
        

      
        return matches;
    }
    
}