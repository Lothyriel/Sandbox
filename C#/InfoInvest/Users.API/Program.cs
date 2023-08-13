using Core.API;
using Core.API.MongoDB;
using MediatR;
using Users.Infra.Clients;
using Users.Infra.Managers;
using Users.Model.Client;
using Users.Model.Managers;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();
builder.Services.AddEndPointDefinitions(typeof(Program));
builder.Services.AddMediatR(typeof(Program));
builder.Services.AddSingleton<IClientRepository, ClientMongoRepository>();
builder.Services.AddSingleton<IManagerRepository, ManagerMongoRepository>();
builder.ConfigureMongoClient();

var app = builder.Build();

if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI();
}

app.UseEndpointDefinitions();
app.Run();