package domain;

import java.io.Serializable;

import javax.persistence.CascadeType;
import javax.persistence.Entity;
import javax.persistence.GeneratedValue;
import javax.persistence.GenerationType;
import javax.persistence.Id;
import javax.persistence.OneToOne;

@Entity
public class Cliente extends Entidade implements Serializable {
	/**
	 * 
	 */
	private static final long serialVersionUID = 1L;

	public Cliente(Endereco endereco, String nome, String cpf) {
		super();
		this.endereco = endereco;
		this.nome = nome;
		this.cpf = cpf;
	}
	
	public Cliente(){}
	
	@OneToOne(cascade=CascadeType.PERSIST)
	private Endereco endereco;	
	private String nome;
	private String cpf;
	@Id
	@GeneratedValue(strategy=GenerationType.IDENTITY)
	private int id;

	public int getId() {
		return id;
	}

	public void setId(int id) {
		this.id = id;
	}

	@Override
	public String Validar() {
		String mensagem = "";

		if (getNome() == null) {
			mensagem += "Nome inválido";
		}
		if (getCpf() == null) {
			mensagem += "CPF inválido";
		}

		return mensagem;
	}

	public Endereco getEndereco() {
		return endereco;
	}

	public void setEndereco(Endereco endereco) {
		this.endereco = endereco;
	}

	public String getNome() {
		return nome;
	}

	public void setNome(String nome) {
		this.nome = nome;
	}

	public String getCpf() {
		return cpf;
	}

	public void setCpf(String cpf) {
		this.cpf = cpf;
	}
}