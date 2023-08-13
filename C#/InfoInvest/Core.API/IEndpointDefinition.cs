using Microsoft.AspNetCore.Builder;

namespace Core.API
{
    public interface IEndpointDefinition
    {
        void DefineEndpoints(WebApplication app);
    }
}