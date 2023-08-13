package services;

import java.io.IOException;
import java.net.URI;
import java.net.URISyntaxException;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.net.http.HttpResponse.BodyHandlers;

import org.json.JSONObject;

import domain.Endereco;

public class EnderecoServices {
	private static final String CONSULSTA_CEP_URI = "https://viacep.com.br/ws/{CEP}/json/";
	private HttpClient client;

	public EnderecoServices() {
		client = HttpClient.newHttpClient();
	}

	public Endereco getEndereco(String cep) throws URISyntaxException, IOException, InterruptedException {
		String cepAddress = CONSULSTA_CEP_URI.replace("{CEP}", cep);

		HttpRequest request = HttpRequest.newBuilder().uri(new URI(cepAddress)).version(HttpClient.Version.HTTP_2).GET()
				.build();

		HttpResponse<String> response = client.send(request, BodyHandlers.ofString());

		if (response.statusCode() == 400) {
			return null;
		}

		JSONObject obj = new JSONObject(response.body());

		if (obj.has("erro") == true) {
			return null;
		}

		String logradouro = obj.getString("logradouro");
		String numero = "Informação não disponibilizada pela API CEP";
		String complemento = obj.getString("complemento");
		String bairro = obj.getString("bairro");
		String cidade = obj.getString("localidade");
		String uf = obj.getString("uf");

		return new Endereco(logradouro, numero, complemento, bairro, cidade, cep, uf);
	}
}