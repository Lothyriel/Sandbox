using l2l.skin.Requests;
using MediatR;

public class UpdateStatusRequest : IRequest<UpdateStatusResponse>
{
    public required string SummonerName { get; set; }
    public required string ChampionName { get; set; }
    public required string SkinId { get; set; }
}
