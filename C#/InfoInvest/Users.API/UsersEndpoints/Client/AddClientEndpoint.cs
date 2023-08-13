using MediatR;
using Users.Model;
using Users.Model.Client;

namespace Users.API.UsersEndpoints.Client
{
    public class AddClientEndpoint
    {
        public void DefineEndpoints(WebApplication app)
        {
            app.MapPost("api/users/AddClient", AddManager);
        }

        private async Task<Guid> AddManager(IMediator mediator, UserVM user)
        {
            return await mediator.Send(new AddClientRequest(user));
        }
    }
}