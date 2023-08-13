using Assets.Model;
using Core.Infra.MongoDB;
using MongoDB.Bson.Serialization;
using MongoDB.Bson.Serialization.IdGenerators;
using MongoDB.Driver;

namespace Assets.Infra
{
    public class AssetMongoRepository : MongoRepository<Asset>, IAssetRepository
    {
        public AssetMongoRepository(IMongoClient client) : base(client)
        {
        }

        protected override string CollectionName { get; } = "AssetsCollection";
    }
}