using Core.API;
using MediatR;
using Users.Model.Client;

namespace Users.API.UsersEndpoints.Client
{
    public class GetClientEndpoint : IEndpointDefinition
    {
        public void DefineEndpoints(WebApplication app)
        {
            app.MapGet("api/users/getClient", GetClient);
        }

        private async Task<IResult> GetClient(IMediator mediator, Guid id)
        {
            var client = await mediator.Send(new GetClientRequest(id));

            return client is not null ? Results.Ok(client) : Results.NoContent();
        }
    }
}