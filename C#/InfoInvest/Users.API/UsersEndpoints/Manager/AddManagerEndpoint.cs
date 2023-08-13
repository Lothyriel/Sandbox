using Core.API;
using MediatR;
using Users.Model;
using Users.Model.Managers;

namespace Users.API.UsersEndpoints.Manager
{
    public class AddManagerEndpoint : IEndpointDefinition
    {
        public void DefineEndpoints(WebApplication app)
        {
            app.MapPost("api/users/addManager", AddManager);
        }

        private async Task<Guid> AddManager(IMediator mediator, UserVM user)
        {
            return await mediator.Send(new AddManagerRequest(user));
        }
    }
}