using MediatR;
using Users.Model.Client;

namespace Users.API.Handlers
{
    public class GetClientHandler : IRequestHandler<GetClientRequest, ClientDetailsVM?>
    {
        private readonly IClientRepository _repository;

        public GetClientHandler(IClientRepository repository)
        {
            _repository = repository;
        }

        public async Task<ClientDetailsVM?> Handle(GetClientRequest request, CancellationToken cancellationToken)
        {
            var client = await _repository.GetById(request.Id);
            return client is not null ? new ClientDetailsVM(client.Name, client.BirthDate, client.Investments) : null;
        }
    }
}