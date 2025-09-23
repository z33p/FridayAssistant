#!/bin/bash
# =============================================================================
# KUBERNETES DASHBOARD PORT-FORWARD (LOCAL ACCESS)
# =============================================================================
# Este script cria acesso local ao Dashboard via port-forward
# Execução: DA MÁQUINA LOCAL (com kubectl configurado)
# Uso: Quando quiser acessar via localhost em vez do subdomínio
# =============================================================================

echo "🔗 Iniciando port-forward para Dashboard..."

# Verificar se o Dashboard está rodando
if ! kubectl get deployment kubernetes-dashboard-web -n kubernetes-dashboard &> /dev/null; then
    echo "❌ Dashboard não encontrado!"
    echo "Execute primeiro: ./dashboard.sh"
    exit 1
fi

echo "🌐 Dashboard será acessível em: https://localhost:8443"
echo "🎫 Para obter token: ./token-dashboard.sh"
echo ""
echo "⚠️  Pressione Ctrl+C para parar o port-forward"
echo ""

# Iniciar port-forward para acesso local
kubectl -n kubernetes-dashboard port-forward svc/kubernetes-dashboard-kong-proxy 8443:443
