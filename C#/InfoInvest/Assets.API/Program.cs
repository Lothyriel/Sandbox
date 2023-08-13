using Assets;
using Core.API;
using MediatR;
using Core.API.MongoDB;
using Assets.Infra;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();
builder.Services.AddEndPointDefinitions(typeof(Program));
builder.Services.AddMediatR(typeof(Program));
builder.Services.AddSingleton<IAssetRepository, AssetMongoRepository>();
builder.ConfigureMongoClient();

var app = builder.Build();

if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI();
}

app.UseEndpointDefinitions();
app.Run();