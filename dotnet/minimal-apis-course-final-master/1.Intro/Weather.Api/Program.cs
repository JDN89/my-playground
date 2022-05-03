using Weather.Api;

var builder = WebApplication.CreateBuilder(args);
var app = builder.Build();

app.MapGet("/weatherforecast", (ILogger<Program> logger) =>
{
    return Enumerable.Range(1, 5).Select(index => new WeatherForecast
        {
            Date = DateTime.Now.AddDays(index),
            TemperatureC = Random.Shared.Next(-20, 55),
            Summary = WeatherData.Summaries[Random.Shared.Next(WeatherData.Summaries.Length)]
        })
        .ToArray();
});

app.Run();
