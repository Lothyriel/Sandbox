# Start ollama docker container and pull selected model

docker run -d -v ollama:/root/.ollama -p 11434:11434 --name ollama ollama/ollama

docker exec -it ollama ollama pull {MODEL_NAME}
