using MediatR;

namespace l2l.skin.Requests
{
    public class UpdateStatusResponse
    {
        public required string Message { get; set; }

        public required bool Success { get; set; }
    }
}
