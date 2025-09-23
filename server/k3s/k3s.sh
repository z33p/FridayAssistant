#!/bin/bash
# =============================================================================
# K3S CLUSTER INSTALLATION SCRIPT
# =============================================================================
# Este script instala e configura um cluster K3s completo do zero
# Execução: APENAS NO SERVIDOR (AWS Lightsail Ubuntu)
# Pré-requisitos: Servidor Ubuntu com IP público
# =============================================================================

echo "🚀 Iniciando instalação do K3s cluster..."

# Criar arquivo de swap de 2GB para melhorar performance
echo "📝 Configurando swap de 2GB..."
sudo fallocate -l 2G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile

# Tornar o swap permanente após reboot
echo "💾 Persistindo configuração de swap..."
echo '/swapfile none swap sw 0 0' | sudo tee -a /etc/fstab

# Instalar K3s com versão específica e TLS SAN para o IP público
echo "⚙️ Instalando K3s v1.28.1+k3s1..."
curl -sfL https://get.k3s.io | INSTALL_K3S_VERSION=v1.28.1+k3s1 sh -s - server --tls-san 100.27.66.245

# Copiar configuração do kubeconfig para o usuário atual
echo "🔐 Configurando kubeconfig..."
sudo cp /etc/rancher/k3s/k3s.yaml kube_config
sudo chown $USER kube_config

echo "✅ K3s instalado com sucesso!"
echo "📋 Para usar: export KUBECONFIG=~/kube_config"
