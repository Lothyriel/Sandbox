using MediatR;
using Users.Model.Managers;

namespace Users.API.Handlers
{
    public class AddManagerHandler : IRequestHandler<AddManagerRequest, Guid>
    {
        private readonly IManagerRepository _repository;

        public AddManagerHandler(IManagerRepository repository)
        {
            _repository = repository;
        }
        public async Task<Guid> Handle(AddManagerRequest request, CancellationToken ct)
        {
            //poderia usar um Fluent Validations para as validacoes
            var manager = new Manager(request.User.Name, request.User.BirthDate);

            await _repository.Add(manager);

            return manager.Id;
        }
    }
}