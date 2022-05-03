using Microsoft.AspNetCore.Authentication;

namespace Library.Api.Auth;

public class ApiKeyAuthSchemeOptions : AuthenticationSchemeOptions
{
    public string ApiKey { get; set; } = "VerySecret";
}
