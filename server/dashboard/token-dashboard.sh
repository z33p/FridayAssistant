#!/bin/bash
# =============================================================================
# KUBERNETES DASHBOARD TOKEN GENERATOR
# =============================================================================
# Este script gera o token de acesso para o Dashboard
# Execução: DA MÁQUINA LOCAL (com kubectl configurado)
# Pré-requisitos: Dashboard já instalado
# =============================================================================

echo "🎫 Gerando token de acesso para o Dashboard..."

# Verificar se o service account existe
if ! kubectl get serviceaccount admin-user -n kubernetes-dashboard &> /dev/null; then
    echo "❌ Service account 'admin-user' não encontrado!"
    echo "Execute primeiro: ./dashboard.sh"
    exit 1
fi

# Gerar token temporário (válido por 1 hora)
echo "⏳ Gerando token temporário..."
TOKEN=$(kubectl create token admin-user -n kubernetes-dashboard --duration=3600s)

if [ -n "$TOKEN" ]; then
    echo "✅ Token gerado com sucesso!"
    echo ""
    echo "🔑 Seu token de acesso (válido por 1 hora):"
    echo "=================================================="
    echo "$TOKEN"
    echo "=================================================="
    echo ""
    echo "📋 Para usar:"
    echo "1. Acesse: https://k8s-dashboard.z33p.com"
    echo "2. Escolha 'Token' como método de login"
    echo "3. Cole o token acima"
    echo ""
    echo "💡 Dica: Copie o token agora, ele expira em 1 hora!"
else
    echo "❌ Erro ao gerar token!"
    exit 1
fi
