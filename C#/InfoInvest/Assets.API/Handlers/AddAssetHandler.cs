using Assets.Model;
using Assets.Model.Requests;
using MediatR;

namespace Assets.API.Handlers
{
    public class AddAssetHandler : IRequestHandler<AddAssetRequest, Guid>
    {
        private readonly IAssetRepository _repository;

        public AddAssetHandler(IAssetRepository repository)
        {
            _repository = repository;
        }
        public async Task<Guid> Handle(AddAssetRequest request, CancellationToken ct)
        {
            //poderia usar um Fluent Validations para as validacoes
            var asset = new Asset(request.Asset.Name, request.Asset.Group); //poderia usar um Automapper para mapear de VM/entidade

            await _repository.Add(asset);

            return asset.Id;
        }
    }
}
