using MediatR;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();

var app = builder.Build();

if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI();
}

app.MapGet("status", (GetStatusRequest getStatus, IMediator mediator) => mediator.Send(getStatus));
app.MapPost("status", (UpdateStatusRequest update, IMediator mediator) => mediator.Send(update));

app.Run();