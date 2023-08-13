using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Diagnostics;
using Microsoft.AspNetCore.Http;
using Microsoft.Extensions.DependencyInjection;
using Newtonsoft.Json;
using static System.Net.Mime.MediaTypeNames;

namespace Core.API
{
    public static class EndpointDefinitonExtensions
    {
        public static void AddEndPointDefinitions(this IServiceCollection services, params Type[] types)
        {
            var endpointDefinitions = new List<IEndpointDefinition>();

            foreach (var type in types)
            {
                endpointDefinitions.AddRange(GetEndpoints(type).Select(Activator.CreateInstance).Cast<IEndpointDefinition>());
            }

            services.AddSingleton(endpointDefinitions);
        }

        private static IEnumerable<Type> GetEndpoints(Type type)
        {
            return type.Assembly.ExportedTypes.Where(x => typeof(IEndpointDefinition).IsAssignableFrom(x) && !x.IsInterface);
        }

        public static void UseEndpointDefinitions(this WebApplication app)
        {
            app.UseExceptionHandler(exceptionHandlerApp =>
            {
                exceptionHandlerApp.Run(async context =>
                {
                    context.Response.StatusCode = StatusCodes.Status500InternalServerError;

                    context.Response.ContentType = Application.Json;

                    var exceptionHandlerPathFeature = context.Features.Get<IExceptionHandlerPathFeature>()!;

                    var json = JsonConvert.SerializeObject(exceptionHandlerPathFeature.Error, Formatting.Indented);

                    await context.Response.WriteAsync(json);
                });
            });

            var definitions = app.Services.GetRequiredService<List<IEndpointDefinition>>();

            foreach (var endpointDefinition in definitions)
            {
                endpointDefinition.DefineEndpoints(app);
            }
        }
    }
}