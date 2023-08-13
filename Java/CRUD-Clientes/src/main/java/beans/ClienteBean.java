package beans;

import java.util.List;

import javax.faces.bean.ManagedBean;
import javax.faces.bean.SessionScoped;

import domain.Cliente;
import services.ClienteServices;
import services.EnderecoServices;

@ManagedBean(name = "clienteBean")
@SessionScoped
public class ClienteBean {
	private ClienteViewModel clienteModel;
	private ClienteServices clienteServices;
	private String message;

	public ClienteBean() {
		this.clienteModel = new ClienteViewModel();
		this.clienteServices = new ClienteServices(new EnderecoServices());
	}

	public void adicionar() throws Exception{
		if (this.clienteModel.getCep() == "" || this.clienteModel.getCpf() == "" || this.clienteModel.getNome() == "") {
			this.message = "Preencha todos os campos";
			return;
		}

		this.message = this.clienteServices.adicionar(clienteModel);

		this.clienteModel = new ClienteViewModel();
	}

	public void editar(int id){
		Cliente cliente = this.clienteServices.getCliente(id);

		if (cliente == null) {
			this.message = "Cliente não existe";
			return;
		}

		this.clienteModel = new ClienteViewModel();
		this.clienteModel.setCep(cliente.getEndereco().getCep());
		this.clienteModel.setNome(cliente.getNome());
		this.clienteModel.setCpf(cliente.getCpf());
		this.clienteModel.setId(cliente.getId());
	}

	public void excluir(int id){
		this.message = this.clienteServices.excluir(id);
	}

	public ClienteViewModel getCliente() {
		return this.clienteModel;
	}

	public void setCliente(ClienteViewModel cliente) {
		this.clienteModel = cliente;
	}

	public List<Cliente> getClientes() {
		return this.clienteServices.getClientes();
	}

	public String getMessage() {
		return message;
	}

	public void setMessage(String error) {
		this.message = error;
	}
}