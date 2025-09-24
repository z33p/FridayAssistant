#!/bin/bash

# Caminho do arquivo de deployment
DEPLOYMENT_FILE="k8s/friday-mcp-manager-deployment.yml"

# Extrai a tag atual do arquivo de deployment
currentTag=$(grep -oP 'image: z33p/friday-mcp-manager:\K[0-9]{4}-[0-9]{2}-[0-9]{2}.v[0-9]+' "$DEPLOYMENT_FILE")

# Obtém a data de hoje
today=$(date +%Y-%m-%d)

if [[ $currentTag =~ ^([0-9]{4}-[0-9]{2}-[0-9]{2})\.v([0-9]+)$ ]]; then
    currentDate="${BASH_REMATCH[1]}"
    currentVersion="${BASH_REMATCH[2]}"
    if [[ "$currentDate" == "$today" ]]; then
        # Se a data for igual à de hoje, incrementa a versão
        newVersion=$((currentVersion + 1))
    else
        # Se a data mudou, começa do 1
        newVersion=1
    fi
else
    # Se não encontrar tag, começa do zero
    newVersion=1
fi

dockerTag="${today}.v${newVersion}"

echo "Gerando nova tag: $dockerTag"

# Atualiza a imagem no arquivo de deployment
echo "Atualizando image tag em $DEPLOYMENT_FILE para $dockerTag..."
sed -i "s|image: z33p/friday-mcp-manager:.*|image: z33p/friday-mcp-manager:$dockerTag|g" "$DEPLOYMENT_FILE"

echo "Image tag atualizada com sucesso!"

# Build da imagem
docker build --pull --rm -f "FridayMcpManager.Dockerfile" -t friday-mcp-manager:$dockerTag "."

# Tag para Docker Hub
docker tag friday-mcp-manager:$dockerTag docker.io/z33p/friday-mcp-manager:$dockerTag

# Push para Docker Hub
docker image push docker.io/z33p/friday-mcp-manager:$dockerTag 

# Aplica service
kubectl apply -f k8s/friday-mcp-manager-service.yml

# Aplica deployment
kubectl apply -f k8s/friday-mcp-manager-deployment.yml