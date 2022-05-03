using Microsoft.AspNetCore.Authorization;

var builder = WebApplication.CreateBuilder(args);
//Configure our service
var app = builder.Build();

app.MapGet("/", () => "Hello World!");

app.Run();
