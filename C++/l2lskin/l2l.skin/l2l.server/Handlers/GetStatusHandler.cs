using MediatR;

namespace l2l.skin.Handlers
{
    public class GetStatusHandler : IRequestHandler<GetStatusRequest, GetStatusResponse>
    {
        public GetStatusHandler()
        {
        }

        public Task<GetStatusResponse> Handle(GetStatusRequest request, CancellationToken cancellationToken)
        {
            //SUMMONER-V4 lol/summoner/v4/summoners/by-name/{summonerName}
            //SPECTATOR-V4 /lol/spectator/v4/active-games/by-summoner/{encryptedSummonerId}

            //var infoPlayer = chupar a api de summoner
            //var infoAmigo = infoPlayer.chupar a api de partida
            //var dadosTeammates = infoAmigo.chupar o que queremos
            //var dadosBanco = banco.chupar(dadosTeammates)
            //return dadosBanco

            throw new NotImplementedException();
        }
    }
}
