package repositories;

import java.util.List;

import javax.persistence.EntityManager;
import javax.persistence.EntityManagerFactory;
import javax.persistence.Persistence;
import javax.persistence.criteria.CriteriaBuilder;
import javax.persistence.criteria.CriteriaQuery;

import domain.Cliente;

public class ClienteRepository {
	private EntityManagerFactory entityManagerFactory;
	private EntityManager entityManager;

	public ClienteRepository() {
		this.entityManagerFactory = Persistence.createEntityManagerFactory("projeto_clientes");
		entityManager = entityManagerFactory.createEntityManager();

	}

	public void adicionar(Cliente cliente) {
		entityManager.getTransaction().begin();
		entityManager.persist(cliente);
		entityManager.getTransaction().commit();
	}
	
	public void atualizar(Cliente cliente) {
		entityManager.getTransaction().begin();
		Cliente clienteAntigo = entityManager.getReference(Cliente.class, cliente.getId());
		
		clienteAntigo.setNome(cliente.getNome());
		clienteAntigo.setEndereco(cliente.getEndereco());
		clienteAntigo.setCpf(cliente.getCpf());
		clienteAntigo.setId(cliente.getId());

		entityManager.merge(clienteAntigo);
		entityManager.getTransaction().commit();
	}
	
	public List<Cliente> getClientes() {
		CriteriaBuilder builder = entityManager.getCriteriaBuilder();
		CriteriaQuery<Cliente> criteria = builder.createQuery(Cliente.class);
		criteria.from(Cliente.class);

		return entityManager.createQuery(criteria).getResultList();
	}

	public Cliente getCliente(int id) {
		return entityManager.find(Cliente.class, id);
	}

	public void excluir(Cliente cliente) {
		entityManager.getTransaction().begin();

		entityManager.remove(cliente);

		entityManager.getTransaction().commit();
	}
}