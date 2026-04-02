#!/bin/bash
# =============================================================================
# K3S LOCAL CONFIGURATION SCRIPT
# =============================================================================
# Este script configura o acesso LOCAL ao cluster K3s remoto
# Execução: APENAS NA MÁQUINA LOCAL (seu computador)
# Pré-requisitos: Cluster K3s já instalado no servidor
# =============================================================================

echo "🔗 Configurando acesso local ao cluster K3s..."

# Buscar kubeconfig diretamente do /etc/rancher/k3s/k3s.yaml no servidor remoto
# (mais confiável que o kube_config salvo manualmente, sempre tem as credenciais atualizadas)
echo "📥 Baixando kubeconfig do servidor..."
ssh -i ~/.ssh/MyPrivateLightsail.pem admin@100.27.66.245 "sudo cat /etc/rancher/k3s/k3s.yaml" > ~/.kube/config
chmod 600 ~/.kube/config

# Substituir o endereço local pelo domínio público
echo "🔧 Atualizando endereço do servidor..."
sed -i 's|https://127.0.0.1:6443|https://k8s.z33p.com:6443|g' ~/.kube/config

echo "✅ Configuração local concluída!"
echo "📋 Teste com: kubectl get nodes"
