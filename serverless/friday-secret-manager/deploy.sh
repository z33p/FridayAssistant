# Ask user for Docker tag version
echo "Enter Docker tag version (e.g. 2024-12-22.v1):"

# Check if dockerTag was passed as argument and if it wasn't ask the user for it
if [ -z "$1" ]; then
    read dockerTag
else
    dockerTag=$1
fi

while true; do
    # Check if dockerTag is in the correct format
    if [[ ! $dockerTag =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}.v[0-9]+$ ]]; then
        echo "Invalid Docker tag version. Please enter a valid version (e.g. 2024-12-22.v1):"
        read dockerTag
        continue
    fi

    # If checks pass, break the loop
    break
done

# Update the image tag in the deployment file
echo "Updating image tag in k8s/friday-secret-manager-deployment.yml to $dockerTag..."
sed -i "s|image: z33p/friday-secret-manager:.*|image: z33p/friday-secret-manager:$dockerTag|g" k8s/friday-secret-manager-deployment.yml

echo "Image tag updated successfully!"

# Create Docker image from Dockerfile
docker build --pull --rm -f "FridaySecretManager.Dockerfile" -t friday-secret-manager:$dockerTag "."

# Tag the image for Docker Hub
docker tag friday-secret-manager:$dockerTag docker.io/z33p/friday-secret-manager:$dockerTag

# Push Docker image to Docker Hub
docker image push docker.io/z33p/friday-secret-manager:$dockerTag 

# Apply service
kubectl apply -f k8s/friday-secret-manager-service.yml

# Apply deployment
kubectl apply -f k8s/friday-secret-manager-deployment.yml
