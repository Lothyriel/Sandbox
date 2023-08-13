using Core.API;
using MediatR;
using Users.Model.Managers;

namespace Users.API.UsersEndpoints.Manager
{
    public class GetManagerEndpoint : IEndpointDefinition
    {
        public void DefineEndpoints(WebApplication app)
        {
            app.MapGet("api/users/getManager", GetManager);
        }

        private async Task<IResult> GetManager(IMediator mediator, Guid id)
        {
            var user = await mediator.Send(new GetManagerRequest(id));

            return user is not null ? Results.Ok(user) : Results.NoContent();
        }
    }
}