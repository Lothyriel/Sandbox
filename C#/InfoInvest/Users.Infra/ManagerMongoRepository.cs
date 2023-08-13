using Core.Infra.MongoDB;
using MongoDB.Bson.Serialization;
using MongoDB.Bson.Serialization.IdGenerators;
using MongoDB.Driver;
using Users.Model.Managers;

namespace Users.Infra.Managers
{
    public class ManagerMongoRepository : MongoRepository<Manager>, IManagerRepository
    {
        public ManagerMongoRepository(IMongoClient client) : base(client)
        {
        }

        protected override string CollectionName { get; } = "ManagersCollection";
    }
}