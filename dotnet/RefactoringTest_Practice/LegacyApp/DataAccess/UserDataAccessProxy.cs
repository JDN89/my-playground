using LegacyApp.Models;

namespace LegacyApp.DataAccess
{
    public class UserDataAccessProxy: IUserDataAccess
    {
        public void AddUser(User user)
        {
            // solve static problem through a proxy class -> make unit testable!
           UserDataAccess.AddUser(user); 
        }
    }
}