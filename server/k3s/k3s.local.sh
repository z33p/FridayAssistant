#!/bin/bash
# =============================================================================
# K3S LOCAL CONFIGURATION SCRIPT
# =============================================================================
# Este script configura o acesso LOCAL ao cluster K3s remoto
# Execução: APENAS NA MÁQUINA LOCAL (seu computador)
# Pré-requisitos: Cluster K3s já instalado no servidor
# =============================================================================

echo "🔗 Configurando acesso local ao cluster K3s..."

# Baixar kubeconfig do servidor remoto via SCP
echo "📥 Baixando kubeconfig do servidor..."
scp -i ~/.ssh/MyPrivateLightsail.pem admin@100.27.66.245:kube_config ~/.kube/config

# ATENÇÃO: Após baixar, edite o arquivo ~/.kube/config
# Substitua: server: https://127.0.0.1:6443
# Por:       server: https://k8s.z33p.com:6443

echo "⚠️  IMPORTANTE: Edite ~/.kube/config e altere:"
echo "   server: https://127.0.0.1:6443"
echo "   para:   server: https://k8s.z33p.com:6443"
echo ""
echo "✅ Configuração local concluída!"
echo "📋 Teste com: kubectl get nodes"
