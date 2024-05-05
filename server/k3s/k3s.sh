# Create 2Gb swapfile
sudo fallocate -l 2G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile

# Persist swapfile
echo '/swapfile none swap sw 0 0' | sudo tee -a /etc/fstab

# Install k3s
curl -sfL https://get.k3s.io | INSTALL_K3S_VERSION=v1.28.1+k3s1 sh -s - server --tls-san 100.27.66.245
sudo cp /etc/rancher/k3s/k3s.yaml kube_config
sudo chown $USER kube_config
