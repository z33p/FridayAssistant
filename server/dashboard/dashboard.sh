#!/bin/bash
# =============================================================================
# KUBERNETES DASHBOARD INSTALLATION SCRIPT
# =============================================================================
# Este script instala o Kubernetes Dashboard completo com autenticação
# Execução: DA MÁQUINA LOCAL (com kubectl configurado)
# Pré-requisitos: Cluster K3s funcionando + Helm instalado
# =============================================================================

echo "🎛️ Instalando Kubernetes Dashboard..."

# Adicionar repositório oficial do Kubernetes Dashboard
echo "📦 Adicionando repositório Kubernetes Dashboard..."
helm repo add kubernetes-dashboard https://kubernetes.github.io/dashboard/

# Atualizar repositórios do Helm
echo "🔄 Atualizando repositórios Helm..."
helm repo update

# Instalar Dashboard usando Helm com configurações personalizadas
echo "⚙️ Instalando Dashboard via Helm..."
helm upgrade --install kubernetes-dashboard kubernetes-dashboard/kubernetes-dashboard --create-namespace --namespace kubernetes-dashboard -f dashboard-values.yml

# Aguardar deployment ficar pronto
echo "⏳ Aguardando Dashboard ficar pronto..."
kubectl wait --for=condition=available --timeout=300s deployment/kubernetes-dashboard-web -n kubernetes-dashboard

# Criar service account para acesso administrativo
echo "👤 Criando service account admin..."
kubectl apply -f service-account.yaml

# Aplicar cluster role binding para permissões admin
echo "🔐 Configurando permissões administrativas..."
kubectl apply -f cluster-role-binding.yml

# Criar secret para token de acesso
echo "🎫 Criando token de acesso..."
kubectl apply -f secret.yml

echo "✅ Dashboard instalado com sucesso!"
echo ""
echo "🌐 Acesso via subdomínio:"
echo "   https://k8s-dashboard.z33p.com"
echo ""
echo "🔗 Port-forward local:"
echo "   kubectl -n kubernetes-dashboard port-forward svc/kubernetes-dashboard-kong-proxy 8443:443"
echo ""
echo "🎫 Para obter token de acesso:"
echo "   ./token-dashboard.sh"

