using MediatR;

public class GetStatusRequest : IRequest<GetStatusResponse>
{
    public GetStatusRequest(string summonerName)
    {
        SummonerName = summonerName;
    }

    public string SummonerName { get; }
}