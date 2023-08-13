package services;

import java.util.InputMismatchException;
import java.util.List;

import beans.ClienteViewModel;
import domain.Cliente;
import domain.Endereco;
import repositories.ClienteRepository;

public class ClienteServices {
	private ClienteRepository clienteRepository;
	private EnderecoServices enderecoServices;

	public ClienteServices(EnderecoServices enderecoServices) {
		this.enderecoServices = enderecoServices;
		this.clienteRepository = new ClienteRepository();
	}
	
	public String adicionar(ClienteViewModel model) throws Exception {
		if (validarCPF(model.getCpf()) == false) {
			return "CPF inválido!";
		}

		Endereco endereco = enderecoServices.getEndereco(model.getCep());

		if (endereco == null) {
			return "CEP inválido";
		}

		Cliente cliente = new Cliente(endereco, model.getNome(), model.getCpf());
		cliente.setId(model.getId());

		String validacao = cliente.Validar();

		if (validacao != "") {
			return validacao;
		}

		return Salvar(cliente);
	}

	private String Salvar(Cliente cliente) {
		Cliente clienteExistente = this.clienteRepository.getCliente(cliente.getId());
		
		if (clienteExistente == null) {
			clienteRepository.adicionar(cliente);
			return "Cliente cadastrado!";
		}
		
		clienteRepository.atualizar(cliente);
		return "Cliente editado";
	}

	private boolean validarCPF(String cpf) {
		if (cpf.length() < 11) {
			return false;
		}

		char dig10, dig11;
		int sm, i, r, num, peso;

		// "try" - protege o codigo para eventuais erros de conversao de tipo (int)
		try {
			// Calculo do 1o. Digito Verificador
			sm = 0;
			peso = 10;
			for (i = 0; i < 9; i++) {
				// converte o i-esimo caractere do CPF em um numero:
				// por exemplo, transforma o caractere '0' no inteiro 0
				// (48 eh a posicao de '0' na tabela ASCII)
				num = (int) (cpf.charAt(i) - 48);
				sm = sm + (num * peso);
				peso--;
			}

			r = 11 - (sm % 11);
			if ((r == 10) || (r == 11))
				dig10 = '0';
			else
				dig10 = (char) (r + 48); // converte no respectivo caractere numerico

			// Calculo do 2o. Digito Verificador
			sm = 0;
			peso = 11;
			for (i = 0; i < 10; i++) {
				num = (int) (cpf.charAt(i) - 48);
				sm = sm + (num * peso);
				peso--;
			}

			r = 11 - (sm % 11);
			if ((r == 10) || (r == 11))
				dig11 = '0';
			else
				dig11 = (char) (r + 48);

			// Verifica se os digitos calculados conferem com os digitos informados.
			if ((dig10 == cpf.charAt(9)) && (dig11 == cpf.charAt(10)))
				return (true);

			return (false);
		} catch (InputMismatchException erro) {
			return (false);
		}
	}

	public List<Cliente> getClientes() {
		return clienteRepository.getClientes();
	}

	public Cliente getCliente(int id) {
		return clienteRepository.getCliente(id);
	}

	public String excluir(int id) {
		Cliente cliente = this.getCliente(id);

		if (cliente == null) {
			return "Cliente não existe";
		}

		this.clienteRepository.excluir(cliente);
		return "Cliente excluido";
	}
}