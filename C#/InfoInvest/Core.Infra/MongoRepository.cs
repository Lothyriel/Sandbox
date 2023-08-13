using Core.Model;
using MongoDB.Bson;
using MongoDB.Bson.Serialization;
using MongoDB.Bson.Serialization.Serializers;
using MongoDB.Driver;

namespace Core.Infra.MongoDB
{
    public abstract class MongoRepository<T> : IRepository<T> where T : Entity
    {
        private const string DatabaseName = "InfoInvestDB";
        public MongoRepository(IMongoClient client)
        {
            BsonSerializer.RegisterSerializer(new GuidSerializer(GuidRepresentation.Standard)); //because the default GUID serialization format is a legacy Microsoft one
            var database = client.GetDatabase(DatabaseName);
            Collection = database.GetCollection<T>(CollectionName);
        }
        protected IMongoCollection<T> Collection { get; }
        protected abstract string CollectionName { get; }
        public async Task Add(T entity)
        {
            await Collection.InsertOneAsync(entity);
        }
        public async Task<T?> GetById(Guid id)
        {
            return await Collection.Find(Builders<T>.Filter.Eq(x => x.Id, id)).FirstOrDefaultAsync();
        }
        public async Task<List<T>> GetRange(int start, int end)
        {
            return await Collection.Find(Builders<T>.Filter.Empty).Skip(start).Limit(end).ToListAsync();
        }
    }
}