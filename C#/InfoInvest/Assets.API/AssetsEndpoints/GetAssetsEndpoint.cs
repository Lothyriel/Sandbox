using Assets.Model;
using Assets.Model.Requests;
using Core.API;
using MediatR;

namespace API.AssetsEndpoints
{
    public class GetAssetsEndpoint : IEndpointDefinition
    {
        public void DefineEndpoints(WebApplication app)
        {
            app.MapGet("/api/assets/getAssets", GetAssets);
        }

        private async Task<List<Asset>> GetAssets(IMediator mediator, int start, int end)
        {
            return await mediator.Send(new GetAssetsRequest(start, end));
        }
    }
}