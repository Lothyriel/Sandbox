using Microsoft.AspNetCore.Builder;
using Microsoft.Extensions.Configuration;
using MongoDB.ApplicationInsights.DependencyInjection;

namespace Core.API.MongoDB
{
    public static class MongoExtensions
    {
        public static void ConfigureMongoClient(this WebApplicationBuilder builder)
        {
            builder.Services.AddMongoClient
            (
                builder.Configuration.GetConnectionString("MongoConnection")
                    .Replace("MONGO_USER", Environment.GetEnvironmentVariable("MONGO_USER"))
                    .Replace("MONGO_PASSWORD", Environment.GetEnvironmentVariable("MONGO_PASSWORD"))
            );
        }
    }
}