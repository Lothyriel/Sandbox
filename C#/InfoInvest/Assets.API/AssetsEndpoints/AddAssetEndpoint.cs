using Assets.Model.Requests;
using Assets.Model.ViewModels;
using Core.API;
using MediatR;

namespace Assets.API.AssetsEndpoints
{
    public class AddAssetEndpoint : IEndpointDefinition
    {
        public void DefineEndpoints(WebApplication app)
        {
            app.MapPost("/api/assets/addAsset", AddAsset);
        }

        private async Task<Guid> AddAsset(IMediator mediator, AddAssetVM asset)
        {
            return await mediator.Send(new AddAssetRequest(asset));
        }
    }
}