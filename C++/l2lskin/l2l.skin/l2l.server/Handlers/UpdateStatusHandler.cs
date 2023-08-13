using l2l.skin.Requests;
using MediatR;

namespace l2l.skin.Handlers
{
    public class UpdateStatusHandler : IRequestHandler<UpdateStatusRequest, UpdateStatusResponse>
    {
        public Task<UpdateStatusResponse> Handle(UpdateStatusRequest request, CancellationToken cancellationToken)
        {
            //SUMMONER-V4 lol/summoner/v4/summoners/by-name/{summonerName}

            //var infoPlayer = chupar a api de summoner | talvez nao seja necessario
            //var result = salvar no nosso banco o nome ou o ID
            //return o sucesso da operaçao

            throw new NotImplementedException();
        }
    }
}
