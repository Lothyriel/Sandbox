using MediatR;
using Users.Model;
using Users.Model.Managers;

namespace Users.API.Handlers
{
    public class GetManagerHandler : IRequestHandler<GetManagerRequest, UserVM?>
    {
        private readonly IManagerRepository _repository;

        public GetManagerHandler(IManagerRepository repository)
        {
            _repository = repository;
        }

        public async Task<UserVM?> Handle(GetManagerRequest request, CancellationToken cancellationToken)
        {
            var manager = await _repository.GetById(request.Id);
            return manager is not null ? new UserVM(manager.Name, manager.BirthDate) : null;
        }
    }
}
