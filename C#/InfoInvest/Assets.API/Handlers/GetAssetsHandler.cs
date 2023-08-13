using Assets.Model;
using Assets.Model.Requests;
using MediatR;

namespace Assets.API.Handlers
{
    public class GetAssetsHandler : IRequestHandler<GetAssetsRequest, List<Asset>>
    {
        public GetAssetsHandler(IAssetRepository repository)
        {
            _repository = repository;
        }

        private readonly IAssetRepository _repository;

        public async Task<List<Asset>> Handle(GetAssetsRequest request, CancellationToken cancellationToken)
        {
            return await _repository.GetRange(request.Start, request.End);
        }
    }
}