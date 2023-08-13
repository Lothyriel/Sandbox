using Core.Infra.MongoDB;
using MongoDB.Bson.Serialization;
using MongoDB.Bson.Serialization.IdGenerators;
using MongoDB.Driver;
using Users.Model.Client;

namespace Users.Infra.Clients
{
    public class ClientMongoRepository : MongoRepository<Client>, IClientRepository
    {
        public ClientMongoRepository(IMongoClient client) : base(client)
        {
        }

        protected override string CollectionName { get; } = "ClientsCollection";
    }
}